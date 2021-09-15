use near_sdk::{AccountId, CryptoHash, env, Timestamp, PanicOnDefault, BorshStorageKey, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{Base58CryptoHash, ValidAccountId};

use crate::account::{Account, VAccount};
use crate::battle::{Battle, VBattle};
use crate::cluster::{Cluster, VCluster};
use crate::dragon::{Dragon, VDragon};

mod account;
mod dragon;
mod cluster;
mod battle;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Accounts,
    Dragons,
    Clusters,
    Battles,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
struct Contract {
    owner_id: AccountId,
    accounts: LookupMap<AccountId, VAccount>,
    dragons: LookupMap<u64, VDragon>,
    clusters: LookupMap<u64, VCluster>,
    battles: LookupMap<String, VBattle>,
    last_dragon_id: u64,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId) -> Self {
        let mut this = Self {
            owner_id: owner_id.as_ref().clone(),
            accounts: LookupMap::new(StorageKey::Accounts),
            dragons: LookupMap::new(StorageKey::Dragons),
            clusters: LookupMap::new(StorageKey::Clusters),
            battles: LookupMap::new(StorageKey::Battles),
            last_dragon_id: 0,
        };
        this.set_cluster(0, Cluster { max_lvl: 255, waiting_for_battle: None });
        this
    }

    pub fn create_account(&mut self) {
        let account_id = env::predecessor_account_id();
        assert!(!self.accounts.contains_key(&account_id), "ERR_ACCOUNT_EXISTS");
        self.set_account(&account_id, Account::new());
    }

    pub fn dragon_create(&mut self, account_id: ValidAccountId) -> u64 {
        assert_eq!(env::predecessor_account_id(), self.owner_id);
        let dragon = Dragon::random(account_id.as_ref());
        self.set_dragon(self.last_dragon_id, dragon);
        self.last_dragon_id += 1;
        self.last_dragon_id - 1
    }

    pub fn dragon_select(&mut self, dragon_id: u64) {
        let account_id = env::predecessor_account_id();
        let mut account = self.get_account(&account_id);
        if dragon_id != u64::MAX {
            let dragon = self.get_dragon(dragon_id);
            assert_eq!(dragon.owner_id, account_id, "ERR_NOT_OWNER");
        }
        account.set_dragon(dragon_id);
        self.set_account(&account_id, account);
    }

    fn internal_get_info(&self) -> (Account, Cluster, u64, Dragon) {
        let account = self.get_account(&env::predecessor_account_id());
        let cluster = self.get_cluster(account.cluster_id);
        let dragon_id = account.current_dragon.expect("ERR_NO_DRAGON_SELECTED");
        let dragon = self.get_dragon(dragon_id);
        (account, cluster, dragon_id, dragon)
    }

    fn internal_create_battle(&mut self, dragon_a: u64, dragon_b: u64) {
        let battle = Battle::new(dragon_a, dragon_b);
        self.set_battle(&format!("{}:{}", dragon_a, dragon_b), battle);
    }

    pub fn battle_start(&mut self) -> bool {
        let (account, mut cluster, dragon_id, dragon) = self.internal_get_info();
        assert!(dragon.level <= cluster.max_lvl, "ERR_HIGH_LEVEL");
        let started_battle = if let Some(waiting_dragon_id) = cluster.waiting_for_battle {
            self.internal_create_battle(dragon_id, waiting_dragon_id);
            cluster.waiting_for_battle = None;
            true
        } else {
            cluster.waiting_for_battle = Some(dragon_id);
            false
        };
        self.set_cluster(account.cluster_id, cluster);
        started_battle
    }

    pub fn battle_commit_actions(&mut self, battle_id: String, hash_actions: Base58CryptoHash) {
        let (account, mut cluster, dragon_id, dragon) = self.internal_get_info();
        let mut battle = self.get_battle(&battle_id);
        battle.set_hash_actions(dragon_id, hash_actions.into());
        self.set_battle(&battle_id, battle);
    }

    pub fn battle_reveal_actions(&mut self, battle_id: String, actions: Vec<u8>) {
        let (account, mut cluster, dragon_id, mut dragon) = self.internal_get_info();
        let mut battle = self.get_battle(&battle_id);
        battle.set_actions(dragon_id, actions);
        if battle.complete() {
            let other_dragon_id = if dragon_id == battle.dragon_a { battle.dragon_b } else { battle.dragon_a };
            let mut other_dragon = self.get_dragon(other_dragon_id);
            if dragon_id == battle.dragon_a {
                battle.run(&mut dragon, &mut other_dragon);
            } else {
                battle.run(&mut other_dragon, &mut dragon);
            }
            self.set_dragon(dragon_id, dragon);
            self.set_dragon(other_dragon_id, other_dragon);
        }
        self.set_battle(&battle_id, battle);
    }

    pub fn cluster_select(&mut self, cluster_id: u64) {
        let account_id = env::predecessor_account_id();
        let mut account = self.get_account(&account_id);
        // TODO: add more properties of the cluster.
        account.cluster_id = cluster_id;
        self.set_account(&account_id, account);
    }
}

impl Contract {
    fn get_account(&self, account_id: &AccountId) -> Account {
        self.accounts.get(&account_id).expect("NO_ACCOUNT").into()
    }

    fn set_account(&mut self, account_id: &AccountId, account: Account) {
        self.accounts.insert(&account_id, &VAccount::V1(account));
    }

    fn get_dragon(&self, dragon_id: u64) -> Dragon {
        self.dragons.get(&dragon_id).expect("NO_DRAGON").into()
    }

    fn set_dragon(&mut self, dragon_id: u64, dragon: Dragon) {
        self.dragons.insert(&dragon_id, &VDragon::V1(dragon));
    }

    fn get_cluster(&self, cluster_id: u64) -> Cluster {
        self.clusters.get(&cluster_id).expect("NO_CLUSTER").into()
    }

    fn set_cluster(&mut self, cluster_id: u64, cluster: Cluster) {
        self.clusters.insert(&cluster_id, &VCluster::V1(cluster));
    }

    fn get_battle(&self, battle_id: &String) -> Battle {
        self.battles.get(&battle_id).expect("NO_BATTLE").into()
    }

    fn set_battle(&mut self, battle_id: &String, battle: Battle) {
        self.battles.insert(&battle_id, &VBattle::V1(battle));
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{MockedBlockchain, testing_env};
    use near_sdk::test_utils::{accounts, VMContextBuilder};

    use super::*;

    #[test]
    pub fn test_basic() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let mut contract = Contract::new(accounts(0));

        for i in 1..3 {
            testing_env!(context.predecessor_account_id(accounts(i)).build());
            contract.create_account();
        }

        testing_env!(context.predecessor_account_id(accounts(0)).build());
        for i in 1..3 {
            let _ = contract.dragon_create(accounts(i));
        }

        testing_env!(context.predecessor_account_id(accounts(1)).build());
        contract.dragon_select(0);
        assert!(!contract.battle_start());
        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.dragon_select(1);
        assert!(contract.battle_start());
    }
}
