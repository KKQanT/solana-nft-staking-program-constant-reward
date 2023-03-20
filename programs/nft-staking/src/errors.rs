use anchor_lang::prelude::*;

#[error_code]
pub enum OGAStakingError {
  #[msg("unknown error")]
  UnknownError,
}