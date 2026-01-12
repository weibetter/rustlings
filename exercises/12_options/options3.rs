#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Create an Option containing a Point struct with x=100, y=200
    let optional_point = Some(Point { x: 100, y: 200 });

    // Pattern match on the Option value
    match optional_point {
        // ARM 1: If optional_point is Some, bind it to p using a reference
        // `ref p` borrows the Point instead of moving it (doesn't consume optional_point)
        // This is required because println!("{optional_point:?}") below needs the value
        Some(ref p) => println!("Coordinates are {},{}", p.x, p.y),
        // ARM 2: Catch-all pattern for any other case (like None)
        // Panics if this arm is reached
        _ => panic!("No match!"),
    }

    // Print the entire optional_point using Debug formatting
    // This works because `ref p` in the match did NOT move optional_point
    // If we had used Some(p) instead, this line would fail (value moved)
    println!("{optional_point:?}"); // Don't change this line.
}
