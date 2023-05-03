// src/block_transaction_detailer.rs
use csv::Writer;
use prettytable::{Cell, Row, Table};
use web3::transports::Http;
use web3::types::{Block, BlockId, BlockNumber, Transaction, U256};
use web3::Web3;

use crate::errors::EssenError;

pub async fn get_transaction_details_table(block_number: u64) -> Result<(), EssenError> {
    let block = fetch_block(block_number).await?;

    if let Some(block) = block {
        println!(
            "Block number: {:?}",
            block
                .number
                .ok_or_else(|| EssenError::Block("No Block Number Found".to_string()))
        );

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("From"),
            Cell::new("To"),
            Cell::new("Value (ETH)"),
            Cell::new("Gas"),
            Cell::new("Gas Price (ETH)"),
        ]));

        for tx in block.transactions {
            let value_eth = to_f64(tx.value) / 1e18;
            let gas_price_eth = to_f64(tx.gas_price) / 1e18;
            let decimal_count = 30;

            table.add_row(Row::new(vec![
                Cell::new(&format!("{:?}", tx.from)),
                Cell::new(&format!("{:?}", tx.to)),
                Cell::new(&format!("{:.1$}", value_eth, decimal_count)),
                Cell::new(&format!("{:.1$}", tx.gas, decimal_count)),
                Cell::new(&format!("{:.1$}", gas_price_eth, decimal_count)),
            ]));
        }

        table.printstd();

        Ok(())
    } else {
        Err(EssenError::Block(
            "No block found for the specified block number".to_string(),
        ))
    }
}

pub async fn get_transaction_details_csv(block_number: u64) -> Result<(), EssenError> {
    let block = fetch_block(block_number).await?;

    if let Some(block) = block {
        println!(
            "Block number: {:?}",
            block
                .number
                .ok_or_else(|| EssenError::Block("No Block Number Found".to_string()))
        );
        let mut csv_writer = Writer::from_writer(std::io::stdout());

        csv_writer
            .write_record(["From", "To", "Value (ETH)", "Gas", "Gas Price (ETH)"])
            .map_err(|e| EssenError::Csv(e.to_string(), e))?;

        for tx in block.transactions {
            let value_eth = to_f64(tx.value) / 1e18;
            let gas_price_eth = to_f64(tx.gas_price) / 1e18;
            let from_str = tx
                .from
                .map(|addr| format!("{:?}", addr))
                .unwrap_or_default();
            let to_str = tx.to.map(|addr| format!("{:?}", addr)).unwrap_or_default();
            let decimal_count = 30;

            csv_writer
                .write_record(&[
                    from_str,
                    to_str,
                    format!("{:.1$}", value_eth, decimal_count),
                    format!("{:.1$}", tx.gas, decimal_count),
                    format!("{:.1$}", gas_price_eth, decimal_count),
                ])
                .map_err(|e| EssenError::Csv(e.to_string(), e))?;
        }

        csv_writer
            .flush()
            .map_err(|e| EssenError::Std(e.to_string(), e))?;

        Ok(())
    } else {
        Err(EssenError::Block(
            "No block found for the specified block number".to_string(),
        ))
    }
}

pub async fn get_transaction_details_stacked(block_number: u64) -> Result<(), EssenError> {
    let block = fetch_block(block_number).await?;

    if let Some(block) = block {
        println!(
            "Block number: {:?}",
            block
                .number
                .ok_or_else(|| EssenError::Block("No Block Number Found".to_string()))
        );
        let mut csv_writer = Writer::from_writer(std::io::stdout());

        csv_writer
            .write_record(["From", "To", "Value (ETH)", "Gas", "Gas Price (ETH)"])
            .map_err(|e| EssenError::Csv(e.to_string(), e))?;

        for tx in block.transactions {
            let value_eth = to_f64(tx.value) / 1e18;
            let gas_price_eth = to_f64(tx.gas_price) / 1e18;
            let from_str = tx
                .from
                .map(|addr| format!("{:?}", addr))
                .unwrap_or_default();
            let to_str = tx.to.map(|addr| format!("{:?}", addr)).unwrap_or_default();
            let decimal_count = 30;

            println!("From: {}", from_str);
            println!("To: {}", to_str);
            println!("Value (ETH): {:.1$}", value_eth, decimal_count);
            println!("Gas: {:.1$}", tx.gas, decimal_count);
            println!("Gas Price (ETH): {:.1$}", gas_price_eth, decimal_count);
            println!()
        }

        Ok(())
    } else {
        Err(EssenError::Block(
            "No block found for the specified block number".to_string(),
        ))
    }
}

fn to_f64(value: U256) -> f64 {
    let (quotient, remainder) = value.div_mod(U256::from(10u64.pow(18)));
    let quotient_f64 = quotient.low_u64() as f64;
    let remainder_f64 = remainder.low_u64() as f64 / 1e18;
    quotient_f64 + remainder_f64
}

async fn fetch_block(block_number: u64) -> Result<Option<Block<Transaction>>, EssenError> {
    let infura_http_url = "https://mainnet.infura.io/v3/6376f591d7bd4ca5a4aef588675e6fa6";
    let transport = Http::new(infura_http_url).map_err(|e| EssenError::Web3(e.to_string(), e))?;
    let web3 = Web3::new(transport);
    let block_number: BlockId = BlockNumber::Number(block_number.into()).into();
    let block: Option<Block<Transaction>> = web3
        .eth()
        .block_with_txs(block_number)
        .await
        .map_err(|e| EssenError::Web3(e.to_string(), e))?;
    Ok(block)
}
