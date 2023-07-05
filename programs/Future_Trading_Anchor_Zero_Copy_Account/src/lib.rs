use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

use borsh::{BorshDeserialize, BorshSerialize};
use std::ops::DerefMut;

#[allow(unused_imports)]
use std::mem::transmute;
declare_id!("C4TTjWcjNv7XcjMPzSTRQaDw5Ka4moCooHS1d1dPjhRn");

const MAX_TRANSACTION: usize = 5000;
const RECOVER_SIZE: usize = 1000;
// const TRANSACTION_TEMPLATE: Contract = Contract {
//     seller_name: [0; 20],
//     seller_birth_day: [0; 20],
//     seller_address: [0; 150],
//     seller_phone: [0; 20],
//     seller_sub_phone: [0; 20],
//     buyer_name: [0; 20],
//     buyer_birth_day: [0; 20],
//     buyer_address: [0; 150],
//     buyer_phone: [0; 20],
//     buyer_sub_phone: [0; 20],
//     item: [0; 20],
//     kind: [0; 20],
//     formal_day: [0; 20],
//     area_flat_unit: [0; 20],
//     address: [0; 150],
//     option: 0,
//     flat_price: 0,
//     contract_price: [0; 20],
//     first_yn: [0; 20],
//     first_price: [0; 20],
//     first_end_count: [0; 20],
//     second_yn: [0; 20],
//     second_price: [0; 20],
//     second_end_count: [0; 20],
//     third_yn: [0; 20],
//     third_price: [0; 20],
//     third_end_count: [0; 20],
//     return_date: [0; 20]
// };

#[program]
pub mod future_trading_anchor_zero_copy_account {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let mut base_account = ctx.accounts.contract_account.load_init()?;
        base_account.counter = 0;

        Ok(())
    }
    pub fn insert(ctx: Context<Insert>, data: ContractInstruction) -> ProgramResult {
        let mut base_account = ctx.accounts.contract_account.load_mut()?;
        let account_data = base_account.deref_mut();

        let instruction_data = parse_instruction_data(data);

        account_data.append(instruction_data);

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(zero)]
    pub contract_account: AccountLoader<'info, ContractAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Insert<'info> {
    #[account(mut)]
    pub contract_account: AccountLoader<'info, ContractAccount>,
}

#[derive(Debug)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ContractInstruction {
    pub seller_name: String,
    pub seller_birth_day: String,
    pub seller_address: String,
    pub seller_phone: String,
    pub seller_sub_phone: String,
    pub buyer_name: String,
    pub buyer_birth_day: String,
    pub buyer_address: String,
    pub buyer_phone: String,
    pub buyer_sub_phone: String,
    pub item: String,
    pub kind: String,
    pub formal_day: String,
    pub area_flat_unit: String,
    pub address: String,
    pub option: i32,
    pub flat_price: i32,
    pub contract_price: i32,
    pub first_yn: i8,
    pub first_price: i32,
    pub first_end_count: i32,
    pub second_yn: i8,
    pub second_price: i32,
    pub second_end_count: i32,
    pub third_yn: i8,
    pub third_price: i32,
    pub third_end_count: i32,
    pub return_date: String
}

#[derive(Debug, PartialEq)]
#[zero_copy]
pub struct Contract {
    pub seller_name: [u8; 20],
    pub seller_birth_day: [u8; 20],
    pub seller_address: [u8; 150],
    pub seller_phone: [u8; 20],
    pub seller_sub_phone: [u8; 20],
    pub buyer_name: [u8; 20],
    pub buyer_birth_day: [u8; 20],
    pub buyer_address: [u8; 150],
    pub buyer_phone: [u8; 20],
    pub buyer_sub_phone: [u8; 20],
    pub item: [u8; 20],
    pub kind: [u8; 20],
    pub formal_day: [u8; 20],
    pub area_flat_unit: [u8; 20],
    pub address: [u8; 150],
    pub option: i32,
    pub flat_price: i32,
    pub contract_price: i32,
    pub first_yn: i8,
    pub first_price: i32,
    pub first_end_count: i32,
    pub second_yn: i8,
    pub second_price: i32,
    pub second_end_count: i32,
    pub third_yn: i8,
    pub third_price: i32,
    pub third_end_count: i32,
    pub return_date: [u8; 20]
}

#[account(zero_copy)]
pub struct ContractAccount {
    pub counter: i32,
    pub contracts: [Contract; MAX_TRANSACTION]
}

impl ContractAccount {
    fn append(&mut self, data: Contract) {
        self.contracts[self.counter as usize] = data;
        self.counter += 1;
    }
}

fn string_to_bytes<S: AsRef<str>>(s: S, len: usize) -> Vec<u8> {
    let mut vec = vec![0; len];
    let bytes = s.as_ref().as_bytes();
    vec[..bytes.len()].copy_from_slice(bytes);
    vec
}

fn parse_instruction_data(data: ContractInstruction) -> Contract {
    Contract {
        seller_name: convert_string_to_20bytes(data.seller_name),
        seller_birth_day: convert_string_to_20bytes(data.seller_birth_day),
        seller_address: convert_string_to_150bytes(data.seller_address),
        seller_phone: convert_string_to_20bytes(data.seller_phone),
        seller_sub_phone: convert_string_to_20bytes(data.seller_sub_phone),
        buyer_name: convert_string_to_20bytes(data.buyer_name),
        buyer_birth_day: convert_string_to_20bytes(data.buyer_birth_day),
        buyer_address: convert_string_to_150bytes(data.buyer_address),
        buyer_phone: convert_string_to_20bytes(data.buyer_phone),
        buyer_sub_phone: convert_string_to_20bytes(data.buyer_sub_phone),
        item: convert_string_to_20bytes(data.item),
        kind: convert_string_to_20bytes(data.kind),
        formal_day: convert_string_to_20bytes(data.formal_day),
        area_flat_unit: convert_string_to_20bytes(data.area_flat_unit),
        address: convert_string_to_150bytes(data.address),
        option: data.option,
        flat_price: data.flat_price,
        contract_price: data.contract_price,
        first_yn: data.first_yn,
        first_price: data.first_price,
        first_end_count: data.first_end_count,
        second_yn: data.second_yn,
        second_price: data.second_price,
        second_end_count: data.second_end_count,
        third_yn: data.third_yn,
        third_price: data.third_price,
        third_end_count: data.third_end_count,
        return_date: convert_string_to_20bytes(data.return_date)
    }
}

#[allow(dead_code)]
fn convert_string_to_10bytes(string: String) -> [u8; 10] {
    
    let mut bytes_array: [u8; 10] = [0; 10];

    let bytes = string.as_bytes().to_vec();
    for (i, b) in bytes.into_iter().enumerate() {
        bytes_array[i] = b;
    }

    bytes_array
}

#[allow(dead_code)]
fn convert_string_to_11bytes(string: String) -> [u8; 11] {
    
    let mut bytes_array: [u8; 11] = [0; 11];

    let bytes = string.as_bytes().to_vec();
    for (i, b) in bytes.into_iter().enumerate() {
        bytes_array[i] = b;
    }

    bytes_array
}

#[allow(dead_code)]
fn convert_string_to_20bytes(string: String) -> [u8; 20] {
    
    let mut bytes_array: [u8; 20] = [0; 20];

    let bytes = string.as_bytes().to_vec();
    for (i, b) in bytes.into_iter().enumerate() {
        bytes_array[i] = b;
    }

    bytes_array
}

#[allow(dead_code)]
fn convert_string_to_100bytes(string: String) -> [u8; 100] {
    
    let mut bytes_array: [u8; 100] = [0; 100];

    let bytes = string.as_bytes().to_vec();
    for (i, b) in bytes.into_iter().enumerate() {
        bytes_array[i] = b;
    }

    bytes_array
}

#[allow(dead_code)]
fn convert_string_to_150bytes(string: String) -> [u8; 150] {
    
    let mut bytes_array: [u8; 150] = [0; 150];

    let bytes = string.as_bytes().to_vec();
    for (i, b) in bytes.into_iter().enumerate() {
        bytes_array[i] = b;
    }

    bytes_array
}

#[allow(dead_code)]
fn convert_string_to_300bytes(string: String) -> [u8; 300] {
    
    let mut bytes_array: [u8; 300] = [0; 300];

    let bytes = string.as_bytes().to_vec();
    for (i, b) in bytes.into_iter().enumerate() {
        bytes_array[i] = b;
    }

    bytes_array
}