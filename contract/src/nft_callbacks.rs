use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

const GAS_FOR_NFT_ON_TRANSFER: Gas = Gas(9_000_000_000_000);
//oko 8 tgas je slanje jednog NFT

// TODO: Refactor everything
//TODO : asda sas
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RaffleArgs {
    ticket_price: String,
    supply: u32,
    end_date: String,
}

trait NonFungibleTokenApprovalsReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) -> String;

    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_token_owner: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> String;
}

//31 tgas
#[near_bindgen]
impl NonFungibleTokenApprovalsReceiver for Contract {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId, // token.owner_id
        approval_id: u64,
        msg: String,
    ) -> String {
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

        let new_approval = Some(approval_id);
        // nft_contract_id
        let RaffleArgs {
            ticket_price,
            supply,
            end_date,
        } = near_sdk::serde_json::from_str(&msg).expect("Not valid MarketArgs");

        let raffle_id = create_raffle_id(&nft_contract_id, &token_id, &signer_id);

        let option_id = Some(raffle_id.clone());
        let new_raffle = create_single_raffle(
            raffle_id.clone(),
            owner_id.clone(),
            supply,
            ticket_price,
            end_date,
        );

        self.all_raffles.insert(&raffle_id, &new_raffle);

        let promise = ext_nft_contract::ext(nft_contract_id.clone())
            // .with_static_gas(GAS_FOR_NFT_ON_TRANSFER)
            .with_static_gas(Gas(5 * TGAS))
            .with_attached_deposit(ONE_YOCTO)
            .nft_transfer(receiver_id, token_id, new_approval, option_id);
        // .nft_transfer(receiver_id, token_id, new_approval, memo, msg);

        return "Success Approve".to_owned();
    }

    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_token_owner: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> String {
        // enforce cross contract call and owner_id is signer
        //Kontrakt koji me pinga
        let nft_contract_id = env::predecessor_account_id();
        //WAllet koji je zapoceo call
        let signer_id = env::signer_account_id();

        let RaffleArgs {
            ticket_price,
            supply,
            end_date,
        } = near_sdk::serde_json::from_str(&msg).expect("Not valid MarketArgs");

        let raffle_id = create_raffle_id(&nft_contract_id, &token_id, &signer_id);

        let new_raffle = create_single_raffle(
            raffle_id.clone(),
            signer_id.clone(),
            supply,
            ticket_price,
            end_date,
        );

        self.all_raffles.insert(&raffle_id, &new_raffle);

        return "Success Transfer".to_string();
    }
}
