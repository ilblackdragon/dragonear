use near_sdk::{CryptoHash, env, Timestamp};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::dragon::Dragon;

const BATTLE_MAX_DURATION: Timestamp = 10 * 60 * 1_000_000;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Battle {
    start_timestamp: Timestamp,
    pub dragon_a: u64,
    pub dragon_b: u64,
    hash_actions_a: Option<CryptoHash>,
    hash_actions_b: Option<CryptoHash>,
    actions_a: Option<Vec<u8>>,
    actions_b: Option<Vec<u8>>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VBattle {
    V1(Battle)
}

impl From<VBattle> for Battle {
    fn from(c: VBattle) -> Self {
        match c {
            VBattle::V1(c) => c
        }
    }
}

impl Battle {
    pub fn new(dragon_a: u64, dragon_b: u64) -> Self {
        Self {
            start_timestamp: env::block_timestamp(),
            dragon_a,
            dragon_b,
            hash_actions_a: None,
            hash_actions_b: None,
            actions_a: None,
            actions_b: None,
        }
    }

    pub fn set_hash_actions(&mut self, dragon_id: u64, hash_actions: CryptoHash) {
        assert!(self.dragon_a == dragon_id || self.dragon_b == dragon_id, "ERR_NOT_YOUR_BATTLE");
        if self.dragon_a == dragon_id {
            self.hash_actions_a = Some(hash_actions)
        } else {
            self.hash_actions_b = Some(hash_actions)
        }
    }

    pub fn set_actions(&mut self, dragon_id: u64, actions: Vec<u8>) {
        assert!(self.dragon_a == dragon_id || self.dragon_b == dragon_id, "ERR_NOT_YOUR_BATTLE");
        // TODO: check hashes.
        if self.dragon_a == dragon_id {
            self.actions_a = Some(actions);
        } else {
            self.actions_b = Some(actions);
        }
    }

    pub fn complete(&self) -> bool {
        // TODO: add expiry
        self.actions_a.is_some() && self.actions_b.is_some() || env::block_timestamp() >= self.start_timestamp + BATTLE_MAX_DURATION
    }

    pub fn run(&self, dragon_a: &mut Dragon, dragon_b: &mut Dragon) {
        // TODO: run the battle.
        let mut i = 0;
        let mut j = 0;
        let actions_a = self.actions_a.clone().unwrap_or(vec![0]);
        let actions_b = self.actions_b.clone().unwrap_or(vec![0]);
        let mut constitution_a = dragon_a.constitution.clone();
        let mut constitution_b = dragon_b.constitution.clone();
        loop {
            dragon_a.apply_skill(&mut constitution_a, &mut constitution_b, actions_a[i]);
            dragon_b.apply_skill(&mut constitution_b, &mut constitution_a, actions_b[i]);
            i = (i + 1) % actions_a.len();
            j = (j + 1) % actions_b.len();
        }
    }
}
