fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x; // Cannot borrow `x` as mutable more than once at a time
        y.push(42); 
        
        // Borrow checker should allow this after reordering
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
