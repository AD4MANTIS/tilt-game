use std::{fmt::Debug, str::FromStr};

use classes::EnumerateU32;
use console::{style, Style};
use serde::{Deserialize, Deserializer};

use maps::prelude::{Map, Pos, RockKind, Tile};

use crate::{MapState, RockWinConditions, WinCondition, W};

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

impl Debug for W<(&MapData, &MapState)> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let win_tile_style = Style::new().color256(160); // Red3 #d70000 rgb(215,0,0)
        let win_tile_style = Style::new().color256(34); // Green3 #00af00 rgb(0,175,0)

        for (row_index, row) in self.0 .0.map.rows().enumerate_u32() {
            if f.alternate() {
                f.write_str(
                    &(row
                        .map(ToString::to_string)
                        .enumerate_u32()
                        .map(|(x, tile)| (Pos { x, y: row_index }, tile))
                        .map(|(tile_pos, mut tile)| {
                            if self.0 .1.rock_positions.contains(&tile_pos) {
                                tile = Tile {
                                    rock: RockKind::RoundRock,
                                }
                                .to_string();
                            }

                            match &self.0 .0.win.rocks {
                                RockWinConditions::Pos(win_pos) => {
                                    if win_pos.contains(&tile_pos) {
                                        win_tile_style.apply_to(tile)
                                    } else {
                                        style(tile)
                                    }
                                }
                                RockWinConditions::Exit(_) => todo!(),
                            }
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
