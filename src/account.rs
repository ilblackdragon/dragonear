use near_sdk::{env, Timestamp};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Account {
    pub cluster_id: u64,
    pub current_dragon: Option<u64>,
    dragon_change_timestamp: Timestamp,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VAccount {
    V1(Account)
}

impl From<VAccount> for Account {
    fn from(c: VAccount) -> Self {
        match c {
            VAccount::V1(c) => c
        }
    }
}

const DAY_DURATION: Timestamp = 24 * 60 * 60 * 1_000_000;

impl Account {
    pub fn new() -> Self {
        Account {
            cluster_id: 0,
            current_dragon: None,
            dragon_change_timestamp: 0,
        }
    }

    pub fn set_dragon(&mut self, dragon_id: u64) {
        assert!(self.dragon_change_timestamp < env::block_timestamp() + DAY_DURATION, "ERR_CHANGE_DRAGON_DELAY");
        if dragon_id == u64::MAX {
            self.current_dragon = None;
        } else {
            self.current_dragon = Some(dragon_id);
        }
        self.dragon_change_timestamp = env::block_timestamp();
    }
}
