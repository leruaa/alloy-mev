use std::task::{Context, Poll};

use alloy::rpc::json_rpc::{RequestPacket, ResponsePacket};
use alloy::signers::Signer;
use alloy::{
    primitives::{hex, keccak256},
    transports::{TransportError, TransportErrorKind, TransportFut},
};
use tower::Service;

use crate::MevHttp;

impl<S: Signer + Clone + Send + Sync + 'static> MevHttp<reqwest::Client, S> {
    fn request_to_mev_share(&self, req: RequestPacket) -> TransportFut<'static> {
        let this = self.clone();

        Box::pin(async move {
            let body = serde_json::to_vec(&req).map_err(TransportError::ser_err)?;

            let signer = this
                .signer
                .ok_or(TransportErrorKind::custom_str("Missing bundle signer"))?;

            let signature = signer
                .sign_message(format!("{:?}", keccak256(&body)).as_bytes())
                .await
                .map_err(TransportErrorKind::custom)?;

            let resp = this
                .http
                .client()
                .post(this.mev_share_url)
                .header(
                    "X-Flashbots-Signature",
                    format!(
                        "{:?}:0x{}",
                        signer.address(),
                        hex::encode(signature.as_bytes())
                    ),
                )
                .body(body)
                .send()
                .await
                .map_err(TransportErrorKind::custom)?;

            let json = resp.text().await.map_err(TransportErrorKind::custom)?;

            let resp =
                serde_json::from_str(&json).map_err(|err| TransportError::deser_err(err, ""))?;

            Ok(resp)
        })
    }
}

impl<S> Service<RequestPacket> for MevHttp<reqwest::Client, S>
where
    S: Signer + Clone + Send + Sync + 'static,
{
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
            RequestPacket::Single(single) => match single.method() {
                m if m.starts_with("mev_") => self.request_to_mev_share(single.into()),
                _ => self.http.call(single.into()),
            },
            other => self.http.call(other),
        }
    }
}
