#[derive(Debug, PartialEq)]
pub enum PlayerAction {
    Call,
    Raise(u32),
    Check,
    Fold,
}
