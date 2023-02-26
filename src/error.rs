#[derive(Debug)]
pub enum Error {
    LockPoisoned,
    ThereIsNoWord,
    WordAlreadyExists,
}
