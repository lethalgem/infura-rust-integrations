use web3::futures::StreamExt;

use crate::errors::EssenError;
use crate::{config, infura_api};

pub async fn subscribe_to_block_list() -> Result<(), EssenError> {
    let project_id = config::get_infura_keys().ok_or_else(|| EssenError::InfuraProjectIdError)?;
    let infura_api = infura_api::InfuraApi::new(&project_id);

    let mut sub = infura_api.subscribe_to_new_blocks().await?;

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
