#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Call,
    Raise(u32),
    Check,
    Fold,
}
