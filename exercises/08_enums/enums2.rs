// ===== THE CHICKEN COMES FIRST =====
// Point struct must be defined BEFORE Message enum, because:
// - The Message::Move(Point) variant USES the Point type
// - Rust needs to know what Point is when it compiles the enum definition
// - You can't reference a type that doesn't exist yet
#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

// ===== THEN THE EGG (THE ENUM) =====
// Now that Point exists, we can use it in enum variants
#[derive(Debug)]
enum Message {
    // Enum variants represent different message types the system can send:
    Resize { width: u64, height: u64 },  // Named fields: resize with width and height
    Move(Point),                          // Tuple variant: contains a Point struct
    Echo(String),                         // Tuple variant: contains a String
    ChangeColor(u8, u8, u8),             // Tuple variant: three color bytes (R, G, B)
    Quit,                                 // Unit variant: no associated data
}

// ===== IMPLEMENT METHODS ON THE ENUM =====
// impl blocks define what messages can DO (their behavior)
impl Message {
    // The call() method: print the debug representation of any Message variant
    // This demonstrates pattern matching on enums—it handles all 5 variants
    fn call(&self) {
        println!("{self:?}");  // Uses #[derive(Debug)] to print the enum
    }
}

fn main() {
    // ===== CREATE AN ARRAY OF DIFFERENT MESSAGE VARIANTS =====
    // This array shows all 5 different ways to construct Message:
    // - Named fields: Resize { ... }
    // - Tuple with Point: Move(Point { ... })
    // - Tuple with String: Echo(String::from(...))
    // - Tuple with three u8s: ChangeColor(...)
    // - Unit variant: Quit (no data)
    let messages = [
        // Variant 1: Resize with named fields (width, height)
        Message::Resize {
            width: 10,
            height: 30,
        },
        // Variant 2: Move containing a Point struct (demonstrates nested types)
        // This only works because Point is defined BEFORE Message
        Message::Move(Point { x: 10, y: 15 }),
        // Variant 3: Echo containing a String
        Message::Echo(String::from("hello world")),
        // Variant 4: ChangeColor containing three u8 color values (R, G, B)
        Message::ChangeColor(200, 255, 255),
        // Variant 5: Quit with no associated data
        Message::Quit,
    ];

    // ===== ITERATE AND CALL METHOD ON EACH MESSAGE =====
    // Loop through each message and call its method
    // The &messages borrow the array without taking ownership
    for message in &messages {
        message.call();  // This prints the debug output for each variant
    }
}
