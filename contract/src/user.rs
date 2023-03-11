use crate::*;

#[near_bindgen]
impl Contract {
    //Init new user after he connects wallet
    pub fn create_user(&mut self, account_id: AccountId) {
        let new_user = SingleUser {
            created_raffles: UnorderedMap::new(StorageKey::CreatedRaffles.try_to_vec().unwrap()),
            purchased_raffles: UnorderedMap::new(
                StorageKey::PurchasedRaffles.try_to_vec().unwrap(),
            ),
            sales_volumen: 0,
            purchased_volumen: 0,
            created_raffles: 0,
        };

        self.users.insert(&account_id, new_user);
    }
}
