use std::task::{Context, Poll};

use crate::Flashbots;
use alloy_json_rpc::{RequestPacket, ResponsePacket};
use alloy_primitives::{keccak256, B256};
use alloy_signer::Signer;
use alloy_transport::{TransportError, TransportErrorKind, TransportFut};
use reqwest::header::HeaderValue;
use tower::Service;

impl<S: Signer + Clone + 'static> Flashbots<reqwest::Client, S> {
    fn request(&self, req: RequestPacket) -> TransportFut<'static> {
        let this = self.clone();

        Box::pin(async move {
            let bytes =
                serde_json::to_vec(&req).map_err(|err| TransportError::deser_err(err, ""))?;

            let signature = this
                .signer
                .sign_message(format!("0x{:x}", B256::from(keccak256(&bytes))).as_bytes())
                .await
                .map_err(TransportErrorKind::custom)?;

            let header_val = HeaderValue::from_str(&format!(
                "{:?}:0x{}",
                this.signer.address(),
                signature.inner()
            ))
            .expect("Header contains invalid characters");

            let resp = this
                .http
                .client()
                .post(this.http.url())
                .header("x-flashbots-signature", header_val)
                .body(bytes)
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

impl<S> Service<RequestPacket> for Flashbots<reqwest::Client, S>
where
    S: Signer + Clone + 'static,
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
                m if m.starts_with("mev_") => self.request(single.into()),
                _ => self.http.call(single.into()),
            },
            other => self.http.call(other),
        }
    }
}
