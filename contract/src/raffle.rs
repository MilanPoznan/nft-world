use core::num;

use crate::*;

pub const ONE_YOCTO: Balance = 1;
const DELIMETER: &str = "_";

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
        purchased_tickets: purchased_tickets,
    };
    single_raffle
}

pub trait RaffleTraits {
    fn get_single_raffle(&self, raffle_id: TokenId) -> SingleRaffle;

    fn get_all_raffles(&self) -> Vec<SingleRaffle>;

    fn remove_user_from_raffle(&self, raffle_id: TokenId, tickets: String) -> SingleRaffle;

    fn test_pseudo_random_number(max_value: u64) -> u64;

    fn pseudo_random_number(&self, raffle_id: String) -> u64;

    fn purchase_raffle(
        &mut self,
        raffle_id: String,
        number_of_tickets: u32,
        purchaser: AccountId,
    ) -> SingleRaffle;
}

#[near_bindgen]
impl RaffleTraits for Contract {
    fn get_single_raffle(&self, raffle_id: TokenId) -> SingleRaffle {
        self.all_raffles.get(&raffle_id).expect("NO faffle found")
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

    fn test_pseudo_random_number(max_value: u64) -> u64 {
        let block_index = env::block_height();
        let block_timestamp = env::block_timestamp();
        let account_id = env::signer_account_id().as_bytes().to_vec();

        let mut seed: u64 = 0;

        for byte in account_id {
            seed = (seed + u64::from(byte)) % max_value as u64;
        }

        let seed_log = format!("First {}", seed);

        env::log_str(&seed_log);
        seed = (seed + block_index) % max_value as u64;

        let seed_log2 = format!("Second seed {} and block index -> {}", seed, block_index);

        env::log_str(&seed_log2);

        seed = (seed + block_timestamp) % max_value as u64;

        let seed_log3 = format!(
            "Third seed {} and block timestamp -> {}",
            seed, block_timestamp
        );

        env::log_str(&seed_log3);

        seed
    }

    fn pseudo_random_number(&self, raffle_id: String) -> u64 {
        let curr_raffle = &self
            .all_raffles
            .get(&raffle_id)
            .expect("No raffle with this ID");

        let max_value = curr_raffle.sold_tickets;

        let block_index = env::block_height();
        let block_timestamp = env::block_timestamp();
        let account_id = env::signer_account_id().as_bytes().to_vec();

        let mut seed: u64 = 0;

        for byte in account_id {
            seed = (seed + u64::from(byte)) % max_value as u64;
        }

        seed = (seed + block_index) % max_value as u64;
        seed = (seed + block_timestamp) % max_value as u64;

        seed
    }

    #[payable]
    fn purchase_raffle(
        &mut self,
        raffle_id: String,
        number_of_tickets: u32,
        purchaser: AccountId,
    ) -> SingleRaffle {
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
            total_tickets_number < max_ticket_number.clone(),
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

    // #[private]
    // #[handle_result]
    // fn cross_contract_nft_transfer(
    //     &self,
    //     #[callback_result] call_result: Result<String, PromiseError>,
    // ) -> Result<String, String> {
    //     return match call_result {
    //         Ok(v) => Ok(v.to_string()),
    //         Err(e) => Err("Error occurs here".to_string()),
    //     };
    // }

    // #[private]
    // #[handle_result]
    // fn cross_callback(
    //     &self,
    //     #[callback_result] call_result: Result<NFTContractMetadata, PromiseError>,
    // ) -> Result<NFTContractMetadata, String> {
    //     // if call_result.is_err() {
    //     //     return "".to_string();
    //     // }
    //     if call_result.is_ok() {
    //         return Ok(call_result.unwrap());
    //     } else {
    //         return Err(String::from("Some error occurs"));
    //     }
    //     // Return the greeting
    //     // let greeting: NFTContractMetadata = call_result.unwrap_or_default();
    //     // let greeting: String = String::from("Sve ok");
    //     // greeting
    // }
}
