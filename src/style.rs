use crate::terminal_size;
use std::fmt::Write;

pub use crossterm::style::Color;

/// Change how the text is displayed to the user.
///
/// ```no_run
/// # use sketch::Style;
/// const FOCUS_STYLE: Style = Style::new().red().bold();
/// let text = FOCUS_STYLE.render("[ Submit]");
/// ```
#[derive(Debug, Default, Clone)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    underline_color: Option<Color>,
    blink: Option<Blink>,
    reverse: bool,
    crossed_out: bool,
    align: Align,
}

/// The speed of text blinking for [`Style::blink`].
#[derive(Debug, Clone)]
pub enum Blink {
    /// Less than 150 times per minute.
    Slow,
    /// Greater than 150 times per minute.
    Rapid,
}

/// Alignment options for text.
#[derive(Debug, Default, Clone)]
pub enum Align {
    /// Align text left.
    #[default]
    Left,
    /// Align text center.
    Center,
    /// Align text right.
    Right,
}

macro_rules! style_method {
    ($method:ident, fg, $value:expr) => {
        #[doc = concat!("Set the text color to [`", stringify!($value), "`].")]
        pub const fn $method(mut self) -> Self {
            self.fg = Some($value);
            self
        }
    };
    ($method:ident, bg, $value:expr) => {
        #[doc = concat!("Set the background color to [`", stringify!($value), "`].")]
        pub const fn $method(mut self) -> Self {
            self.fg = Some($value);
            self
        }
    };
    ($method:ident, underline_color, $value:expr) => {
        #[doc = concat!("Set the underline color to [`", stringify!($value), "`].")]
        pub const fn $method(mut self) -> Self {
            self.underline_color = Some($value);
            self
        }
    };
    ($method:ident, $field:ident, $value:expr, $doc:literal) => {
        #[doc = $doc]
        pub const fn $method(mut self) -> Self {
            self.$field = $value;
            self
        }
    };
}

impl Style {
    /// Create a new style with default values.
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            underline_color: None,
            blink: None,
            reverse: false,
            crossed_out: false,
            align: Align::Left,
        }
    }

    /// Set the color of the text.
    pub const fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set the color of the background.
    pub const fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set the color of the underline.
    pub const fn underline_color(mut self, color: Color) -> Self {
        self.underline = true;
        self.underline_color = Some(color);
        self
    }

    /// Enable blinking and set its speed.
    ///
    /// See [`Style::slow_blink`] and [`Style::rapid_blink`] for shorthands.
    pub const fn blink(mut self, blink: Blink) -> Self {
        self.blink = Some(blink);
        self
    }

    /// Set the alignment of the text.
    ///
    /// See [`Style::left`], [`Style::center`] and [`Style::right`] for shorthands.
    pub const fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    style_method! { left, align, Align::Left, "Align the text to the left." }
    style_method! { center, align, Align::Center, "Align the text in the center." }
    style_method! { right, align, Align::Right, "Align the text to the right." }

    // Modifiers
    style_method! { bold, bold, true, "Make the text bold." }
    style_method! { dim, dim, true, "Make the text dim." }
    style_method! { italic, italic, true, "Make the text italic." }
    style_method! { underline, underline, true, "Underline the text." }
    style_method! { slow_blink, blink, Some(Blink::Slow), "Blink the text slowly." }
    style_method! { rapid_blink, blink, Some(Blink::Rapid), "Blick the text rapidly." }
    style_method! { reverse, reverse, true, "Spawn the text and background colors." }
    style_method! { crossed_out, crossed_out, true, "Cross the text." }

    // Forground/Text Colors
    style_method! { black, fg, Color::Black }
    style_method! { dark_grey, fg, Color::DarkGrey }
    style_method! { red, fg, Color::Red }
    style_method! { dark_red, fg, Color::DarkRed }
    style_method! { green, fg, Color::Green }
    style_method! { dark_green, fg, Color::DarkGreen }
    style_method! { yellow, fg, Color::Yellow }
    style_method! { dark_yellow, fg, Color::DarkYellow }
    style_method! { blue, fg, Color::Blue }
    style_method! { dark_blue, fg, Color::DarkBlue }
    style_method! { magenta, fg, Color::Magenta }
    style_method! { dark_magenta, fg, Color::DarkMagenta }
    style_method! { cyan, fg, Color::Cyan }
    style_method! { dark_cyan, fg, Color::DarkCyan }
    style_method! { white, fg, Color::White }
    style_method! { grey, fg, Color::Grey }

    // Background Colors
    style_method! { on_black, bg, Color::Black }
    style_method! { on_dark_grey, bg, Color::DarkGrey }
    style_method! { on_red, bg, Color::Red }
    style_method! { on_dark_red, bg, Color::DarkRed }
    style_method! { on_green, bg, Color::Green }
    style_method! { on_dark_green, bg, Color::DarkGreen }
    style_method! { on_yellow, bg, Color::Yellow }
    style_method! { on_dark_yellow, bg, Color::DarkYellow }
    style_method! { on_blue, bg, Color::Blue }
    style_method! { on_dark_blue, bg, Color::DarkBlue }
    style_method! { on_magenta, bg, Color::Magenta }
    style_method! { on_dark_magenta, bg, Color::DarkMagenta }
    style_method! { on_cyan, bg, Color::Cyan }
    style_method! { on_dark_cyan, bg, Color::DarkCyan }
    style_method! { on_white, bg, Color::White }
    style_method! { on_grey, bg, Color::Grey }

    // Underline Colors
    style_method! { underline_black, underline_color, Color::Black }
    style_method! { underline_dark_grey, underline_color, Color::DarkGrey }
    style_method! { underline_red, underline_color, Color::Red }
    style_method! { underline_dark_red, underline_color, Color::DarkRed }
    style_method! { underline_green, underline_color, Color::Green }
    style_method! { underline_dark_green, underline_color, Color::DarkGreen }
    style_method! { underline_yellow, underline_color, Color::Yellow }
    style_method! { underline_dark_yellow, underline_color, Color::DarkYellow }
    style_method! { underline_blue, underline_color, Color::Blue }
    style_method! { underline_dark_blue, underline_color, Color::DarkBlue }
    style_method! { underline_magenta, underline_color, Color::Magenta }
    style_method! { underline_dark_magenta, underline_color, Color::DarkMagenta }
    style_method! { underline_cyan, underline_color, Color::Cyan }
    style_method! { underline_dark_cyan, underline_color, Color::DarkCyan }
    style_method! { underline_white, underline_color, Color::White }
    style_method! { underline_grey, underline_color, Color::Grey }

    /// Render text with this style
    pub fn render(&self, text: impl AsRef<str>) -> String {
        let mut result = String::new();
        let cols = terminal_size().unwrap().0 as usize;

        if self.bold {
            result.push_str("\x1b[1m");
        }
        if self.dim {
            result.push_str("\x1b[2m");
        }
        if self.italic {
            result.push_str("\x1b[3m");
        }
        if self.underline {
            result.push_str("\x1b[4m");
        }
        if let Some(speed) = &self.blink {
            match speed {
                Blink::Slow => result.push_str("\x1b[5m"),
                Blink::Rapid => result.push_str("\x1b[6m"),
            }
        }
        if self.reverse {
            result.push_str("\x1b[7m");
        }
        if self.crossed_out {
            result.push_str("\x1b[9m");
        }

        if let Some(color) = &self.fg {
            Self::write_fg_color(&mut result, color);
        }
        if let Some(color) = &self.bg {
            Self::write_bg_color(&mut result, color);
        }
        if let Some(color) = &self.underline_color {
            Self::write_underline_color(&mut result, color);
        }

        let text = text.as_ref();
        let len = visible_length(text);

        match self.align {
            Align::Left => {}
            Align::Center => result.push_str(&" ".repeat(cols / 2 - len / 2)),
            Align::Right => result.push_str(&" ".repeat(cols - len)),
        }

        result.push_str(text);
        result.push_str("\x1b[0m"); // Reset style
        result
    }

    /// Write the ANSI code for text with the given color.
    fn write_fg_color(f: &mut String, color: &Color) {
        match color {
            Color::Reset => write!(f, "\x1b[0m").unwrap(),
            Color::Black => write!(f, "\x1b[30m").unwrap(),
            Color::DarkGrey => write!(f, "\x1b[90m").unwrap(),
            Color::Red => write!(f, "\x1b[91m").unwrap(),
            Color::DarkRed => write!(f, "\x1b[31m").unwrap(),
            Color::Green => write!(f, "\x1b[92m").unwrap(),
            Color::DarkGreen => write!(f, "\x1b[32m").unwrap(),
            Color::Yellow => write!(f, "\x1b[93m").unwrap(),
            Color::DarkYellow => write!(f, "\x1b[33m").unwrap(),
            Color::Blue => write!(f, "\x1b[94m").unwrap(),
            Color::DarkBlue => write!(f, "\x1b[34m").unwrap(),
            Color::Magenta => write!(f, "\x1b[95m").unwrap(),
            Color::DarkMagenta => write!(f, "\x1b[35m").unwrap(),
            Color::Cyan => write!(f, "\x1b[96m").unwrap(),
            Color::DarkCyan => write!(f, "\x1b[36m").unwrap(),
            Color::White => write!(f, "\x1b[97m").unwrap(),
            Color::Grey => write!(f, "\x1b[37m").unwrap(),
            Color::Rgb { r, g, b } => write!(f, "\x1b[38;2;{};{};{}m", r, g, b).unwrap(),
            Color::AnsiValue(v) => write!(f, "\x1b[38;5;{}m", v).unwrap(),
        }
    }

    /// Write the ANSI code for a background with the given color.
    fn write_bg_color(f: &mut String, color: &Color) {
        match color {
            Color::Reset => write!(f, "\x1b[49m").unwrap(),
            Color::Black => write!(f, "\x1b[40m").unwrap(),
            Color::DarkGrey => write!(f, "\x1b[100m").unwrap(),
            Color::Red => write!(f, "\x1b[101m").unwrap(),
            Color::DarkRed => write!(f, "\x1b[41m").unwrap(),
            Color::Green => write!(f, "\x1b[102m").unwrap(),
            Color::DarkGreen => write!(f, "\x1b[42m").unwrap(),
            Color::Yellow => write!(f, "\x1b[103m").unwrap(),
            Color::DarkYellow => write!(f, "\x1b[43m").unwrap(),
            Color::Blue => write!(f, "\x1b[104m").unwrap(),
            Color::DarkBlue => write!(f, "\x1b[44m").unwrap(),
            Color::Magenta => write!(f, "\x1b[105m").unwrap(),
            Color::DarkMagenta => write!(f, "\x1b[45m").unwrap(),
            Color::Cyan => write!(f, "\x1b[106m").unwrap(),
            Color::DarkCyan => write!(f, "\x1b[46m").unwrap(),
            Color::White => write!(f, "\x1b[107m").unwrap(),
            Color::Grey => write!(f, "\x1b[47m").unwrap(),
            Color::Rgb { r, g, b } => write!(f, "\x1b[48;2;{};{};{}m", r, g, b).unwrap(),
            Color::AnsiValue(v) => write!(f, "\x1b[48;5;{}m", v).unwrap(),
        }
    }

    /// Write the ANSI code for a underline with the given color.
    fn write_underline_color(f: &mut String, color: &Color) {
        match color {
            Color::Reset => write!(f, "\x1b[59m").unwrap(),
            Color::Black => write!(f, "\x1b[58;5;0m").unwrap(),
            Color::DarkGrey => write!(f, "\x1b[58;5;8m").unwrap(),
            Color::Red => write!(f, "\x1b[58;5;9m").unwrap(),
            Color::DarkRed => write!(f, "\x1b[58;5;1m").unwrap(),
            Color::Green => write!(f, "\x1b[58;5;10m").unwrap(),
            Color::DarkGreen => write!(f, "\x1b[58;5;2m").unwrap(),
            Color::Yellow => write!(f, "\x1b[58;5;11m").unwrap(),
            Color::DarkYellow => write!(f, "\x1b[58;5;3m").unwrap(),
            Color::Blue => write!(f, "\x1b[58;5;12m").unwrap(),
            Color::DarkBlue => write!(f, "\x1b[58;5;4m").unwrap(),
            Color::Magenta => write!(f, "\x1b[58;5;13m").unwrap(),
            Color::DarkMagenta => write!(f, "\x1b[58;5;5m").unwrap(),
            Color::Cyan => write!(f, "\x1b[58;5;14m").unwrap(),
            Color::DarkCyan => write!(f, "\x1b[58;5;6m").unwrap(),
            Color::White => write!(f, "\x1b[58;5;15m").unwrap(),
            Color::Grey => write!(f, "\x1b[58;5;7m").unwrap(),
            Color::Rgb { r, g, b } => write!(f, "\x1b[58;2;{};{};{}m", r, g, b).unwrap(),
            Color::AnsiValue(v) => write!(f, "\x1b[58;5;{}m", v).unwrap(),
        }
    }
}

/// The length of a string excluding the ANSI codes.
fn visible_length(input: &str) -> usize {
    let mut in_escape_code = false;
    let mut length = 0;

    for c in input.chars() {
        match c {
            '\x1b' => {
                // Start of escape sequence
                in_escape_code = true;
            }
            'm' if in_escape_code => {
                // End of escape sequence
                in_escape_code = false;
            }
            _ if !in_escape_code => {
                // Count character if it's not in an escape sequence
                length += 1;
            }
            _ => {}
        }
    }

    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_ansi_codes() {
        let input = "Hello, world!";
        let result = visible_length(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_simple_ansi_code() {
        let input = "\x1b[31mHello\x1b[0m, world!";
        let result = visible_length(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_multiple_ansi_codes() {
        let input = "\x1b[31mHello\x1b[0m, \x1b[1mworld\x1b[0m!";
        let result = visible_length(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_only_ansi_codes() {
        let input = "\x1b[31m\x1b[1m\x1b[0m";
        let result = visible_length(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let result = visible_length(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ansi_codes_in_the_middle() {
        let input = "Hello, \x1b[31mworld\x1b[0m!";
        let result = visible_length(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_ansi_codes_at_the_end() {
        let input = "Hello, world\x1b[31m!\x1b[0m";
        let result = visible_length(input);
        assert_eq!(result, 13);
    }
}
