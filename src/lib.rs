use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub mod instructions;
pub mod proccessor;
pub mod state;
pub mod test;
use instructions::store_instruction::StoreInstructions;

use crate::proccessor::Proccessor;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    println!("{:?}", instruction_data);
    // Unpack called
    println!("Unpack2");
    let instruction = StoreInstructions::unpack(instruction_data)?;
    println!("Unpack3");
    Proccessor::new(program_id, accounts, instruction)
}
