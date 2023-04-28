use web3::futures::StreamExt;
use web3::{transports::WebSocket, Web3};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let infura_ws_url = "wss://mainnet.infura.io/ws/v3/6376f591d7bd4ca5a4aef588675e6fa6";
    let transport = WebSocket::new(infura_ws_url).await?;
    let web3 = Web3::new(transport);

    // Get the latest block number
    let block_number = web3.eth().block_number().await?;
    println!("Latest block number: {:?}", block_number);

    // Subscribe to new blocks
    let mut sub = web3.eth_subscribe().subscribe_new_heads().await?;

    println!("Subscribed to new blocks");

    while let Some(head) = sub.next().await {
        match head {
            Ok(block) => {
                println!("New block: {:?}", block.number.unwrap());
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    Ok(())
}
