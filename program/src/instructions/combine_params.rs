use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct CombineParams {
    pub source_name : String,
    pub target_name : String
}