#[derive(Debug, PartialEq, Eq)]
pub enum WesternDateError {
    // the Gregorian calendar came in in 1583
    InvalidPrior1583,
}
