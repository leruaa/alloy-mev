use std::task::{Context, Poll};

use alloy::rpc::json_rpc::{RequestPacket, ResponsePacket};
use alloy::{
    primitives::{hex, keccak256},
    transports::{TransportError, TransportErrorKind, TransportFut},
};
use tower::Service;

use crate::MevHttp;

use super::BundleSigner;

impl MevHttp<reqwest::Client> {
    fn send_authenticated_request(
        &self,
        req: RequestPacket,
        bundle_signer: BundleSigner,
    ) -> TransportFut<'static> {
        let this = self.clone();

        Box::pin(async move {
            let body = serde_json::to_vec(&req).map_err(TransportError::ser_err)?;

            let signature = bundle_signer
                .signer
                .sign_message(format!("{:?}", keccak256(&body)).as_bytes())
                .await
                .map_err(TransportErrorKind::custom)?;

            this.http
                .client()
                .post(this.url)
                .header(
                    &bundle_signer.header,
                    format!(
                        "{:?}:0x{}",
                        bundle_signer.address(),
                        hex::encode(signature.as_bytes())
                    ),
                )
                .body(body)
                .send()
                .await
                .map_err(TransportErrorKind::custom)?
                .json()
                .await
                .map_err(TransportErrorKind::custom)
        })
    }
}

impl Service<RequestPacket> for MevHttp<reqwest::Client> {
    type Response = ResponsePacket;
    type Error = TransportError;
    type Future = TransportFut<'static>;

    #[inline]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&mut self, req: RequestPacket) -> Self::Future {
        match req {
            RequestPacket::Single(single) => {
                if let Some(bundle_signer) = self.bundle_signer.clone() {
                    match single.method() {
                        m if m.starts_with("mev_") => {
                            self.send_authenticated_request(single.into(), bundle_signer)
                        }
                        _ => self.http.call(single.into()),
                    }
                } else {
                    self.http.call(single.into())
                }
            }
            other => self.http.call(other),
        }
    }
}
