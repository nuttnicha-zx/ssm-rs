use std::fmt::{Display, Formatter, Result};

/// Clear terminal screen
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

/// Text Color for terminal output
pub enum TextColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    Reset,
}

/// Set the text color in the terminal
impl Display for TextColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let color_code = match self {
            TextColor::Red => "\x1B[31m",
            TextColor::Green => "\x1B[32m",
            TextColor::Yellow => "\x1B[33m",
            TextColor::Blue => "\x1B[34m",
            TextColor::Magenta => "\x1B[35m",
            TextColor::Cyan => "\x1B[36m",
            TextColor::White => "\x1B[37m",
            TextColor::Gray => "\x1B[90m",
            TextColor::Reset => "\x1B[0m",
        };
        write!(f, "{}", color_code)
    }
}
