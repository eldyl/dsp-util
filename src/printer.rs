const ANSI_RESET: &str = "\x1b[0m"; // ANSI reset code

/// Color options for printing to the terminal
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
}

/// Implement Color to match on proper ANSI code
impl Color {
    /// Get ANSI code for color
    fn code(&self) -> &str {
        match self {
            Color::Red => "\x1b[1;31m",
            Color::Green => "\x1b[1;32m",
            Color::Blue => "\x1b[1;34m",
            Color::Yellow => "\x1b[1;33m",
            Color::Magenta => "\x1b[1;35m",
            Color::Cyan => "\x1b[1;36m",
            Color::White => "\x1b[1;37m",
        }
    }
}

/// Print line function that uses ANSI code to display colored text on terminal
pub fn color_println(color: Color, text: &str) {
    println!("{}{}{}", color.code(), text, ANSI_RESET);
}

/// Format string function that uses ANSI code to return string formatted for color
pub fn color_println_fmt(color: Color, text: &str) -> String {
    format!("{}{}{}", color.code(), text, ANSI_RESET)
}
