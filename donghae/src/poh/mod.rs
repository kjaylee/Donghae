// src/poh/mod.rs

use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Poh {
    hash: [u8; 32],
}

impl Poh {
    pub fn new(seed: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(seed);
        Poh {
            hash: hasher.finalize().into(),
        }
    }

    pub fn next_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(&self.hash);
        self.hash = hasher.finalize().into();
    }

    pub fn get_hash(&self) -> [u8; 32] {
        self.hash
    }

    pub fn get_timestamp() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("시간을 가져오는데 실패했습니다.");
        since_the_epoch.as_secs()
    }
}
