use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

use borsh::{BorshDeserialize, BorshSerialize};
use std::ops::DerefMut;

#[allow(unused_imports)]
use std::mem::transmute;
declare_id!("VWSNCrHtp2LJn3R5Uogy14HSwLCMhrNt61ze63AKowr");

const MAX_TRANSACTION: usize = 5000;
const RECOVER_SIZE: usize = 1000;
const TRANSACTION_TEMPLATE: ForwardTransaction = ForwardTransaction {
    seller_name: [0; 20],
    seller_birth_day: [0; 20],
    seller_address: [0; 150],
    seller_phone: [0; 20],
    seller_sub_phone: [0; 20],
    buyer_name: [0; 20],
    buyer_birth_day: [0; 20],
    buyer_address: [0; 150],
    buyer_phone: [0; 20],
    buyer_sub_phone: [0; 20],
    item: [0; 20],
    kind: [0; 20],
    formal_day: [0; 20],
    area_flat_unit: [0; 20],
    address: [0; 150],
    option: [0; 20],
    flat_price: [0; 20],
    contract_price: [0; 20],
    first_yn: [0; 20],
    first_price: [0; 20],
    first_end_count: [0; 20],
    second_yn: [0; 20],
    second_price: [0; 20],
    second_end_count: [0; 20],
    third_yn: [0; 20],
    third_price: [0; 20],
    third_end_count: [0; 20],
    return_date: [0; 20]
};

#[program]
pub mod smart_contract {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let mut base_account = ctx.accounts.my_account.load_init()?;
        base_account.counter = 0;

        Ok(())
    }
    pub fn insert(ctx: Context<Insert>, data: TransactionInstruction) -> ProgramResult {
        let mut base_account = ctx.accounts.my_account.load_mut()?;
        let account_data = base_account.deref_mut();

        let instruction_data = parse_instruction_data(data);

        account_data.append(instruction_data);

        Ok(())
    }

    pub fn insert_by_index(ctx: Context<InsertByIndex>, data: TransactionInstruction , index: i32) -> ProgramResult {
        let mut base_account = ctx.accounts.my_account.load_mut()?;
        let account_data = base_account.deref_mut();

        let instruction_data = parse_instruction_data(data);

        account_data.insert(index as usize, instruction_data);

        Ok(())
    }

    pub fn remove(ctx: Context<Remove>, index: i32) -> ProgramResult {
        let mut base_account = ctx.accounts.my_account.load_mut()?;
        let account_data = base_account.deref_mut();

        account_data.delete(index as usize);

        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: TransactionInstruction, index: i32) -> ProgramResult {
        let mut base_account = ctx.accounts.my_account.load_mut()?;
        let account_data = base_account.deref_mut();

        let instruction_data = parse_instruction_data(data);

        account_data.update(index as usize, instruction_data);

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(zero)]
    pub my_account: AccountLoader<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Insert<'info> {
    #[account(mut)]
    pub my_account: AccountLoader<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct InsertByIndex<'info> {
    #[account(mut)]
    pub my_account: AccountLoader<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct Remove<'info> {
    #[account(mut)]
    pub my_account: AccountLoader<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: AccountLoader<'info, MyAccount>,
}
#[derive(Debug)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransactionInstruction {
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
    pub option: String,
    pub flat_price: String,
    pub contract_price: String,
    pub first_yn: String,
    pub first_price: String,
    pub first_end_count: String,
    pub second_yn: String,
    pub second_price: String,
    pub second_end_count: String,
    pub third_yn: String,
    pub third_price: String,
    pub third_end_count: String,
    pub return_date: String
}

#[derive(Debug, PartialEq)]
#[zero_copy]
pub struct ForwardTransaction {
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
    pub option: [u8; 20],
    pub flat_price: [u8; 20],
    pub contract_price: [u8; 20],
    pub first_yn: [u8; 20],
    pub first_price: [u8; 20],
    pub first_end_count: [u8; 20],
    pub second_yn: [u8; 20],
    pub second_price: [u8; 20],
    pub second_end_count: [u8; 20],
    pub third_yn: [u8; 20],
    pub third_price: [u8; 20],
    pub third_end_count: [u8; 20],
    pub return_date: [u8; 20]
}

#[account(zero_copy)]
pub struct MyAccount {
    pub counter: i32,
    pub deep: i32,
    pub transaction_ls: [ForwardTransaction; MAX_TRANSACTION]
}

// append can only be done in index [0, 8999]
// delete can only be done in index [0, 9999]
// insert can only be done in index [9000, 9999]
impl MyAccount {
    fn append(&mut self, data: ForwardTransaction) {
        if self.counter > self.deep {
            msg!("Error: counter is bigger than deep.");
        } else if self.counter == self.deep {
            msg!("# counter == deep, Insert data in {}", self.counter);
            self.transaction_ls[self.counter as usize] = data;
            self.deep += 1;
        } else {
            for i in 0..self.deep as usize {
                // if self.transaction_ls[i] eqaul to TRANSACTION_TEMPLATE, replace it with data.
                if self.transaction_ls[i] == TRANSACTION_TEMPLATE {
                    msg!("# counter < deep, Insert data in {}", i);
                    self.transaction_ls[i] = data;
                    break;
                }
            }
        }
        
        self.counter += 1;
    }

    // After popping, counter must be decreased by 1.
    // There is no reason for the counter bigger than deep.
    // If the counter is equal to deep, it means there is no gap in stored data.
    // If the counter is smaller than deep, it means there is a gap in stored data, so we need to replace the gap with the default template.
    fn delete(&mut self, index: usize) {
        if index > self.deep as usize {
            msg!("Error: Index is bigger than deep.");
        } else if index == self.deep as usize {
            msg!("Index is equal to deep.");
            self.transaction_ls[index] = TRANSACTION_TEMPLATE;
            self.deep -= 1;
        } else {
            msg!("Index is less than deep.");
            self.transaction_ls[index] = TRANSACTION_TEMPLATE;
        }
        self.counter -= 1;

        msg!("After deleting, counter is {}, deep is {}", self.counter, self.deep);
    }

    fn insert(&mut self, index: usize, data: ForwardTransaction) {
        // if index is equal or bigger than 10000, print index out of range.
        // if index is smaller than 10000 and equal to or bigger than 8999, insert data into transaction_ls[index].
        // others, print this is not a valid index.

        if index >= MAX_TRANSACTION {
            msg!("Error: Index is out of range.");
        } else if index >= MAX_TRANSACTION - RECOVER_SIZE as usize {
            msg!("Index is equal to or bigger than {} and less than {}.", MAX_TRANSACTION - RECOVER_SIZE, MAX_TRANSACTION);
            self.transaction_ls[index] = data;
        } else {
            msg!("Index is less than {}.", MAX_TRANSACTION - RECOVER_SIZE);
        }
    }

    fn update(&mut self, index: usize, data: ForwardTransaction) {
        if index >= MAX_TRANSACTION {
            msg!("Error: Index is out of range.");
        } else {
            self.transaction_ls[index] = data;
        }
    }
}

fn parse_instruction_data(data: TransactionInstruction) -> ForwardTransaction {
    ForwardTransaction {
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
        option: convert_string_to_20bytes(data.option),
        flat_price: convert_string_to_20bytes(data.flat_price),
        contract_price: convert_string_to_20bytes(data.contract_price),
        first_yn: convert_string_to_20bytes(data.first_yn),
        first_price: convert_string_to_20bytes(data.first_price),
        first_end_count: convert_string_to_20bytes(data.first_end_count),
        second_yn: convert_string_to_20bytes(data.second_yn),
        second_price: convert_string_to_20bytes(data.second_price),
        second_end_count: convert_string_to_20bytes(data.second_end_count),
        third_yn: convert_string_to_20bytes(data.third_yn),
        third_price: convert_string_to_20bytes(data.third_price),
        third_end_count: convert_string_to_20bytes(data.third_end_count),
        return_date: convert_string_to_20bytes(data.return_date),
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