use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RaffleArgs {
    ticket_price: i32,
}
trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
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

        let memo = Some(String::from("Hello, world!"));

        let new_approval = Some(approval_id);
        // nft_contract_id
        let RaffleArgs { ticket_price } =
            near_sdk::serde_json::from_str(&msg).expect("Not valid MarketArgs");

        let promise = ext_nft_contract::ext(nft_contract_id.clone())
            .with_static_gas(Gas(1 * TGAS))
            .with_attached_deposit(ONE_YOCTO)
            .nft_transfer(receiver_id, token_id, new_approval, memo);

        let owner_paid_storage = self.storage_deposits.get(&signer_id).unwrap_or(0);
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
