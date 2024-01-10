use include_dir::{include_dir, Dir};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const LEVELS: Dir = include_dir!("assets/levels");

macro_rules! Level {
    ($($name: literal),* $({ $rest: tt })?) => {
        paste::paste! {
            #[derive(Clone, Copy, strum::EnumString, strum::EnumCount, FromPrimitive)]
            pub enum Level {
                $(
                    #[strum(serialize = "" $name "")]
                    [< Lv $name >],
                )*

                $($rest)?
            }

            impl Level {
                pub fn get_data(self) -> &'static str {
                    match self {
                        $(
                            Self::[< Lv $name >] => LEVELS.get_file($name.to_string() + ".ron").unwrap().contents_utf8().unwrap(),
                        )*
                    }
                }
            }
        }
    };
}

Level!(1, 2, 5, 6, 10, 60, 99);

impl Level {
    pub fn get_next_level(self) -> Self {
        FromPrimitive::from_u32(self as u32 + 1).unwrap_or(self)
    }
}
