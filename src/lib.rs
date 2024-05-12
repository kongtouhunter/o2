pub mod consts;
pub mod error;
pub mod instruction;
mod loaders;
mod processor;
pub mod state;
pub mod utils;

pub use consts::*;
use instruction::*;
use processor::*;
use solana_program::{
    self, account_info::AccountInfo, declare_id, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey,
};

// TODO Initialize with mining paused
// TODO Require hardcoded admin key for initialization

declare_id!("mineS8xKxv3GurPq4RAssdZa6kNSXuuxJCEVtQwPZX4");

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::id()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (tag, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match OreInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))? {
        OreInstruction::Register => process_register(program_id, accounts, data)?,
        OreInstruction::Deregister => process_deregister(program_id, accounts, data)?,
        OreInstruction::Reset => process_reset(program_id, accounts, data)?,
        OreInstruction::Mine => process_mine(program_id, accounts, data)?,
        OreInstruction::Claim => process_claim(program_id, accounts, data)?,
        OreInstruction::Stake => process_stake(program_id, accounts, data)?,
        OreInstruction::Upgrade => process_upgrade(program_id, accounts, data)?,
        OreInstruction::Initialize => process_initialize(program_id, accounts, data)?,
        OreInstruction::UpdateAdmin => process_update_admin(program_id, accounts, data)?,
        OreInstruction::UpdateTolerance => process_update_tolerance(program_id, accounts, data)?,
        OreInstruction::Pause => process_pause(program_id, accounts, data)?,
    }

    Ok(())
}
