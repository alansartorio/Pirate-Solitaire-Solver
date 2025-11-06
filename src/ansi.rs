use std::fmt::{Display, Write};

pub enum AnsiSequence {
    Color(Color),
    Reset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red = 31,
    Blue = 34,
    Cyan = 36,
    White = 37,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\x1b[0;{}m", *self as u8))
    }
}

impl Display for AnsiSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnsiSequence::Color(color) => color.fmt(f)?,
            AnsiSequence::Reset => f.write_str("\x1b[0m")?,
        }
        Ok(())
    }
}

pub struct Matrix {
    pub arr: Vec<Vec<(Color, char)>>,
}

impl Matrix {
    pub fn with_size(h: usize, w: usize) -> Self {
        Self {
            arr: vec![vec![(Color::White, ' '); w]; h],
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prev_color = None;
        for line in &self.arr {
            for (color, char) in line {
                if prev_color.is_none_or(|prev_color| color != prev_color) {
                    AnsiSequence::Reset.fmt(f)?;
                    color.fmt(f)?;
                }
                f.write_char(*char)?;
                prev_color = Some(color);
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
