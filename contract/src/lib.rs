// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Promise};
use near_sdk::collections::LookupMap;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DeCash{
   memo: LookupMap<String, Vec<String>>
}

impl Default for DeCash{
  fn default() -> Self {
    Self {
        memo: LookupMap::new(b"memo".to_vec())
    }
  }
}

#[near_bindgen]
impl DeCash{
  
   pub fn add_memo(&mut self, memo_text:String, price:String){
        let account_id=env::signer_account_id();
        let contains_user=self.memo.contains_key(&account_id);
        if contains_user{
            let mut temp_list=match self.memo.get(&account_id){
                Some(x)=>x,
                None=> vec![]
            };

            temp_list.push(memo_text+" || "+ &price+"NEAR");
            self.memo.insert(&account_id, &temp_list);

        }else{
            let fresh_vec=vec![memo_text+" || "+ &price+"NEAR"];
            self.memo.insert(&account_id, &fresh_vec);

        }
   }
   pub fn transfer_money(&mut self, account_id: AccountId, amount:f64){
    Promise::new(account_id).transfer(amount as u128);
   }

   pub fn get_memos(self, user:String) -> Vec<String>{
    match self.memo.get(&user){
        Some(x)=>x,
        None=>vec![]
    }
   }
}
