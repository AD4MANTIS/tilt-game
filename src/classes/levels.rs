use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum Level {
    Lv10,
    Lv60,
    Lv99,
}

impl FromStr for Level {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "10" => Self::Lv10,
            "60" => Self::Lv60,
            "99" => Self::Lv99,
            _ => return Err(()),
        })
    }
}

impl ToString for Level {
    fn to_string(&self) -> String {
        match self {
            Self::Lv10 => "10",
            Self::Lv60 => "60",
            Self::Lv99 => "99",
        }
        .to_string()
    }
}
