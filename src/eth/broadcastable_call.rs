use std::{fmt::Debug, future::IntoFuture, marker::PhantomData, pin::Pin};

use alloy::{
    providers::ext::MevBuilder,
    rpc::{
        client::RpcCall,
        json_rpc::{Request, RpcObject},
    },
    transports::{BoxFuture, TransportResult},
};
use futures::{future::join_all, Future, FutureExt};
use pin_project::pin_project;

use crate::utils::build_rpc_client;

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
            .map(|e| {
                let client = build_rpc_client(e.url.clone());
                let rpc_call = RpcCall::new(request.clone(), client.transport().clone());
                let mut mev = MevBuilder::new_rpc(rpc_call);
                if let Some(signer) = &e.signer {
                    mev = mev.with_auth(signer.clone())
                }
                mev.into_future()
            })
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
