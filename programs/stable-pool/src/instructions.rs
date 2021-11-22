use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount,Mint};


use crate::{
    states::*,
    constant::*,
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct SendSolAtTime <'info>{
    pub sender:  Signer<'info>,

    pub sender_wsol_token: Account<'info, TokenAccount>,
    pub receiver_wsol_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,

}
