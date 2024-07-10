use std::{fmt::Debug, marker::PhantomData, pin::Pin};

use alloy::{
    rpc::{
        client::RpcCall,
        json_rpc::{Request, RpcObject},
    },
    transports::{BoxFuture, Transport, TransportResult},
};
use futures::{future::join_all, Future, FutureExt};
use pin_project::pin_project;

use crate::MevHttp;

use super::Endpoints;

/// Allows to broadcast a request to many RPC endpoints.
#[pin_project]
pub struct BroadcastableCall<C, Params, Resp> {
    #[pin]
    fut: BoxFuture<'static, Vec<TransportResult<Resp>>>,
    phantom: PhantomData<(C, Params)>,
}

impl<C, Params, Resp> BroadcastableCall<C, Params, Resp>
where
    C: Clone + Send + Sync + 'static,
    Params: RpcObject,
    Resp: RpcObject,
    MevHttp<C>: Transport + Clone,
{
    /// Creates a new [`BroadcastableCall`].
    pub fn new(endpoints: &Endpoints<C>, request: Request<Params>) -> Self {
        let calls = endpoints
            .iter()
            .map(|mev_http| RpcCall::new(request.clone(), mev_http.clone()))
            .collect::<Vec<_>>();

        Self {
            fut: join_all(calls).boxed(),
            phantom: PhantomData::<(C, Params)>,
        }
    }
}

impl<C, Params, Resp> Future for BroadcastableCall<C, Params, Resp>
where
    C: Clone + Send + Sync + 'static,
    Params: RpcObject,
    Resp: RpcObject,
    MevHttp<C>: Transport + Clone,
{
    type Output = Vec<TransportResult<Resp>>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.project().fut.poll(cx)
    }
}

impl<C, Params, Resp> Debug for BroadcastableCall<C, Params, Resp> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BroadcastableCall").finish()
    }
}
