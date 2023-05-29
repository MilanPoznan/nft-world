CONT_ID=raffle-nft4.testnet ime kontrakt ownera Tacnije main wallet
nft-tst.testnet 


deploy comand: near deploy --wasmFile ./contract/target/wasm32-unknown-unknown/release/hello_near.wasm --accountId $CONT_ID


** Kada mi je param deserializovan sa serde json (near_sdk::serde_json::from_str(&msg)) pvako pozivam msg param

near call contract-name nft_approve '{"token_id": "2:2", "account_id": "nft-proba.testnet", "msg": "{\"ticket_price\": 100000000000000000000000}" }' --accountId my-account

NEW_CONT=raffle-nft2.testnet
CONT_ID=raffle-nft3.testnet

near call $CONT_ID get_all_raffles --accountId $CONT_ID

