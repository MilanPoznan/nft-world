use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

const GAS_FOR_NFT_ON_TRANSFER: Gas = Gas(40_000_000_000_000);

// TODO: Refactor everything
//TODO : asda sas
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RaffleArgs {
    ticket_price: String,
    // supply: u32,
    // end_date: String,
}
trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );

    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_token_owner: AccountId,
        token_id: TokenId,
        msg: String,
    );
}

#[near_bindgen]
impl NonFungibleTokenApprovalsReceiver for Contract {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId, // token.owner_id
        approval_id: u64,
        msg: String,
    ) {
        // enforce cross contract call and owner_id is signer
        //Kontrakt koji me pinga
        let nft_contract_id = env::predecessor_account_id();
        //WAllet koji je zapoceo call
        let signer_id = env::signer_account_id();
        let receiver_id = env::current_account_id();

        assert_ne!(
            env::current_account_id(), //Account ID of this smart contract
            nft_contract_id,
            "Our: nft_on_approve should only be called via cross-contract call"
        );

        assert_eq!(
            owner_id, signer_id,
            "NFT WORLD: owner_id should be signer_id"
        );

        let memo = Some(String::from("H"));

        let new_approval = Some(approval_id);
        // nft_contract_id
        let RaffleArgs {
            ticket_price,
            // supply,
            // end_date,
        } = near_sdk::serde_json::from_str(&msg).expect("Not valid MarketArgs");

        let promise = ext_nft_contract::ext(nft_contract_id.clone())
            // .with_static_gas(GAS_FOR_NFT_ON_TRANSFER)
            .with_static_gas(Gas(24 * TGAS))
            .with_attached_deposit(ONE_YOCTO)
            // .nft_transfer(receiver_id, token_id, new_approval, memo);
            .nft_transfer_call(receiver_id, token_id, new_approval, memo, msg);

        let owner_paid_storage = self.storage_deposits.get(&signer_id).unwrap_or(0);
    }

    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_token_owner: AccountId,
        token_id: TokenId,
        msg: String,
    ) {
        // enforce cross contract call and owner_id is signer
        //Kontrakt koji me pinga
        let nft_contract_id = env::predecessor_account_id();
        //WAllet koji je zapoceo call
        let signer_id = env::signer_account_id();

        let RaffleArgs {
            ticket_price,
            // supply,
            // end_date,
        } = near_sdk::serde_json::from_str(&msg).expect("Not valid MarketArgs");

        let raffle_id = Contract::create_raffle_id(&nft_contract_id, &token_id, &signer_id);

        // let new_raffle =
        //     Contract::create_single_raffle(raffle_id.clone(), supply, ticket_price, end_date);

        // self.all_raffles.insert(&raffle_id, &new_raffle);
    }

    // #[private]
    // #[handle_result]
    // pub fn on_nft_trasfer(&self, #[callback_result] call_result: Result<()>) {
    //     match call_result {
    //         Ok(_) => String::from("NFT transfered successfully"),
    //         Err(e) => String::from("NFT transfered failed"),
    //     }
    // }
}
