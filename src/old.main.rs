use std::sync::Arc;

use whatsapp_rust::bot::Bot;
use whatsapp_rust::store::SqliteStore;
use whatsapp_rust_tokio_transport::TokioWebSocketTransportFactory;
use whatsapp_rust_ureq_http_client::UreqHttpClient;

use wacore::types::events::Event;
use waproto::whatsapp;

use qrcode::QrCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = Arc::new(SqliteStore::new("whatsapp.db").await?);

    let mut bot = Bot::builder()
        .with_backend(backend)
        .with_transport_factory(TokioWebSocketTransportFactory::new())
        .with_http_client(UreqHttpClient::new())
        .on_event(handle_event)
        .build()
        .await?;

    let runner = bot.run().await?;
    runner.await?;

    Ok(())
}

async fn handle_event(
    event: Event,
    _client: Arc<whatsapp_rust::client::Client>,
) {
    match event {
        Event::PairingQrCode { code, .. } => {
            print_qr(&code);
        }

        Event::Message(msg, info) => {
            if let Some(text) = extract_text(&msg) {
                println!(
                    "[MSG] from {} → {}",
                    info.source.sender,
                    text
                );
            }
        }

        _ => {}
    }
}

fn extract_text(msg: &whatsapp::Message) -> Option<String> {
    if let Some(text) = &msg.conversation {
        return Some(text.clone());
    }

    if let Some(ext) = &msg.extended_text_message {
        if let Some(text) = &ext.text {
            return Some(text.clone());
        }
    }

    None
}

fn print_qr(payload: &str) {
    println!("[PAIRING] QR code:");

    match QrCode::new(payload.as_bytes()) {
        Ok(qr) => {
            let rendered = qr
                .render::<char>()
                .quiet_zone(false)
                .module_dimensions(2, 1)
                .build();

            println!("{}", rendered);
        }
        Err(_) => {
            println!("{}", payload);
        }
    }
}
