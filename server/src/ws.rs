use std::time::Duration;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde_derive::Deserialize;
use serde_json::json;
use tokio_util::sync::CancellationToken;

use crate::helpers::random_between_0_3;

pub fn router() -> Router {
    Router::new().route("/", get(ws_handler))
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        println!("client connected");
        let login_res = ws_login(socket).await;
        if login_res.is_none() {
            println!("client failed login");
            return;
        }
        let socket = login_res.unwrap();
        println!("client logged in");
        let (mut sender, mut receiver) = socket.split();
		let cancel = CancellationToken::new();
		let cancel_clone = cancel.clone();
		tokio::spawn(async move {
			loop {
				tokio::time::sleep(Duration::from_secs(2)).await;
				if cancel_clone.is_cancelled() {
                    println!("client didn't ping, disconnecting...");
					let _ = sender.send(Message::Text(json!({ "type": "error", "message": "client must send ping every 30 seconds. disconnecting client..." }).to_string())).await;
					return;
				}
				let msg = match random_between_0_3() {
					0 => {
						json!({ "type": "msg_zero", "zero_info": "this is messaage zero" }).to_string()
					}
					1 => {
						json!({ "type": "msg_one", "one_info": "this is messaage one" }).to_string()
					}
					2 => {
						json!({ "type": "msg_two", "two_info": "this is messaage two" }).to_string()
					}
                    3 => {
                        println!("randomly disconnecting client...");
                        cancel_clone.cancel();
                        return;
                    }
					_ => "this won't happen".to_string()
				};
				let _ = sender.send(Message::Text(msg)).await;
			}
		});

		// kick off client if no msg in 30 second
		loop {
            if cancel.is_cancelled() {
                break;
            }
			tokio::select! {
				_ = receiver.next() => {},
				_ = tokio::time::sleep(Duration::from_secs(30)) => {
					cancel.cancel();
					break;
				}
                _ = cancel.cancelled() => {
                    break;
                }
			}
		}
    })
}

#[derive(Deserialize, Debug)]
struct LoginMessage {
    username: String,
}

async fn ws_login(mut socket: WebSocket) -> Option<WebSocket> {
    let login_msg = socket.recv().await;

    if login_msg.is_none() {
        let _ = socket
            .send(Message::Text(
                json!({ "type": "error", "message": "failed to get login message" }).to_string(),
            ))
            .await;
        let _ = socket.close().await;
        return None;
    }

    let login_msg = login_msg.unwrap();

    if let Err(e) = login_msg {
        let _ = socket
            .send(Message::Text(json!({ "type": "error", "message": format!("failed to get login message | {e:?}") }).to_string()))
            .await;
        let _ = socket.close().await;
        return None;
    }

    match login_msg.unwrap() {
        Message::Text(login_msg) => {
            let login_info = serde_json::from_str::<LoginMessage>(&login_msg);
            if let Err(e) = login_info {
                let _ = socket
                    .send(Message::Text(json!({ "type": "error", "message": format!("failed to get parse login message | {e:?}") }).to_string()))
                    .await;
                let _ = socket.close().await;
                return None;
            }
            let login_info = login_info.unwrap();
            let _ = socket.send(Message::Text(json!({ "type": "login", "message": format!("username: {}", login_info.username) }).to_string())).await;
            Some(socket)
        }
        other => {
            let _ = socket
                .send(Message::Text(json!({ "type": "error", "message": format!("invalid login message | {other:?}") }).to_string()))
                .await;
            let _ = socket.close().await;
            None
        }
    }
}
