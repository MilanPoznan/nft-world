use std::collections::HashMap;

// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

pub use crate::raffle::*;
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap, UnorderedSet, Vector};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault, Promise,
    PromiseError, PromiseResult,
};

pub use crate::raffle::*;

pub use crate::external::*;
pub use crate::nft_callbacks::*;
pub mod raffle;

pub mod external;
pub mod nft_callbacks;

//RaffledId will be combination contract+tokenId+userAcc
pub type RaffleID = String;
pub type NftId = String;
pub type ContractID = String;

pub struct TicketTransactions {
    pub date_time: String,
    pub transaction: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Tickets {
    number_of_tickets: u32,
    ticket_range: String,
}

// #[derive()]
// #[derive(Hash, Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SingleRaffle {
    pub raffle_id: String,
    pub supply: u32,
    pub ticket_price: String,
    pub end_date: String,
    pub is_ended: bool,
    pub winner: Option<AccountId>,
    pub raffle_creator: AccountId,
    pub sold_tickets: u32,
    pub purchased_tickets: HashMap<String, AccountId>, // pub purchased_tickets: UnorderedMap<AccountId, Tickets>,
}

//BorshDeserialize & BorshSerialize allow the structure to be read and written into the contract's state
//Serialize & Deserialize allow the structure to be used as an input type and return type of the contract's methods.

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SingleUser {
    pub created_raffles: LookupMap<RaffleID, SingleRaffle>,
    pub purchased_raffles: LookupMap<RaffleID, SingleRaffle>,
    pub sales_volumen: u32,
    pub purchased_volumen: u32,
    pub created_raffles_num: u32,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // pub external_account_id: AccountId,
    pub owner_id: AccountId,
    pub supported_nft_contracts: LookupSet<ContractID>,
    pub all_raffles: UnorderedMap<String, SingleRaffle>,
    pub users: UnorderedMap<AccountId, SingleUser>,
    //
    pub by_owner_id: LookupMap<AccountId, UnorderedSet<RaffleID>>,
    //keep track of the storage that accounts have payed
    pub storage_deposits: LookupMap<AccountId, Balance>,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AllRaffles,
    ByOwnerId,
    CreatedRaffles,
    Users,
    PurchasedRaffles,
    PurchasedTickets,
    StorageDeposit,
    SupportedSet,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let mut supported_nft_contracts =
            LookupSet::new(StorageKey::SupportedSet.try_to_vec().unwrap());
        let this = Self {
            owner_id,
            supported_nft_contracts,
            all_raffles: UnorderedMap::new(StorageKey::AllRaffles.try_to_vec().unwrap()),
            users: UnorderedMap::new(StorageKey::Users.try_to_vec().unwrap()),
            by_owner_id: LookupMap::new(StorageKey::ByOwnerId.try_to_vec().unwrap()),
            storage_deposits: LookupMap::new(StorageKey::StorageDeposit.try_to_vec().unwrap()),
        };

        this
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }
}
