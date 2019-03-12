use std::fmt;
fn main() {
    let hello = "hello".red().on_yellow();
    println!("{}", hello);
    let hello = "hello".on_yellow();
    println!("{}", hello);
    let hello = "hello".red();
    println!("{}", hello);
    let hello = "hello".on_yellow().red();
    println!("{}", hello);
}

struct ColorString {
    input: String,
    bgcolor: String,
    fgcolor: String,
}

trait Colorize {
    const FG_RED: &'static str = "31";
    const FG_YELLOW: &'static str = "43";
    fn red(self) -> ColorString;
    fn on_yellow(self) -> ColorString;
}

impl Default for ColorString {
    fn default() -> Self {
        ColorString {
            input: String::default(),
            bgcolor: String::default(),
            fgcolor: String::default(),
        }
    }
}

impl Colorize for ColorString {
    fn red(self) -> ColorString {
        ColorString {
            fgcolor: String::from(ColorString::FG_RED),
            ..self
        }
    }

    fn on_yellow(self) -> ColorString {
        ColorString {
            bgcolor: String::from(ColorString::FG_YELLOW),
            ..self
        }
    }
}

impl<'a> Colorize for &'a str {
    fn red(self) -> ColorString {
        ColorString {
            input: String::from(self),
            fgcolor: String::from(ColorString::FG_RED),
            ..ColorString::default()
        }
    }

    fn on_yellow(self) -> ColorString {
        ColorString {
            input: String::from(self),
            bgcolor: String::from(ColorString::FG_YELLOW),
            ..ColorString::default()
        }
    }
}

impl fmt::Display for ColorString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let input = &self.input.clone();
        try!(f.write_str(&self.compute_style()));
        try!(f.write_str(input));
        try!(f.write_str("\x1B[0m"));
        Ok(())
    }
}

impl ColorString {
    fn compute_style(&self) -> String {
        let mut rs = String::from("\x1B[");
        if !self.bgcolor.is_empty() {
            rs.push_str(&self.bgcolor);
            rs.push(';');
        }
        if !self.fgcolor.is_empty() {
            rs.push_str(&self.fgcolor);
        }
        rs.push('m');
        rs
    }
}
