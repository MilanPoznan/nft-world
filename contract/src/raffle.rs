use crate::*;

pub const ONE_YOCTO: Balance = 1;
const DELIMETER: &str = "-";

#[near_bindgen]
impl Contract {
    pub fn create_raffle_id(
        nft_contract_id: &AccountId,
        token_id: &TokenId,
        owner_id: &AccountId,
    ) -> String {
        format!(
            "{}{}{}{}{}",
            nft_contract_id, DELIMETER, token_id, DELIMETER, owner_id
        )
    }

    pub fn create_single_raffle(
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
            purchased_tickets: UnorderedMap::new(b"raffle_id".to_vec()),
        };
        return single_raffle;
    }

    pub fn set_approval_id_to_contract(
        &self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
    }

    pub fn send_nft_to_contract(
        &self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> Promise {
        let account: AccountId = "nft-tst.testnet".parse().unwrap();

        // let receiver_id = env::current_account_id();
        let promise = nft_demo::ext(account.clone())
            .with_static_gas(Gas(1 * TGAS))
            .with_attached_deposit(ONE_YOCTO)
            .nft_transfer(receiver_id, token_id, approval_id, memo);

        // let promise = nft_demo::ext(account.clone())
        // .with_static_gas(Gas(10 * TGAS))
        // .nft_transfer(
        //     "nft-proba.testnet",
        // )

        return promise.then(
            Self::ext(env::current_account_id())
                // .with_static_gas(Gas(5 * TGAS))
                .cross_contract_nft_transfer(),
        );
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

        // return match call_result {
        //     Promise(v) => Ok(v.to_string()),
        //     Value(e) => Err(e.into().to_string()),
        // };
        // if call_result.is_ok() {
        //     return Ok("NFT SHOULD BE TRANSFERED".to_string());
        // } else {
        //     return Err(call_result);
        // }
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
