use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Transfer, ID};

use crate::{
    instructions::*,
    error::*
};

pub fn process_send_sol_at_time(ctx: Context<SendSolAtTime>, amount: u64,time: u64) -> ProgramResult {

    let cur_timestamp = ctx.accounts.clock.unix_timestamp as u64;
    if cur_timestamp < time {
        return Err(SendError::TooSoonError.into());
    }

    // transfer from user to pool
    let cpi_accounts = Transfer {
        from: ctx.accounts.sender_wsol_token.to_account_info().clone(),
        to: ctx.accounts.receiver_wsol_token.to_account_info().clone(),
        authority: ctx.accounts.sender.to_account_info().clone(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info().clone();
    
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, amount)?;
    Ok(())
}
