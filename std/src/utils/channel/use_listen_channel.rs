use std::future::Future;

use async_broadcast::RecvError;
use dioxus::prelude::*;

use super::UseChannel;

pub type UseListenChannelError = RecvError;

/// Create a messages listener for the given channel.
pub fn use_listen_channel<MessageType: Clone + 'static, Handler>(
    channel: &UseChannel<MessageType>,
    action: impl Fn(Result<MessageType, UseListenChannelError>) -> Handler + 'static,
) where
    Handler: Future<Output = ()> + 'static,
{
    use_memo_with_dependencies((channel,), move |(mut channel,)| {
        spawn(async move {
            let mut receiver = channel.receiver();
            loop {
                let message = receiver.recv().await;
                let message_err = message.clone().err();
                action(message).await;
                if message_err == Some(UseListenChannelError::Closed) {
                    break;
                }
            }
        })
    });
}
