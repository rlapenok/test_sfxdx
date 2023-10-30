use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]

pub struct Payload {
    pub amount: f64,
    pub bump_seed: Option<u8>,
}
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub enum StoreInstructions {
    InitStore(Payload),
    ChangePrice(Payload),
    By(Payload),
    Sell(Payload),
    CreateToken,
}

impl StoreInstructions {
    pub fn unpack(instruction: &[u8]) -> Result<Self, ProgramError> {
        let row = instruction.split_at(4);
        let inst = StoreInstructions::try_from_slice(row.1)?;
        match inst.clone() {
            Self::InitStore(_) => Ok(inst),
            Self::ChangePrice(_) => Ok(inst),
            _ => {
                todo!()
            }
        }
    }
    pub fn init_store(
        payload: Payload,
        program_id: Pubkey,
        init: Pubkey,
        pda_key: Pubkey,
    ) -> Instruction {
        let instr = borsh::to_vec(&StoreInstructions::InitStore(payload)).unwrap();
        Instruction::new_with_borsh(
            program_id,
            &instr,
            vec![
                AccountMeta::new(init, true),
                AccountMeta::new(pda_key, false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        )
    }
    pub fn update_store(
        payload: Payload,
        program_id: Pubkey,
        init: Pubkey,
        pda_key: Pubkey,
    ) -> Instruction {
        let instr = borsh::to_vec(&StoreInstructions::ChangePrice(payload)).unwrap();
        Instruction::new_with_borsh(
            program_id,
            &instr,
            vec![
                AccountMeta::new(init, true),
                AccountMeta::new(pda_key, false),
            ],
        )
    }
    pub fn create_toke() {}
}
