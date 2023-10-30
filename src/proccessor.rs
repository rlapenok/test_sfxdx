use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use spl_token::state::Mint;

use crate::{
    instructions::store_instruction::{Payload, StoreInstructions},
    state::store_state::{Store, SEED},
};

trait ProccessorInstruction {
    fn init_store(program_id: &Pubkey, accounts: &[AccountInfo], payload: Payload)
        -> ProgramResult;
    fn update_store(
        program_id: &Pubkey,
        account: &[AccountInfo],
        payload: Payload,
    ) -> ProgramResult;
}

pub struct Proccessor;

impl Proccessor {
    pub fn new(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction: StoreInstructions,
    ) -> ProgramResult {
        match instruction {
            StoreInstructions::InitStore(payload) => {
                return <Proccessor as ProccessorInstruction>::init_store(
                    program_id, accounts, payload,
                )
            }
            StoreInstructions::ChangePrice(payload) => {
                return <Proccessor as ProccessorInstruction>::update_store(
                    program_id, accounts, payload,
                );
            }
            _ => {
                todo!()
            }
        }
    }
}

impl ProccessorInstruction for Proccessor {
    fn init_store(program_id: &Pubkey, account: &[AccountInfo], payload: Payload) -> ProgramResult {
        let iterator = &mut account.iter();
        let admin = next_account_info(iterator)?;
        let pda_account = next_account_info(iterator)?;
        let system_program = next_account_info(iterator)?;
        let pda_pubkey = pda_account.key;
        let admin_pubkey = admin.key.to_bytes();

        //Проверка, что аккаунт, который может создавать store подписал транзакцию
        if !admin.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        } else {
            //Создаем новый Store
            let store = Store::new(payload.amount, admin_pubkey);
            let mut writer = Vec::new();
            store.serialize(&mut writer)?;
            //Сколько места будет занимать данные
            let rent = Rent::default();
            let rent_lamp = rent.minimum_balance(writer.len());
            let space = writer.len();
            //Инструкция для создания аккаунта
            let instr = system_instruction::create_account(
                admin.key,
                pda_pubkey,
                rent_lamp,
                space as u64,
                program_id,
            );
            let signer_seeds: &[&[_]] = &[SEED.as_bytes(), &[payload.bump_seed.unwrap()]];
            let account_info = &[admin.clone(), pda_account.clone(), system_program.clone()];
            invoke_signed(&instr, account_info, &[&signer_seeds])?;
            let mut pda_account_data = Store::try_from_slice(&pda_account.data.borrow()).unwrap();
            pda_account_data = store;
            pda_account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
        }
        Ok(())
    }
    fn update_store(
        program_id: &Pubkey,
        account: &[AccountInfo],
        payload: Payload,
    ) -> ProgramResult {
        let iterator = &mut account.iter();
        let init_acc = next_account_info(iterator)?;
        let pda_account = next_account_info(iterator)?;
        if !init_acc.is_signer {}
        if program_id != pda_account.owner {
            return Err(ProgramError::IllegalOwner);
        }
        let mut pda_account_data = Store::try_from_slice(&pda_account.data.borrow()).unwrap();
        if pda_account_data.init != init_acc.key.to_bytes() {}
        pda_account_data.update(payload.amount);
        pda_account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
        Ok(())
    }
}
