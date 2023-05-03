mod block_subscriber;
mod block_transaction_detailer;
mod config;
mod errors;
mod helpers;
mod infura_api;

use clap::{arg, command, Command};
use errors::handle_error;

#[tokio::main]
async fn main() {
    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("block_txns")
                .about("Fetch all transactions from a specific Ethereum block. Print in pretty table.")
                .arg(arg!([BLOCK]))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("block_txns_csv")
                .about("Fetch all transactions from a specific Ethereum block. Print in csv format for easy copy and paste to spreadsheet.")
                .arg(arg!([BLOCK]))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("block_txns_stacked")
                .about("Fetch all transactions from a specific Ethereum block. Print in stacked and color coded format for easy reading of an individual transaction.")
                .arg(arg!([BLOCK]))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("subscribe").about(
            "Open a websocket to observe each Ethereum block number as it is added on chain",
        ))
        .subcommand(Command::new("rm_config").about(
            "Remove all config data",
        ))
        .get_matches();

    let subcommand = matches.subcommand();
    let (subcommand, sub_m) = if let Some(subc) = subcommand {
        subc
    } else {
        eprintln!("Missing subcommand.");
        return;
    };

    match subcommand {
        "block_txns" => {
            let block_number = match helpers::get_block_number(sub_m) {
                Ok(block_number) => block_number,
                Err(e) => {
                    handle_error(&e.to_string());
                    return;
                }
            };

            match block_transaction_detailer::get_transaction_details_table(block_number).await {
                Ok(_) => {}
                Err(e) => {
                    handle_error(&e.to_string());
                }
            }
        }
        "block_txns_csv" => {
            let block_number = match helpers::get_block_number(sub_m) {
                Ok(block_number) => block_number,
                Err(e) => {
                    handle_error(&e.to_string());
                    return;
                }
            };

            match block_transaction_detailer::get_transaction_details_csv(block_number).await {
                Ok(_) => {}
                Err(e) => {
                    handle_error(&e.to_string());
                }
            }
        }
        "block_txns_stacked" => {
            let block_number = match helpers::get_block_number(sub_m) {
                Ok(block_number) => block_number,
                Err(e) => {
                    handle_error(&e.to_string());
                    return;
                }
            };

            match block_transaction_detailer::get_transaction_details_stacked(block_number).await {
                Ok(_) => {}
                Err(e) => {
                    handle_error(&e.to_string());
                }
            }
        }
        "subscribe" => match block_subscriber::subscribe_to_block_list().await {
            Ok(_) => {}
            Err(e) => {
                handle_error(&e.to_string());
            }
        },
        "rm_config" => match config::remove_config_file() {
            Ok(_) => println!("Successfully removed config file"),
            Err(e) => {
                handle_error(&e.to_string());
            }
        },
        _ => eprintln!("Invalid subcommand. Run with --help for usage information."),
    }
}
