use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::{StreamExt, SinkExt};
use serde_json::{json, Value};
use url::Url;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_ws_url ;
    let ws_url = Url::parse(raw_ws_url)?;
    
    let (ws_stream, _) = connect_async(ws_url).await?;
    
    println!("WebSocket connected to Solana!");
    
    let (mut write, mut read) = ws_stream.split();
    
    let subscription_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            {
                "mentions": ["675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"]
            },
            {
                "commitment": "processed"
            }
        ]
    });
    
    write.send(Message::Text(subscription_request.to_string())).await?;
    
    println!("Subscribed to Raydium program logs. Listening for new pool creations...");
    
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => match msg {
                Message::Text(text) => {
                    if text.contains("initialize2") {
                        println!("New pool creation detected!");
                        if let Ok(json) = serde_json::from_str::<Value>(&text) {
                            if let Some(signature) = json["params"]["result"]["value"]["signature"].as_str() {
                                println!("Transaction signature: {}", signature);
                            } else {
                                println!("Couldn't extract signature from the message");
                            }
                        } else {
                            println!("Failed to parse JSON from the message");
                        }
                    }
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
    
    println!("WebSocket connection closed.");
    Ok(())
}