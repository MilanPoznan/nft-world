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

#[near_bindgen]
impl Contract {
    pub fn get_single_raffles(&self, raffle_id: TokenId) -> SingleRaffle {
        self.all_raffles.get(&raffle_id).expect("NO faffle found")
    }

    pub fn get_all_raffles(&self) -> Vec<SingleRaffle> {
        let mut vec: Vec<SingleRaffle> = vec![];
        for raffle in self.all_raffles.values() {
            vec.push(raffle);
        }
        vec
    }

    //Just for testing purpose
    pub fn insert_raflle_test(
        &mut self,
        nft_contract_id: AccountId,
        token_id: TokenId,
        owner_id: AccountId,
        supply: u32,
        ticket_price: String,
        end_date: String,
    ) {
        // let signer_id = env::signer_account_id();

        let raffle_id = create_raffle_id(&nft_contract_id, &token_id, &owner_id);

        let new_raffle = create_single_raffle(raffle_id.clone(), supply, ticket_price, end_date);

        self.all_raffles.insert(&raffle_id, &new_raffle);
    }

    #[payable]
    pub fn purchase_raffle(
        &mut self,
        raffle_id: String,
        number_of_tickets: u32,
        purchaser: AccountId,
    ) -> SingleRaffle {
        let signer_id = env::predecessor_account_id();

        assert_eq!(purchaser, signer_id, "Buyer should be signer");

        let mut current_raffle = self.all_raffles.get(&raffle_id).expect("NO faffle found");

        let ticket_price = &current_raffle.ticket_price;

        let attached_deposit = env::attached_deposit();

        let u32_price: u32 = ticket_price.to_string().parse().unwrap();

        let total_ticket_ammount = number_of_tickets * u32_price;

        //Increase tickets
        let curr_tickets_num = current_raffle.sold_tickets;
        let total_tickets = current_raffle.sold_tickets + number_of_tickets;

        let ticket_range = format!("{}-{}", curr_tickets_num, total_tickets);
        let total_ticket_ammount_128: u128 = total_ticket_ammount.to_string().parse().unwrap();

        assert!(
            attached_deposit <= total_ticket_ammount_128,
            "Not enough money"
        );

        let new_ticket = Tickets {
            number_of_tickets,
            ticket_range,
        };

        current_raffle
            .purchased_tickets
            .insert(signer_id.clone(), new_ticket);

        //Increase number of sold tickets in state
        current_raffle.sold_tickets = total_tickets;

        current_raffle
    }

    #[private]
    #[handle_result]
    pub fn cross_contract_nft_transfer(
        &self,
        #[callback_result] call_result: Result<String, PromiseError>,
    ) -> Result<String, String> {
        return match call_result {
            Ok(v) => Ok(v.to_string()),
            Err(e) => Err("Error occurs here".to_string()),
        };
    }

    #[private]
    #[handle_result]
    pub fn cross_callback(
        &self,
        #[callback_result] call_result: Result<NFTContractMetadata, PromiseError>,
    ) -> Result<NFTContractMetadata, String> {
        // if call_result.is_err() {
        //     return "".to_string();
        // }
        if call_result.is_ok() {
            return Ok(call_result.unwrap());
        } else {
            return Err(String::from("Some error occurs"));
        }
        // Return the greeting
        // let greeting: NFTContractMetadata = call_result.unwrap_or_default();
        // let greeting: String = String::from("Sve ok");
        // greeting
    }
}
