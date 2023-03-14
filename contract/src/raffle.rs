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
    // let unique_map_str = format!({}{}, "b".to_string())
    let single_raffle = SingleRaffle {
        raffle_id,
        supply,
        ticket_price,
        end_date,
        // purchased_tickets: UnorderedMap::new(b"raffle_id".to_vec()),
    };
    single_raffle
}

#[near_bindgen]
impl Contract {
    pub fn get_single_raffles(&self, raffle_id: TokenId) -> SingleRaffle {
        // assert!(self.all_raffles.is_empty() == true, "No Raffles ");

        let x = self.all_raffles.get(&raffle_id).expect("NO faffle found");
        x
    }

    pub fn get_all_raffles(&mut self) -> Vec<SingleRaffle> {
        let mut vec: Vec<SingleRaffle> = vec![];
        for raffle in self.all_raffles.values() {
            vec.push(raffle);
        }
        vec
    }

    // pub fn test_raffle(&self) {
    //     self.all_raffles.keys(
    // }
    // pub fn get_state(self) -> UnorderedMap<String, SingleRaffle> {
    //     self.all_raffles
    // }

    pub fn insert_raffle_to_state(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId, // token.owner_id
        nft_contract_id: AccountId,
        supply: u32,
        ticket_price: String,
        end_date: String,
    ) -> SingleRaffle {
        let id = create_raffle_id(&nft_contract_id, &token_id, &owner_id);
        let single_raffle = create_single_raffle(id.clone(), supply, ticket_price, end_date);
        self.all_raffles.insert(&id, &single_raffle);

        let raffle = self.all_raffles.get(&id).expect("No raffle");
        raffle
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
