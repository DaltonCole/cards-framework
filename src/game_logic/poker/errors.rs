use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PlayerError {
    #[error("Cannot check if there is a bet!")]
    CheckActionOnBet,
}
