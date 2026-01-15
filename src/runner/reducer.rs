use crate::domain::message_event::MessageEvent;
use crate::domain::command::Command;
use tracing::info;

pub fn decide_action(event: &MessageEvent) -> Command {
    let text = event.text.trim().to_lowercase();
    match text.as_str() {
        "!ping" => {
            // Log especializado para comandos
            info!(
                target: "bot::command",
                command = "!ping",
                from = %event.sender,
                "command received"
            );

            Command::Reply("Pong! 🏓".to_string())
        }
        _ => Command::Ignore,
    }
}
