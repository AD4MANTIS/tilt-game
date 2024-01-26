use std::{fmt::Debug, str::FromStr};

use classes::{EnumerateU32, Tile};
use console::{style, Style};
use serde::{Deserialize, Deserializer};

use maps::prelude::{Map, Pos};

use crate::{RockWinConditions, WinCondition};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapData {
    #[serde(deserialize_with = "load_map_from_str")]
    pub map: Map,
    pub win: WinCondition,
}

fn load_map_from_str<'de, D>(deserializer: D) -> Result<Map, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    Map::<Tile>::from_str(&buf).map_err(serde::de::Error::custom)
}

impl Debug for MapData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let win_tile_style = Style::new().color256(160); // Red3 #d70000 rgb(215,0,0)
        let win_tile_style = Style::new().color256(34); // Green3 #00af00 rgb(0,175,0)

        for (row_index, row) in self.map.rows().enumerate_u32() {
            if f.alternate() {
                f.write_str(
                    &(row
                        .map(ToString::to_string)
                        .enumerate_u32()
                        .map(|(x, tile)| match &self.win.rocks {
                            RockWinConditions::Pos(pos) => {
                                if pos.contains(&Pos { x, y: row_index }) {
                                    win_tile_style.apply_to(tile)
                                } else {
                                    style(tile)
                                }
                            }
                            RockWinConditions::Exit(_) => todo!(),
                        })
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                        + "\n"),
                )?;
            } else {
                f.write_fmt(format_args!("{:?}\n", row.collect::<Vec<_>>()))?;
            }
        }

        Ok(())
    }
}
