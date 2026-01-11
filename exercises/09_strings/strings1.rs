// ===== STRING TYPES: THE CHICKEN AND EGG AGAIN =====
// Rust has TWO string types with different purposes:
//
// 1. &str (string slice):
//    - Type: "blue"
//    - Fixed, immutable reference to string data
//    - Stored in read-only memory (compiled into the binary)
//    - Cannot grow or change size
//    - Lifetime is tied to the data it references
//
// 2. String (owned string):
//    - Type: String::from("blue") or "blue".to_string()
//    - Dynamic, owned, heap-allocated
//    - Can grow, shrink, and be modified (with mut)
//    - Must be freed when dropped (ownership rules apply)
//    - Flexible, but more expensive than &str
//
// The function signature says:
fn current_favorite_color() -> String {
    // ===== WHY .to_string() IS REQUIRED =====
    // "blue" is a &str (string literal)
    // The return type is String (owned)
    // Rust cannot automatically convert &str to String (different types!)
    //
    // .to_string() does the conversion:
    // - Takes the &str "blue"
    // - Allocates new memory on the heap
    // - Copies "blue" into that memory
    // - Returns a String that owns that memory
    //
    // This is NECESSARY because ownership rules require that the function
    // return an owned value (the caller becomes responsible for it).
    // A &str reference would dangle (the function would go out of scope).
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
