use web3::api::SubscriptionStream;
use web3::transports::{Http, WebSocket};
use web3::types::{Block, BlockId, BlockNumber, Transaction};
use web3::Web3;

use crate::errors::EssenError;

pub struct InfuraApi {
    project_id: String,
}

impl InfuraApi {
    pub fn new(project_id: &str) -> Self {
        Self {
            project_id: project_id.to_string(),
        }
    }

    pub async fn fetch_block(
        self,
        block_number: u64,
    ) -> Result<Option<Block<Transaction>>, EssenError> {
        let infura_http_url = format!("https://mainnet.infura.io/v3/{}", &self.project_id);
        let transport =
            Http::new(&infura_http_url).map_err(|e| EssenError::Web3(e.to_string(), e))?;
        let web3 = Web3::new(transport);

        let block_number: BlockId = BlockNumber::Number(block_number.into()).into();
        let block: Option<Block<Transaction>> = web3
            .eth()
            .block_with_txs(block_number)
            .await
            .map_err(|e| EssenError::Web3(e.to_string(), e))?;
        Ok(block)
    }

    pub async fn subscribe_to_new_blocks(
        self,
    ) -> Result<SubscriptionStream<WebSocket, web3::types::BlockHeader>, EssenError> {
        let infura_ws_url = format!("wss://mainnet.infura.io/ws/v3/{}", &self.project_id);
        let transport = WebSocket::new(&infura_ws_url)
            .await
            .map_err(|e| EssenError::Web3(e.to_string(), e))?;
        let web3 = Web3::new(transport);

        let block_number = web3
            .eth()
            .block_number()
            .await
            .map_err(|e| EssenError::Web3(e.to_string(), e))?;

        println!("Latest block number: {:?}", block_number);

        let subscription = web3
            .eth_subscribe()
            .subscribe_new_heads()
            .await
            .map_err(|e| EssenError::Web3(e.to_string(), e))?;

        Ok(subscription)
    }
}
