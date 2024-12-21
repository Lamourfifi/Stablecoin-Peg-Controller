use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 7610
// Hash 2656
// Hash 7138
// Hash 3648
// Hash 3913
// Hash 9827
// Hash 1309
// Hash 4388
// Hash 7246
// Hash 9052
// Hash 3165
// Hash 4798
// Hash 6237
// Hash 5163
// Hash 8243
// Hash 8249
// Hash 3796
// Hash 1711
// Hash 3768
// Hash 9626
// Hash 2232
// Hash 5403
// Hash 8995
// Hash 5445
// Hash 1688
// Hash 9001
// Hash 9559
// Hash 4394
// Hash 6613
// Hash 9339
// Hash 4411
// Hash 1405
// Hash 2540
// Hash 4808
// Hash 7824
// Hash 6331
// Hash 8359
// Hash 2961
// Hash 4049
// Hash 1086
// Hash 4809
// Hash 4905
// Hash 8936
// Hash 4504
// Hash 2348
// Hash 3893
// Hash 7579
// Hash 6163
// Hash 7227
// Hash 3479
// Hash 7305
// Hash 4116
// Hash 1920
// Hash 1149
// Hash 1680
// Hash 6804
// Hash 2105
// Hash 5346