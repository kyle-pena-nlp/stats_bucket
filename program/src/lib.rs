mod stats_bucket;
mod instructions;
use instructions::{
    Instructions,
    combine_params::CombineParams,
    push_params::PushParams
};

use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_error::PrintProgramError, 
    msg
};

use borsh::BorshDeserialize;

// TODO: proper program ID from localnet deploy
solana_program::declare_id!("MyProgram1111111111111111111111111111111112");

// Program entry point
entrypoint!(process_instruction);

pub fn process_instruction<'a>(program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8]) -> ProgramResult {


    let data : Instructions = Instructions::try_from_slice(instruction_data)?;

    match data {
        Instructions::Push(push_params) => do_push(&push_params, accounts).unwrap(),
        Instructions::Combine(combine_params) => do_combine(&combine_params, accounts).unwrap()
    }

    Ok(())
}

fn do_push<'a>(params: &PushParams, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {

    let ctx = instructions::accounts::PushAccounts::context(accounts);
    let (bucket_pda, bump) = Pubkey::find_program_address(&[b"bucket", params.name.as_bytes()], &crate::ID);
    Ok(())
}

fn do_combine<'a>(params : &CombineParams, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = instructions::accounts::CombineAccounts::context(accounts);
    Ok(())
}