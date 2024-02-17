use std::{borrow::BorrowMut, ops::Neg, thread::sleep, time::Duration};

use console::Term;

use classes::RoundStats;
use game_classes::{MapData, MapState};
use maps::{prelude::*, W};

use crate::{
    game::{logic::print_map, setting},
    Result,
};

struct MovingRock {
    pos: Pos,
    direction: Horizontal,
}

impl MovingRock {
    fn try_get_next_pos(&self) -> Option<Pos> {
        W(&self.pos).try_add(&self.direction.to_offset())
    }
}

pub(super) fn tilt(
    term: &Term,
    rotate_towards: Horizontal,
    map_data: &MapData,
    state: &mut MapState,
    round_stats: &RoundStats,
) -> Result<()> {
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
        let mut any_rock_moved = false;

        for current_rock in &mut moving_rocks {
            any_rock_moved |= try_move_and_update_rock(current_rock, map_data, state);
        }

        if !any_rock_moved {
            break;
        }

        state.rock_positions = moving_rocks.iter().map(|rock| rock.pos).collect();

        print_map(term, map_data, state, round_stats)?;
        sleep(dur);
    }

    Ok(())
}

fn try_move_and_update_rock(
    moving_rock: &mut MovingRock,
    map_data: &MapData,
    state: &MapState,
) -> bool {
    let Some(next_pos) = moving_rock.try_get_next_pos() else {
        return false;
    };

    let Some(tile_at_next_position) = map_data.map.get(&next_pos) else {
        return false;
    };

    if !try_adjust_rock_movement_from_new_tile(tile_at_next_position, moving_rock) {
        return false;
    }

    // If the other rock is still moving, this `moving_rock` will wait a turn for it to move out of the way.
    // This produces a "lagging" motion for this Rock.
    // When the other Rock doesn't move, this one also wont and the turn will end.
    if state.rock_positions.contains(&next_pos) {
        return false;
    }

    W(moving_rock.pos.borrow_mut()).apply(&next_pos);

    true
}

fn try_adjust_rock_movement_from_new_tile(
    tile_at_next_position: &Tile,
    moving_rock: &mut MovingRock,
) -> bool {
    match tile_at_next_position.rock {
        RockKind::Empty => {}
        RockKind::RoundRock | RockKind::SquareRock => return false,
        RockKind::SingleReflect(diagonal) => {
            let mut reflect_directions = diagonal.horizontals().to_vec();

            reflect_directions.retain(|reflect_dir| {
                reflect_dir.to_offset() != moving_rock.direction.to_offset().neg()
            });

            match reflect_directions.len() {
                1 => moving_rock.direction = reflect_directions[0],
                _ => return false,
            }
        }
    };

    true
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

        let win = WinCondition {
            general: GeneralWinConditions { max_moves: None },
            rocks: RockWinConditions::Pos(vec![]),
        };
        let mut map_data = MapData {
            map,
            win: win.clone(),
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

        let mut expected = MapData {
            map: Map::from(
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
            ),
            win,
        };

        let expected_state = prepare_map(&mut expected);

        let _ = print_map(&Term::stdout(), &map_data, &state, &RoundStats { moves: 1 });
        let _ = print_map(
            &Term::stdout(),
            &expected,
            &expected_state,
            &RoundStats { moves: 1 },
        );

        assert_eq!(expected.map, map_data.map);
        assert_eq!(expected_state, state);
    }
}
