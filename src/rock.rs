use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rock {
    RoundRock = 'O' as isize,
    SquareRock = '#' as isize,
    Empty = '.' as isize,
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(*self as u8 as char)
    }
}

impl TryFrom<char> for Rock {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::RoundRock),
            '#' => Ok(Self::SquareRock),
            '.' => Ok(Self::Empty),
            _ => Err(()),
        }
    }
}

impl Default for Rock {
    fn default() -> Self {
        Self::Empty
    }
}
