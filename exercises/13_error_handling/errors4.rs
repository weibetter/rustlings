use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // cmp() is the "compare" method from std::cmp::Ord trait
        // value.cmp(&0) compares value against 0 and returns an Ordering enum
        // Ordering has three variants:
        // - Ordering::Less: value < 0 (negative)
        // - Ordering::Equal: value == 0 (zero)
        // - Ordering::Greater: value > 0 (positive)
        match value.cmp(&0) {
            // If value is negative, return error
            Ordering::Less => Err(CreationError::Negative),
            // If value is zero, return error
            Ordering::Equal => Err(CreationError::Zero),
            // If value is positive, create PositiveNonzeroInteger and wrap in Ok
            // Cast i64 to u64 since we confirmed value > 0
            Ordering::Greater => Ok(Self(value as u64)),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
