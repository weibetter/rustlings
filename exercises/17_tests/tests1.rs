// Tests are important to ensure that your code does what you think it should
// do.

// Function: is_even
// Returns: bool
// - true if n is even (divisible by 2)
// - false if n is odd (not divisible by 2)
// Logic: n % 2 == 0 returns true when remainder is 0 (even), false otherwise
fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.

    use crate::is_even;

    #[test]
    fn you_can_assert() {
        // Test the function `is_even` with some values.
        // assert!(condition) passes if condition is true, fails if false
        assert!(is_even(2));      // Return: true (2 is even) → assertion passes
        assert!(!is_even(3));     // Return: false (3 is odd) → ! negates to true → assertion passes
    }
}
