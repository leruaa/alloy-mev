use std::{fmt::Debug, marker::PhantomData, pin::Pin};

use alloy::{
    rpc::{
        client::RpcCall,
        json_rpc::{Request, RpcObject},
    },
    transports::{BoxFuture, TransportResult},
};
use futures::{future::join_all, Future, FutureExt};
use pin_project::pin_project;

use super::Endpoints;

/// Allows to broadcast a request to many RPC endpoints.
#[pin_project]
pub struct BroadcastableCall<Params, Resp> {
    #[pin]
    fut: BoxFuture<'static, Vec<TransportResult<Resp>>>,
    phantom: PhantomData<Params>,
}

impl<Params, Resp> BroadcastableCall<Params, Resp>
where
    Params: RpcObject,
    Resp: RpcObject,
{
    /// Creates a new [`BroadcastableCall`].
    pub fn new(endpoints: &Endpoints, request: Request<Params>) -> Self {
        let calls = endpoints
            .iter()
            .map(|mev_http| RpcCall::new(request.clone(), mev_http.clone()))
            .collect::<Vec<_>>();

        Self {
            fut: join_all(calls).boxed(),
            phantom: PhantomData::<Params>,
        }
    }
}

impl<Params, Resp> Future for BroadcastableCall<Params, Resp> {
    type Output = Vec<TransportResult<Resp>>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.project().fut.poll(cx)
    }
}

impl<Params, Resp> Debug for BroadcastableCall<Params, Resp> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BroadcastableCall").finish()
    }
}
