use thiserror::Error;
#[derive(Error, Debug, PartialEq, Eq)]
pub enum PlayerError {
    #[error("Not enough chips for blind!")]
    NotEnoughForBlind,
    #[error("Cannot check if there is a bet")]
    CheckActionOnBet,
}
