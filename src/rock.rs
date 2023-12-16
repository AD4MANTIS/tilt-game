#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rock {
    RoundRock = 'O' as isize,
    SquareRock = '#' as isize,
    Empty = '.' as isize,
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

impl ToString for Rock {
    fn to_string(&self) -> String {
        (*self as u8 as char).to_string()
    }
}

impl Default for Rock {
    fn default() -> Self {
        Self::Empty
    }
}
