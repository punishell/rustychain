use chrono::prelude::*;
use rand::Rng;
use libp2p::{
    core::upgrade,
    futures::StreamExt,
    mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::{Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    Transport,
};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime};
use tokio::{
    io::{stdin, AsyncBufReadExt, BufReader},
    select, spawn,
    sync::mpsc,
    time::sleep,
};

const DIFFICULTY_PREFIX: &str = "00";


pub struct App {
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
    pub(crate) transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String,transactions: Vec<Transaction>) -> Self {
        let now = Utc::now();
        let (nonce, hash) = mine_block(id, now.timestamp(), &previous_hash, &data);
        Self {
            id,
            hash,
            timestamp: now.timestamp(),
            previous_hash,
            data,
            nonce,
            transactions,
        }
    }
        fn genesis() -> Self{
            let time_now = Utc::now().timestamp();
            let genesis_hash = String::from("genesis");
            let data = String::from("genesis!");
            let nonce = rand::random();
            let hash = calculate_hash(0, time_now , &genesis_hash, &data, nonce);
        Block {
            id: 0,
            timestamp: time_now,
            previous_hash: genesis_hash,
            data: data,
            nonce: nonce,
            hash: String::from_utf8_lossy(&hash).to_string(),
            transactions: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    nonce: u128,
    from: String,
    created_at: SystemTime,
    pub(crate) record: String,
    signature: Option<String>,
}

impl Transaction {
    pub fn new(from: String, transaction_data: String, nonce: u128) -> Self {
        Transaction {
            from,
            nonce,
            record: transaction_data,
            created_at: SystemTime::now(),
            signature: None,
        }
    }
}

fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    info!("mining block...");
    let mut nonce = 0;

    loop {
        if nonce % 100000 == 0 {
            info!("nonce: {}", nonce);
        }
        let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash = hash_to_binary_representation(&hash);
        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            info!(
                "mined! nonce: {}, hash: {}, binary hash: {}",
                nonce,
                hex::encode(&hash),
                binary_hash
            );
            return (nonce, hex::encode(hash));
        }
        nonce += 1;
    }
}

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

fn main() {
  println!("HELLO WORLDS");
}


#[cfg(test)]
mod tests {
    use super::*;
#[test] 
fn test_genesis_block() {
    //create blockchain
let blockchain = vec![Block::genesis()];
assert_eq!(blockchain[0].id , 0);

}
}
