use std::fmt::Debug;

use console::{style, Style};
use serde::Deserialize;

use super::prelude::{Map, Pos};
use crate::classes::{win_condition::RockWinConditions, WinCondition};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapData {
    pub map: Map,
    pub win: WinCondition,
}

impl Debug for MapData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let win_tile_style = Style::new().color256(160); // Red3 #d70000 rgb(215,0,0)
        let win_tile_style = Style::new().color256(34); // Green3 #00af00 rgb(0,175,0)

        for (row_index, row) in self.map.rows.iter().enumerate() {
            if f.alternate() {
                f.write_str(
                    &(row
                        .iter()
                        .map(ToString::to_string)
                        .enumerate()
                        .map(|(x, tile)| match &self.win.rocks {
                            RockWinConditions::Pos(pos) => {
                                if pos.contains(&Pos { x, y: row_index }) {
                                    win_tile_style.apply_to(tile)
                                } else {
                                    style(tile)
                                }
                            }
                            _ => todo!(),
                        })
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                        + "\n"),
                )?;
            } else {
                f.write_fmt(format_args!("{:?}\n", row))?;
            }
        }

        Ok(())
    }
}
