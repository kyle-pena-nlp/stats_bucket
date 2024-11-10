use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct PushParams {
    pub name : String,
    pub ys : Vec<f32>
}
