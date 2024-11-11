mod stats_bucket;
mod instructions;
mod stats_bucket_account;
mod errors;

use instructions::{
    Instructions,
    combine_params::CombineParams,
    push_params::PushParams
};

use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult,
    pubkey::Pubkey
};

use borsh::{BorshDeserialize,BorshSerialize};
use stats_bucket::StatsBucket;
use stats_bucket_account::StatsBucketAccount;

// TODO: proper program ID from localnet deploy
solana_program::declare_id!("MyProgram1111111111111111111111111111111112");

// Program entry point
entrypoint!(process_instruction);

pub fn process_instruction<'a>(program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8]) -> ProgramResult {

    // deserialize instructions
    let data : Instructions = Instructions::try_from_slice(instruction_data)?;

    // depending on the instruction type, push to a bucket or merge the source bucket to the target (init if needed)
    match data {
        Instructions::Push(push_params) => do_push(&push_params, accounts).unwrap(),
        Instructions::Combine(combine_params) => do_combine(&combine_params, accounts).unwrap()
    }

    Ok(())
}

fn do_push<'a>(params: &PushParams, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {

    // deserialize accounts from instruction
    let ctx = instructions::accounts::PushAccounts::context(accounts).unwrap();

    // derive the PDA for the payer's bucket
    let (bucket_pda, bump) = Pubkey::find_program_address(&[b"bucket", ctx.accounts.payer.key.as_ref(),  params.name.as_bytes()], &crate::ID);

    // make sure the PDA matches what was supplied by the client
    let bucket = ctx.accounts.bucket;
    if *bucket.key != bucket_pda {
        return Err(errors::Errors::WrongBucketPDA.into())
    }

    // deserialize the bucket PDA
    let data = &bucket.data.borrow();
    let mut stats_bucket_account = StatsBucketAccount::try_from_slice(data).unwrap();

    // convert the account into a stats bucket object, and update the statistics with the data
    let mut stats_bucket : StatsBucket = stats_bucket_account.as_stats_bucket();
    stats_bucket.update(&params.ys);

    // write the results back to the account data
    stats_bucket_account.copy_from_stats_bucket(stats_bucket);
    let mut buffer = Vec::new();
    stats_bucket_account.serialize(&mut buffer).map_err(|_| errors::Errors::SerializationError).unwrap();
    bucket.data.borrow_mut().copy_from_slice(&buffer);

    Ok(())
}

fn do_combine<'a>(params : &CombineParams, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = instructions::accounts::CombineAccounts::context(accounts).unwrap();

    let (target_bucket_pda, bump) = Pubkey::find_program_address(&[b"bucket", ctx.accounts.payer.key.as_ref(), params.target_name.as_bytes()], &crate::ID);

    if *ctx.accounts.target_bucket.key != target_bucket_pda {
        return Err(errors::Errors::WrongTargetBucketPDA.into());
    }

    // deserialize account data
    let source_bucket_account = StatsBucketAccount::try_from_slice(&ctx.accounts.source_bucket.data.borrow()).unwrap();
    let mut target_bucket_account = StatsBucketAccount::try_from_slice(&ctx.accounts.target_bucket.data.borrow()).unwrap();

    // convert to stats bucket objects
    let source_bucket = source_bucket_account.as_stats_bucket();
    let mut target_bucket = target_bucket_account.as_stats_bucket();

    // combine the buckets
    target_bucket.combine(&source_bucket);

    // deserialize and write back to target bucket
    target_bucket_account.copy_from_stats_bucket(target_bucket);
    let mut buffer = Vec::new();
    target_bucket_account.serialize(&mut buffer).map_err(|_| errors::Errors::SerializationError).unwrap();

    Ok(())
}