use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct CombineParams {
    pub source_name : String,
    pub target_name : String
}


#[repr(C)]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct PushParams {
    pub name : String,
    pub ys : Vec<i64>
}


#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum StatsBucketInstruction {

    #[account(0, signer, name = "payer", desc = "payer")]
    #[account(1, writable, name="bucket", desc = "bucket")]
    #[account(2, name="system_program", desc = "The system program")]
    Push(PushParams),
    
    #[account(0, signer, name = "payer", desc = "payer")]
    #[account(1, name = "source_bucket", desc = "source bucket")]
    #[account(2, writable, name = "target_bucket", desc = "target bucket")]
    #[account(3, name="system_program", desc = "The system program")]
    Combine(CombineParams)
}