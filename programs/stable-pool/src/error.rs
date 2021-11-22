use anchor_lang::prelude::*;

#[error]
pub enum SendError {
    #[msg("Too soon")]
    TooSoonError,
}