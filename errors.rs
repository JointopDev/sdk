use anchor_lang::prelude::*;

#[error_code]
pub enum JointopError {
    #[msg("Nothing to claim")]
    NothingToClaim,
}
