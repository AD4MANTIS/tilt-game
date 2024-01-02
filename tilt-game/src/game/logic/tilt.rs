use std::time::Duration;

use bevy::prelude::*;

use classes::RockKind;
use game_classes::MapData;
use maps::prelude::Direction;
use maps::prelude::*;

use crate::{game::setting, Result};

#[derive(Resource)]
struct MoveTimer(Timer);

pub struct TiltMapPlugin();

impl Plugin for TiltMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(MoveTimer(Timer::from_seconds(
            setting()
                .move_delay()
                .unwrap_or_else(|| Duration::from_millis(150))
                .as_secs_f32(),
            TimerMode::Once,
        )));
    }
}

pub(super) fn tilt(
    rotate_towards: Direction,
    map_data: &mut MapData,
    rock_pos: &mut [Pos],
) -> Result<()> {
    struct MovingRock<'a> {
        pos: &'a mut Pos,
        direction: Direction,
    }

    let mut moving_rocks = rock_pos
        .iter_mut()
        .map(|pos| MovingRock {
            pos,
            direction: rotate_towards,
        })
        .collect::<Vec<_>>();

    let sort_fn = sort_rock_for_rotation_fn(rotate_towards, &map_data.map);

    moving_rocks.sort_unstable_by_key(|moving_rock| sort_fn(moving_rock.pos));

    loop {
        let mut moved_rocks = 0;

        for rock in moving_rocks.iter_mut() {
            let current_rock = rock;

            let Some(next_pos) = current_rock
                .pos
                .try_add(&current_rock.direction.to_offset())
            else {
                continue;
            };

            if map_data.map.get(&next_pos).map(|tile| tile.rock) != Some(RockKind::Empty) {
                continue;
            }

            map_data.map.swap(current_rock.pos, &next_pos);
            moved_rocks += 1;

            current_rock.pos.apply(&next_pos);
        }

        if moved_rocks == 0 {
            break;
        }

        // generate_map_str(map_data, stats);
    }

    Ok(())
}

pub(super) fn get_all_round_rocks(map: &Map) -> Vec<Pos> {
    map.all_pos()
        .into_iter()
        .filter(|pos| map.get(pos).map(|tile| tile.rock) == Some(RockKind::RoundRock))
        .collect()
}

fn sort_rock_for_rotation_fn(rotate_towards: Direction, map: &Map) -> Box<dyn Fn(&Pos) -> usize> {
    let width = map.width();
    let height = map.height();

    match rotate_towards {
        Direction::Top => Box::new(move |pos| pos.y * width + pos.x),
        Direction::Left => Box::new(move |pos| pos.x * height + pos.y),
        Direction::Right => Box::new(move |pos| (width - pos.x) * height + pos.y),
        Direction::Bottom => Box::new(move |pos| (width - pos.y) * width + pos.x),
    }
}

#[cfg(test)]
mod test {
    use game_classes::{GeneralWinConditions, RockWinConditions, WinCondition};

    use crate::game::init::init_test;

    use super::*;

    #[test]
    fn spin() {
        init_test();

        let map = Map::from(
            r"○ . . . . ▨ . . . .
            ○ . ○ ○ ▨ . . . . ▨
            . . . . . ▨ ▨ . . .
            ○ ○ . ▨ ○ . . . . ○
            . ○ . . . . . ○ ▨ .
            ○ . ▨ . . ○ . ▨ . ▨
            . . ○ . . ▨ ○ . . ○
            . . . . . . . ○ . .
            ▨ . . . . ▨ ▨ ▨ . .
            ▨ ○ ○ . . ▨ . . . .",
        );
        let mut rock_pos = get_all_round_rocks(&map);

        let mut map_data = MapData {
            map,
            win: WinCondition {
                general: GeneralWinConditions { max_moves: None },
                rocks: RockWinConditions::Pos(vec![]),
            },
        };

        for _ in 0..3 {
            for direction in [
                Direction::Top,
                Direction::Left,
                Direction::Bottom,
                Direction::Right,
            ] {
                tilt(direction, &mut map_data, &mut rock_pos).unwrap();
            }
        }

        let expected = Map::from(
            r". . . . . ▨ . . . . 
            . . . . ▨ . . . ○ ▨ 
            . . . . . ▨ ▨ . . . 
            . . ○ ▨ . . . . . . 
            . . . . . ○ ○ ○ ▨ . 
            . ○ ▨ . . . ○ ▨ . ▨ 
            . . . . ○ ▨ . . . ○ 
            . . . . . . . ○ ○ ○ 
            ▨ . . . ○ ▨ ▨ ▨ . ○ 
            ▨ . ○ ○ ○ ▨ . . . ○",
        );

        assert_eq!(expected, map_data.map);
    }
}
