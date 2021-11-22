use anchor_lang::prelude::*;

///processor
pub mod processor;
/// error
pub mod error;
/// instructions
pub mod instructions;

use crate::{
    instructions::*,
    processor::*,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod stable_pool {
    use super::*;

    pub fn send_sol_at_time(ctx: Context<SendSolAtTime>, amount: u64, time: u64) -> ProgramResult {process_send_sol_at_time(ctx, amount, time) }

}