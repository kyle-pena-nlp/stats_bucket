pub mod push_params;
pub mod combine_params;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum Instructions {

    #[account(0, writable, signer, name = "payer", desc = "payer")]
    #[account(1, name="system_program", desc = "The system program")]
    Push(push_params::PushParams),
    
    #[account(0, writable, signer, name = "payer", desc = "payer")]
    #[account(1, name = "source_bucket_owner", desc = "source bucket owner")]
    #[account(2, name="system_program", desc = "The system program")]
    Combine(combine_params::CombineParams)
}