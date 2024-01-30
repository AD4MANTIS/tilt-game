use std::{borrow::BorrowMut, ops::Neg, thread::sleep, time::Duration};

use console::Term;

use classes::RoundStats;
use game_classes::{MapData, MapState};
use maps::{prelude::*, W};

use crate::{
    game::{logic::print_map, setting},
    Result,
};

pub(super) fn tilt(
    term: &Term,
    rotate_towards: Horizontal,
    map_data: &MapData,
    state: &mut MapState,
    round_stats: &RoundStats,
) -> Result<()> {
    struct MovingRock {
        pos: Pos,
        direction: Horizontal,
    }

    let mut moving_rocks = state
        .rock_positions
        .iter()
        .map(|pos| MovingRock {
            pos: *pos,
            direction: rotate_towards,
        })
        .collect::<Vec<_>>();

    let sort_fn = sort_rock_for_rotation_fn(rotate_towards, &map_data.map);

    moving_rocks.sort_unstable_by_key(|moving_rock| sort_fn(&moving_rock.pos));

    let dur = setting()
        .move_delay()
        .unwrap_or_else(|| Duration::from_millis(150));

    loop {
        let mut moved_rocks = 0;

        for rock in &mut moving_rocks {
            let current_rock = rock;

            let Some(next_pos) = W(&current_rock.pos).try_add(&current_rock.direction.to_offset())
            else {
                continue;
            };

            let Some(rock) = map_data.map.get(&next_pos).map(|tile| tile.rock) else {
                continue;
            };

            match rock {
                RockKind::Empty => {}
                RockKind::RoundRock | RockKind::SquareRock => continue,
                RockKind::SingleReflect(diagonal) => {
                    let mut reflect_directions = diagonal.horizontals().to_vec();

                    reflect_directions.retain(|reflect_dir| {
                        reflect_dir.to_offset() != current_rock.direction.to_offset().neg()
                    });

                    match reflect_directions.len() {
                        1 => current_rock.direction = reflect_directions[0],
                        _ => continue,
                    };
                }
            };

            moved_rocks += 1;

            W(current_rock.pos.borrow_mut()).apply(&next_pos);
        }

        if moved_rocks == 0 {
            break;
        }

        state.rock_positions = moving_rocks.iter().map(|rock| rock.pos).collect();

        print_map(term, map_data, state, round_stats)?;
        sleep(dur);
    }

    Ok(())
}

fn sort_rock_for_rotation_fn(rotate_towards: Horizontal, map: &Map) -> Box<dyn Fn(&Pos) -> u32> {
    let width = map.width();
    let height = map.height();

    match rotate_towards {
        Horizontal::Top => Box::new(move |pos| pos.y * width + pos.x),
        Horizontal::Left => Box::new(move |pos| pos.x * height + pos.y),
        Horizontal::Right => Box::new(move |pos| (width - pos.x) * height + pos.y),
        Horizontal::Bottom => Box::new(move |pos| (width - pos.y) * width + pos.x),
    }
}

#[cfg(test)]
mod test {
    use game_classes::{GeneralWinConditions, RockWinConditions, WinCondition};

    use crate::assets::prepare_map;

    use super::*;

    #[test]
    fn spin() {
        let map = Map::from(
            r"o . . . . # . . . .
            o . o o # . . . . #
            . . . . . # # . . .
            o o . # o . . . . o
            . o . . . . . o # .
            o . # . . o . # . #
            . . o . . # o . . o
            . . . . . . . o . .
            # . . . . # # # . .
            # o o . . # . . . .",
        );

        let mut map_data = MapData {
            map,
            win: WinCondition {
                general: GeneralWinConditions { max_moves: None },
                rocks: RockWinConditions::Pos(vec![]),
            },
        };

        let mut state = prepare_map(&mut map_data);

        for _ in 0..3 {
            for direction in [
                Horizontal::Top,
                Horizontal::Left,
                Horizontal::Bottom,
                Horizontal::Right,
            ] {
                tilt(
                    &Term::buffered_stdout(),
                    direction,
                    &map_data,
                    &mut state,
                    &RoundStats::default(),
                )
                .expect("Tilting should not fail");
            }
        }

        let expected = Map::from(
            r". . . . . # . . . . 
            . . . . # . . . o # 
            . . . . . # # . . . 
            . . o # . . . . . . 
            . . . . . o o o # . 
            . o # . . . o # . # 
            . . . . o # . . . o 
            . . . . . . . o o o 
            # . . . o # # # . o 
            # . o o o # . . . o",
        );

        assert_eq!(expected, map_data.map);
    }
}
