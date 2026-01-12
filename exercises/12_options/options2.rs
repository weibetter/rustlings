fn main() {
    // Entry point for the program. No code here; all logic is in tests.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        // Define a string slice with value "rustlings"
        let target = "rustlings";
        // Wrap the string slice in an Option, making it Some(&str)
        let optional_target = Some(target);

        // Pattern match: if optional_target is Some, bind the value to word
        // This checks if the Option contains a value and extracts it
        if let Some(word) = optional_target {
            // Assert that the extracted word equals the original target
            assert_eq!(word, target);
        }
        // If optional_target was None, the body would not execute
    }

    #[test]
    fn layered_option() {
        // Set the upper bound for the range
        let range = 10;
        // Create a vector of Option<i8>, starting with a single None value
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        // Loop from 1 to range (inclusive), pushing Some(i) into the vector
        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        // Initialize cursor to the highest value in the range
        let mut cursor = range;

        // while-let pattern: repeatedly pop from the vector
        // Vec::pop() returns Option<Option<i8>>
        // Outer Some: pop returned a value (not empty)
        // Inner Some(integer): the value is Some(i), not None
        while let Some(Some(integer)) = optional_integers.pop() {
            // Assert that the integer matches the expected cursor value
            assert_eq!(integer, cursor);
            // Decrement cursor for the next expected value
            cursor -= 1;
        }
        // Loop ends when pop returns None or Some(None)

        // After the loop, cursor should be 0 (all values matched)
        assert_eq!(cursor, 0);
    }
}
