// Zero crate colour library
#![warn(clippy::pedantic, clippy::all)]
use std::fmt;

// Foreground colours
pub const FG_BLACK: &str = "[38;5;0m";
pub const FG_RED: &str = "[38;5;1m";
pub const FG_GREEN: &str = "[38;5;2m";
pub const FG_YELLOW: &str = "[38;5;3m";
pub const FG_BLUE: &str = "[38;5;4m";
pub const FG_PURPLE: &str = "[38;5;5m";
pub const FG_CYAN: &str = "[38;5;6m";
pub const FG_WHITE: &str = "[38;5;7m";
pub const FG_LIGHTBLACK: &str = "[38;5;8m";
pub const FG_LIGHTRED: &str = "[38;5;9m";
pub const FG_LIGHTGREEN: &str = "[38;5;10m";
pub const FG_LIGHTYELLOW: &str = "[38;5;11m";
pub const FG_LIGHTBLUE: &str = "[38;5;12m";
pub const FG_LIGHTPURPLE: &str = "[38;5;13m";
pub const FG_LIGHTCYAN: &str = "[38;5;14m";
pub const FG_LIGHTWHITE: &str = "[38;5;15m";

// Background colours
pub const BG_BLACK: &str = "[48;5;0m";
pub const BG_RED: &str = "[48;5;1m";
pub const BG_GREEN: &str = "[48;5;2m";
pub const BG_YELLOW: &str = "[48;5;3m";
pub const BG_BLUE: &str = "[48;5;4m";
pub const BG_PURPLE: &str = "[48;5;5m";
pub const BG_CYAN: &str = "[48;5;6m";
pub const BG_WHITE: &str = "[48;5;7m";
pub const BG_LIGHTBLACK: &str = "[48;5;8m";
pub const BG_LIGHTRED: &str = "[48;5;9m";
pub const BG_LIGHTGREEN: &str = "[48;5;10m";
pub const BG_LIGHTYELLOW: &str = "[48;5;11m";
pub const BG_LIGHTBLUE: &str = "[48;5;12m";
pub const BG_LIGHTPURPLE: &str = "[48;5;13m";
pub const BG_LIGHTCYAN: &str = "[48;5;14m";
pub const BG_LIGHTWHITE: &str = "[48;5;15m";

// Resetting of colours
pub const FG_RESET: &str = "[39m";
pub const BG_RESET: &str = "[49m";

// Text styles
pub const BOLD: &str = "[1m";
pub const BOLD_RESET: &str = "[22m";
pub const UNDERLINE: &str = "[4m";
pub const UNDERLINE_RESET: &str = "[24m";
pub const STRIKE: &str = "[9m";
pub const STRIKE_RESET: &str = "[29m";
pub const ITALIC: &str = "[3m";
pub const ITALIC_RESET: &str = "[23m";
pub const INVERSE: &str = "[7m";
pub const INVERSE_RESET: &str = "[27m";
pub const FAINT: &str = "[2m";
pub const FAINT_RESET: &str = "[22m";

// Resetting of everything
pub const RESET: &str = "[m";

/// Foreground colours for setting text colour
#[derive(Debug, Clone, Copy)]
pub enum Fg {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightPurple,
    LightCyan,
    LightWhite,
    Reset,
}

// Allow use in format macros
impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Fg::Black => FG_BLACK,
                Fg::Red => FG_RED,
                Fg::Green => FG_GREEN,
                Fg::Yellow => FG_YELLOW,
                Fg::Blue => FG_BLUE,
                Fg::Purple => FG_PURPLE,
                Fg::Cyan => FG_CYAN,
                Fg::White => FG_WHITE,
                Fg::LightBlack => FG_LIGHTBLACK,
                Fg::LightRed => FG_LIGHTRED,
                Fg::LightGreen => FG_LIGHTGREEN,
                Fg::LightYellow => FG_LIGHTYELLOW,
                Fg::LightBlue => FG_LIGHTBLUE,
                Fg::LightPurple => FG_LIGHTPURPLE,
                Fg::LightCyan => FG_LIGHTCYAN,
                Fg::LightWhite => FG_LIGHTWHITE,
                Fg::Reset => FG_RESET,
            }
        )
    }
}

/// Background colours for setting text background colour
#[derive(Debug, Clone, Copy)]
pub enum Bg {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightPurple,
    LightCyan,
    LightWhite,
    Reset,
}

// Allow use in format macros
impl fmt::Display for Bg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Bg::Black => BG_BLACK,
                Bg::Red => BG_RED,
                Bg::Green => BG_GREEN,
                Bg::Yellow => BG_YELLOW,
                Bg::Blue => BG_BLUE,
                Bg::Purple => BG_PURPLE,
                Bg::Cyan => BG_CYAN,
                Bg::White => BG_WHITE,
                Bg::LightBlack => BG_LIGHTBLACK,
                Bg::LightRed => BG_LIGHTRED,
                Bg::LightGreen => BG_LIGHTGREEN,
                Bg::LightYellow => BG_LIGHTYELLOW,
                Bg::LightBlue => BG_LIGHTBLUE,
                Bg::LightPurple => BG_LIGHTPURPLE,
                Bg::LightCyan => BG_LIGHTCYAN,
                Bg::LightWhite => BG_LIGHTWHITE,
                Bg::Reset => BG_RESET,
            }
        )
    }
}

/// Style enum to style text
#[derive(Debug, Clone, Copy)]
pub enum Style {
    Bold,
    NoBold,
    Underline,
    NoUnderline,
    Strike,
    NoStrike,
    Italic,
    NoItalic,
    Inverse,
    NoInverse,
    Faint,
    NoFaint,
}

// Allow use in format macros
impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Style::Bold => BOLD,
                Style::NoBold => BOLD_RESET,
                Style::Underline => UNDERLINE,
                Style::NoUnderline => UNDERLINE_RESET,
                Style::Strike => STRIKE,
                Style::NoStrike => STRIKE_RESET,
                Style::Italic => ITALIC,
                Style::NoItalic => ITALIC_RESET,
                Style::Inverse => INVERSE,
                Style::NoInverse => INVERSE_RESET,
                Style::Faint => FAINT,
                Style::NoFaint => FAINT_RESET,
            }
        )
    }
}

/// A reset type that clears all styling at once
#[derive(Debug, Clone, Copy)]
pub struct Reset;

// Allow use in format macros
impl fmt::Display for Reset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", RESET)
    }
}
