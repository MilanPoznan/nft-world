use core::num;

use crate::*;
use near_sdk::{
    env::{self},
    log, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault, Promise,
    PromiseError, PromiseResult,
};

pub const ONE_YOCTO: Balance = 1;
const DELIMETER: &str = "_";

fn find_winner_by_seed(
    pruchased_tickets_map: &HashMap<String, AccountId>,
    seed: u64,
    raffle_creator: &AccountId,
) -> Option<AccountId> {
    if pruchased_tickets_map.len() == 0 {
        return Some(raffle_creator.clone());
    }

    for (range, account_id) in pruchased_tickets_map {
        //Ako je range of tickets
        if range.contains("..") {
            //Split returns Iterator ->
            //Iterator has impl collect trait which will return Vec from Iterator
            let range_split: Vec<&str> = range.split("..").collect();
            let start: u64 = range_split[0].parse().unwrap();
            let end: u64 = range_split[1].parse().unwrap();

            if seed >= start && seed <= end {
                return Some(account_id.clone());
            }
        } else {
            //Single ticket
            let range_u64: u64 = range.parse().unwrap();
            if range_u64 == seed {
                return Some(account_id.clone());
            }
        }
    }
    None
}

fn get_data_from_raffle_id(raffle_id: &str) -> Vec<&str> {
    let contract_nft_user: Vec<&str> = raffle_id.split("_").collect();
    contract_nft_user
}

pub(crate) fn create_raffle_id(
    nft_contract_id: &AccountId,
    token_id: &TokenId,
    owner_id: &AccountId,
) -> String {
    format!(
        "{}{}{}{}{}",
        nft_contract_id, DELIMETER, token_id, DELIMETER, owner_id
    )
}

pub(crate) fn create_single_raffle(
    raffle_id: String,
    raffle_creator: AccountId,
    supply: u32,
    ticket_price: String,
    end_date: String,
) -> SingleRaffle {
    let purchased_tickets = HashMap::new();

    // let unique_map_str = format!({}{}, "b".to_string())
    let single_raffle = SingleRaffle {
        raffle_id,
        supply,
        ticket_price,
        end_date,
        sold_tickets: 0,
        is_ended: false,
        winner: None,
        raffle_creator,
        purchased_tickets: purchased_tickets,
    };
    single_raffle
}

pub trait RaffleTraits {
    fn get_single_raffle(&self, raffle_id: &RaffleID) -> SingleRaffle;

    fn remove_single_raffle(&mut self, raffle_id: TokenId) -> String;

    fn get_all_raffles(&self) -> Vec<SingleRaffle>;

    fn remove_user_from_raffle(&self, raffle_id: TokenId, tickets: String) -> SingleRaffle;

    // fn test_pseudo_random_number(max_value: u64) -> u64;

    fn pseudo_random_number(&self, raffle_id: String) -> (AccountId, u64, String, String, String);

    fn purchase_raffle(
        &mut self,
        raffle_id: String,
        number_of_tickets: u32,
        purchaser: AccountId,
    ) -> SingleRaffle;

    fn on_nft_transfer_callback(&mut self, nft_id: TokenId);

    fn cancel_raffle_and_return_nft(&mut self, raffle_id: String) {}
}

#[near_bindgen]
impl RaffleTraits for Contract {
    fn cancel_raffle_and_return_nft(&mut self, raffle_id: String) {
        let single_raffle = self.get_single_raffle(&raffle_id);
        let predecessor = env::predecessor_account_id();
        let zero_u32: u32 = 0;

        let creator_address = &single_raffle.raffle_creator;

        assert_eq!(
            creator_address.clone(),
            predecessor,
            "Predecessor has to be a raffle creator"
        );

        assert!(
            &single_raffle.sold_tickets == &zero_u32,
            "Already sold {} amount of tickets",
            &single_raffle.sold_tickets
        );

        self.all_raffles.remove(&raffle_id);

        let contract_nft_user = get_data_from_raffle_id(&raffle_id);

        let nft_id = contract_nft_user[1].to_owned();

        let nft_contract_string = contract_nft_user[0].to_owned();
        let nft_contract = AccountId::try_from(nft_contract_string).unwrap();

        let nft_promise = ext_nft_contract::ext(nft_contract.clone())
            .with_static_gas(Gas(9 * TGAS))
            .with_attached_deposit(ONE_YOCTO)
            .nft_transfer(creator_address.clone(), nft_id, None, None);
    }

    fn get_single_raffle(&self, raffle_id: &RaffleID) -> SingleRaffle {
        self.all_raffles.get(&raffle_id).expect("NO faffle found")
    }

    fn remove_single_raffle(&mut self, raffle_id: RaffleID) -> String {
        let single_raffle = self.get_single_raffle(&raffle_id);
        let message = String::from("Raffle removed sucessfully");

        if single_raffle.is_ended {
            self.all_raffles.remove(&raffle_id);
            return message;
        }

        if single_raffle.is_ended == false && single_raffle.sold_tickets == 0 {
            self.all_raffles.remove(&raffle_id);
            return message;
        }

        if single_raffle.is_ended == false && single_raffle.sold_tickets > 0 {
            return String::from("RAffle can't be removed because tickets are already purchased");
        }

        return "Raffle removed sucessfully".to_string();
    }

    fn remove_user_from_raffle(&self, raffle_id: TokenId, tickets: String) -> SingleRaffle {
        let mut raffle = self.all_raffles.get(&raffle_id).expect("NO faffle found");
        raffle.purchased_tickets.remove(&tickets);
        raffle
    }

    fn get_all_raffles(&self) -> Vec<SingleRaffle> {
        let mut vec: Vec<SingleRaffle> = vec![];
        for raffle in self.all_raffles.values() {
            vec.push(raffle);
        }
        vec
    }

    fn pseudo_random_number(&self, raffle_id: String) -> (AccountId, u64, String, String, String) {
        //Get curr raffle
        let curr_raffle = &self
            .all_raffles
            .get(&raffle_id)
            .expect("No raffle with this ID");

        let max_value = curr_raffle.sold_tickets;
        let sold_tickets_u128: u128 = max_value.to_string().parse().unwrap();
        let ticket_price = &curr_raffle.ticket_price;
        let ticket_price_u128: u128 = ticket_price.to_string().parse().unwrap();

        let amount_to_transfer =
            (sold_tickets_u128 * ticket_price_u128) - (sold_tickets_u128 * ticket_price_u128 / 10);

        //START Calculate random number
        let block_index = env::block_height();
        let block_timestamp = env::block_timestamp();
        let account_id = env::signer_account_id().as_bytes().to_vec();

        let mut seed: u64 = 1;

        for byte in account_id {
            seed = (seed + u64::from(byte)) % max_value as u64;
        }

        seed = (seed + block_index) % max_value as u64;
        seed = (seed + block_timestamp) % max_value as u64;

        // seed
        let winner = find_winner_by_seed(
            &curr_raffle.purchased_tickets,
            seed,
            &curr_raffle.raffle_creator,
        )
        .unwrap();

        //END Calculate random number

        let contract_nft_user = get_data_from_raffle_id(&curr_raffle.raffle_id);

        let creator_address = &curr_raffle.raffle_creator;
        let nft_id = contract_nft_user[1].to_owned();

        let nft_contract_string = contract_nft_user[0].to_owned();
        let nft_contract = AccountId::try_from(nft_contract_string).unwrap();

        //Send NFT to winner
        let nft_promise = ext_nft_contract::ext(nft_contract.clone())
            .with_static_gas(Gas(9 * TGAS))
            .with_attached_deposit(ONE_YOCTO)
            .nft_transfer(winner.clone(), nft_id, None, None);

        //Send money to creator
        let send_money_promise = Promise::new(creator_address.clone()).transfer(amount_to_transfer);

        // let callback_promise = nft_promise.then(
        //     env::current_account_id(),
        //     "on_nft_transfer_callback",
        //     &serde_json::to_vec(&nft_id).expect("Failed to serialize input"),
        //     0, // No attached deposit
        //     Gas(5 * TGAS),
        // );

        //Res => ["nft-tst.testnet", "2:2", "ludikonj.testnet"]
        //Na [0] uraditi cross contarcg call i poslati [1] nft  nft_approve / nft_transfer. Mislim da bih mogao koristiti direktno nft_transfer
        //na [2] poslati pare iz kontrakta, br prodatih karata * price / 7,5

        (
            winner,
            seed,
            amount_to_transfer.to_string(),
            sold_tickets_u128.to_string(),
            ticket_price_u128.to_string(),
        )
    }

    //10tgas dodati na ovo
    #[payable]
    fn purchase_raffle(
        &mut self,
        raffle_id: String,
        number_of_tickets: u32,
        purchaser: AccountId,
    ) -> SingleRaffle {
        let required_gas: Gas = Gas(20_000_000_000_000); // 10 TGas

        assert!(
            env::prepaid_gas() >= required_gas,
            "User must attach at least 20 TGas for this operation."
        );

        let signer_id = env::predecessor_account_id();

        assert_eq!(purchaser, signer_id.clone(), "Buyer should be signer");

        let mut current_raffle = self.all_raffles.get(&raffle_id).expect("NO faffle found");

        let ticket_price = &current_raffle.ticket_price;

        let attached_deposit = env::attached_deposit();

        let u32_price: u128 = ticket_price.to_string().parse().unwrap();

        //Ticket price * number of tickets
        let total_tickets_ammount = number_of_tickets as u128 * u32_price;
        let total_ticket_ammount_128: u128 = total_tickets_ammount.to_string().parse().unwrap();

        let err_mess = format!(
            "Not enough money,\n you insert {},\n and we need {}",
            attached_deposit, total_ticket_ammount_128
        );

        //Assert panic when condition is false
        assert!(attached_deposit >= total_ticket_ammount_128, "{err_mess}");

        //Ticket number calculation
        let max_ticket_number = &current_raffle.supply;
        let sold_tickets_for_now = &current_raffle.sold_tickets;
        let next_ticket = sold_tickets_for_now + 1;
        let ticket_range: String;
        let total_tickets_number = &current_raffle.sold_tickets + &number_of_tickets;

        //Insert ticket range, ticket range can be only single number like 5
        //Or ticket range can be range => 5..10;
        if number_of_tickets == 1 {
            ticket_range = next_ticket.to_string();
        } else {
            ticket_range = format!("{}..{}", next_ticket, total_tickets_number);
        }

        assert!(
            total_tickets_number <= max_ticket_number.clone(),
            "Not enough tickets"
        );

        current_raffle.sold_tickets = total_tickets_number;

        let promise = Promise::new(self.owner_id.clone()).transfer(total_ticket_ammount_128);

        current_raffle
            .purchased_tickets
            .insert(ticket_range, signer_id.clone());

        self.all_raffles.insert(&raffle_id, &current_raffle);

        current_raffle
    }

    // fn test_pseudo_random_number(max_value: u64) -> u64 {
    //     let block_index = env::block_height();
    //     let block_timestamp = env::block_timestamp();
    //     let account_id = env::signer_account_id().as_bytes().to_vec();

    //     let mut seed: u64 = 0;

    //     for byte in account_id {
    //         seed = (seed + u64::from(byte)) % max_value as u64;
    //     }

    //     let seed_log = format!("First {}", seed);

    //     env::log_str(&seed_log);
    //     seed = (seed + block_index) % max_value as u64;

    //     let seed_log2 = format!("Second seed {} and block index -> {}", seed, block_index);

    //     env::log_str(&seed_log2);

    //     seed = (seed + block_timestamp) % max_value as u64;

    //     let seed_log3 = format!(
    //         "Third seed {} and block timestamp -> {}",
    //         seed, block_timestamp
    //     );

    //     env::log_str(&seed_log3);

    //     seed
    // }

    #[private]
    fn on_nft_transfer_callback(&mut self, nft_id: TokenId) {
        let is_transfer_successful = match env::promise_result(0) {
            PromiseResult::Successful(_) => true,
            _ => false,
        };
        if is_transfer_successful {
            env::log(format!("NFT with ID: {} transfer succeeded", nft_id).as_bytes());
        } else {
            env::log(format!("NFT with ID: {} transfer failed", nft_id).as_bytes());
        }
    }
}
