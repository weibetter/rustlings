struct Point {
    x: u64,
    y: u64,
}

enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {
        // ===== MATCH EXPRESSION: THE SEQUENCE =====
        // A match statement does the following IN ORDER:
        // 1. Evaluates the expression (in this case: the message parameter)
        // 2. Compares it against each arm (pattern) from top to bottom
        // 3. Stops at the FIRST arm that matches
        // 4. Executes the code for that arm
        // 5. Returns the result (if the match returns a value)
        //
        // Key rule: Only ONE arm executes. Once a match is found, all other
        // arms are skipped (no fall-through like switch statements in C/Java)
        //
        // Pattern matching: Each Message variant has a unique shape, so Rust
        // can tell them apart and extract the associated data automatically.
        
        match message {
            // ARM 1: Message::Resize with named fields
            // Pattern: Matches the Resize variant and extracts width and height
            // Extraction: { width, height } unpacks the named fields
            // Execution: Calls resize() with the extracted values
            // This arm only matches if message is Resize { ... }
            Message::Resize { width, height } => self.resize(width, height),
            
            // ARM 2: Message::Move containing a Point
            // Pattern: Matches the Move variant and extracts the Point struct
            // Extraction: (point) unpacks the tuple—point is now the Point struct
            // Execution: Calls move_position() with the Point
            // This arm only matches if message is Move(Point { ... })
            Message::Move(point) => self.move_position(point),
            
            // ARM 3: Message::Echo containing a String
            // Pattern: Matches the Echo variant and extracts the String
            // Extraction: (s) unpacks the tuple—s is now the String value
            // Execution: Calls echo() with the String
            // This arm only matches if message is Echo(String)
            Message::Echo(s) => self.echo(s),
            
            // ARM 4: Message::ChangeColor containing three u8 values
            // Pattern: Matches the ChangeColor variant and extracts the three bytes
            // Extraction: (r, g, b) unpacks the tuple—r, g, b are the color values
            // Execution: Calls change_color() with the three color components
            // This arm only matches if message is ChangeColor(u8, u8, u8)
            Message::ChangeColor(r, g, b) => self.change_color(r, g, b),
            
            // ARM 5: Message::Quit
            // Pattern: Matches the Quit variant (no data to extract)
            // Extraction: Nothing—Quit is a unit variant with no associated data
            // Execution: Calls quit() with no arguments
            // This arm only matches if message is Quit
            Message::Quit => self.quit(),
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
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        // call process() with different Message variants
        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
