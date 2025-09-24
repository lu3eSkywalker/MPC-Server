use clap::Parser;
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::Message;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::transaction::Transaction;
use solana_sdk::{native_token, signature::Signer, system_instruction};
use spl_memo::solana_program::pubkey::Pubkey;

use crate::cli::Options;
use crate::error::Error;
use crate::serialization::Serializes;

mod cli;
mod error;
pub mod serialization;
pub mod tss;

pub fn create_unsigned_transaction(amount: f64, to: &Pubkey, memo: Option<String>, payer: &Pubkey) -> Transaction {
    let amount = native_token::sol_to_lamports(amount);
    let transfer_ins = system_instruction::transfer(payer, to, amount);
    let msg = match memo {
        None => Message::new(&[transfer_ins], Some(payer)),
        Some(memo) => {
            let memo_ins = Instruction { program_id: spl_memo::id(), accounts: Vec::new(), data: memo.into_bytes() };
            Message::new(&[transfer_ins, memo_ins], Some(payer))
        }
    };
    Transaction::new_unsigned(msg)
}


pub fn hello() {
    println!("solana-tss library works!");
}