mod domain;
mod infra;
mod runner;

use std::sync::Arc;

use whatsapp_rust::bot::{Bot, MessageContext};
use whatsapp_rust::store::SqliteStore;
use whatsapp_rust_tokio_transport::TokioWebSocketTransportFactory;
use whatsapp_rust_ureq_http_client::UreqHttpClient;

use wacore::types::events::Event;
use wacore::proto_helpers::MessageExt;

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::new("error,bot::command=info")
        )
        .init();
    let backend = Arc::new(SqliteStore::new("whatsapp.db").await?);

    let transport_factory = TokioWebSocketTransportFactory::new();
    let http_client = UreqHttpClient::new();

    let mut bot = Bot::builder()
        .with_backend(backend)
        .with_transport_factory(transport_factory)
        .with_http_client(http_client)
        .on_event(|event, client| {
            async move {
                handle_event(event, client).await;
            }
        })
        .build()
        .await?;

    let runner = bot.run().await?;
    runner.await?;

    Ok(())
}

async fn handle_event(
    event: Event,
    client: Arc<whatsapp_rust::client::Client>,
) {
    match event {
        Event::PairingQrCode { code, .. } => {
            infra::whatsapp::print_qr(&code);
        }

        Event::Message(msg, info) => {
            // Criamos o CONTEXTO real da lib
            let ctx = MessageContext {
                message: msg,
                info,
                client,
            };

            // Extração idiomática de texto (via MessageExt)
            let Some(text) = ctx.message.text_content() else {
                return;
            };

            // Converte para evento de domínio
            let domain_event = domain::message_event::MessageEvent {
                sender: ctx.info.source.sender.to_string(),
                chat: ctx.info.source.chat.to_string(),
                is_group: ctx.info.source.is_group,
                text: text.to_string(),
            };

            // Decide ação (core da aplicação)
            let command = runner::reducer::decide_action(&domain_event);
            // Executa efeito colateral
            match command {
                domain::command::Command::Reply(reply_text) => {
                    infra::executor::send_text_reply(&ctx, reply_text).await;
                }

                domain::command::Command::Ignore => {}
            }
        }

        _ => {}
    }
}
