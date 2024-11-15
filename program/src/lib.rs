mod stats_bucket;
mod stats_bucket_instruction;
mod stats_bucket_account;
mod errors;
mod fixed_point_stuff;

use fixed_point_stuff::vec_i64_to_vec_fixed_point;
use stats_bucket_instruction::{
    StatsBucketInstruction,
    PushParams,
    CombineParams
};
use errors::Errors::{
    WrongBucketPDA,
    WrongSourceBucketPDA,
    WrongTargetBucketPDA,
    SerializationError
};

use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    program::invoke_signed,
    system_instruction,
    sysvar::Sysvar
};

use borsh::{BorshDeserialize,BorshSerialize};
use stats_bucket::StatsBucket;
use stats_bucket_account::StatsBucketAccount;
use crate::fixed_point_stuff::into_fixed_point_array;

// TODO: proper program ID from localnet deploy
solana_program::declare_id!("C7DVvsaSQ1k7XcUXoAh9gZyGs6Ki9Qg9zpriBbrcx6tm");

// Program entry point
entrypoint!(process_instruction);

pub fn process_instruction<'a>(program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8]) -> ProgramResult {

    // deserialize instructions
    let data : StatsBucketInstruction = StatsBucketInstruction::try_from_slice(instruction_data)?;

    // depending on the instruction type, push to a bucket or merge the source bucket to the target (init if needed)
    match data {
        StatsBucketInstruction::Push(push_params) => do_push(&push_params, accounts).unwrap(),
        StatsBucketInstruction::Combine(combine_params) => do_combine(&combine_params, accounts).unwrap()
    }

    Ok(())
}

fn do_push<'a>(params: &PushParams, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {

    // deserialize accounts from instruction
    let ctx = stats_bucket_instruction::accounts::PushAccounts::context(accounts).unwrap();

    let bucket_account = ctx.accounts.bucket;
    let payer_account = ctx.accounts.payer;

    // derive the PDA for the payer's bucket
    let (bucket_pda, bump) = Pubkey::find_program_address(&[b"bucket", payer_account.key.as_ref(),  params.name.as_bytes()], &crate::ID);

    // make sure the PDA matches what was supplied by the client
    if *bucket_account.key != bucket_pda {
        return Err(WrongBucketPDA.into())
    }

    if bucket_account.data_is_empty() {
        invoke_signed(&system_instruction::create_account(
            payer_account.key,
            bucket_account.key,
            Rent::get()?.minimum_balance(StatsBucketAccount::get_size()),
            StatsBucketAccount::get_size() as u64,
            &crate::ID
        ), 
        &[
            payer_account.clone(),
            bucket_account.clone(),
        ],
        &[&[bucket_pda.as_ref(), &[bump]]])?;
    }

    // deserialize the bucket PDA
    let data = &bucket_account.data.borrow();
    let mut stats_bucket_account = StatsBucketAccount::try_from_slice(data).unwrap();

    // convert the account into a stats bucket object, and update the statistics with the data
    let mut stats_bucket : StatsBucket = stats_bucket_account.as_stats_bucket();
    stats_bucket.update(&vec_i64_to_vec_fixed_point(&params.ys));

    // write the results back to the account data
    stats_bucket_account.copy_from_stats_bucket(stats_bucket);
    let mut buffer = Vec::new();
    stats_bucket_account.serialize(&mut buffer).map_err(|_| SerializationError).unwrap();
    bucket_account.data.borrow_mut().copy_from_slice(&buffer);

    Ok(())
}

fn do_combine<'a>(params : &CombineParams, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = stats_bucket_instruction::accounts::CombineAccounts::context(accounts).unwrap();

    let payer_account = ctx.accounts.payer;
    let source_bucket_account = ctx.accounts.source_bucket;
    let target_bucket_account = ctx.accounts.target_bucket;

    let (source_bucket_pda, _) = Pubkey::find_program_address(&[b"bucket", ctx.accounts.payer.key.as_ref(), params.source_name.as_bytes() ], &crate::ID);

    if *source_bucket_account.key != source_bucket_pda {
        return Err(WrongSourceBucketPDA.into());
    }

    let (target_bucket_pda, target_bucket_bump) = Pubkey::find_program_address(&[b"bucket", ctx.accounts.payer.key.as_ref(), params.target_name.as_bytes()], &crate::ID);

    if *target_bucket_account.key != target_bucket_pda {
        return Err(WrongTargetBucketPDA.into());
    }

    if target_bucket_account.data_is_empty() {
        invoke_signed(&system_instruction::create_account(
            payer_account.key,
            target_bucket_account.key,
            Rent::get()?.minimum_balance(StatsBucketAccount::get_size()),
            StatsBucketAccount::get_size() as u64,
            &crate::ID
        ), 
        &[
            payer_account.clone(),
            target_bucket_account.clone(),
        ],
        &[&[target_bucket_pda.as_ref(), &[target_bucket_bump]]])?;
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
    target_bucket_account.serialize(&mut buffer).map_err(|_| SerializationError).unwrap();

    Ok(())
}