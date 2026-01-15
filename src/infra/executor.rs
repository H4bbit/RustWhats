use whatsapp_rust::bot::MessageContext;
use waproto::whatsapp as wa;

/// Envia uma resposta simples de texto
pub async fn send_text_reply(
    ctx: &MessageContext,
    text: impl Into<String>,
) {
    let message = wa::Message {
        conversation: Some(text.into()),
        ..Default::default()
    };

    if let Err(e) = ctx.send_message(message).await {
        eprintln!("Erro ao enviar mensagem: {:?}", e);
    }
}
