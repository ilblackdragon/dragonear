use near_sdk::{AccountId, env, Timestamp};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Cluster {
    pub max_lvl: u8,
    /// Last dragon that is waiting for battle in this cluster.
    pub waiting_for_battle: Option<u64>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VCluster {
    V1(Cluster)
}

impl From<VCluster> for Cluster {
    fn from(c: VCluster) -> Self {
        match c {
            VCluster::V1(c) => c
        }
    }
}

impl Cluster {
}