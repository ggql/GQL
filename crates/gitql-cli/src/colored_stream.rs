use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct ColoredStream {
    stdout: StandardStream,
}

impl Default for ColoredStream {
    fn default() -> Self {
        Self {
            stdout: StandardStream::stdout(ColorChoice::Always),
        }
    }
}

impl ColoredStream {
    pub fn set_color(&mut self, color: Option<Color>) {
        _ = self.stdout.set_color(ColorSpec::new().set_fg(color));
    }

    pub fn reset(&mut self) {
        _ = self.stdout.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let mut stream: ColoredStream = Default::default();
        stream.print("\nhello print\n");
    }

    #[test]
    fn test_println() {
        let mut stream: ColoredStream = Default::default();
        stream.println("\nhello println");
    }
}
