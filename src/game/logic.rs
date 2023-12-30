use std::{thread::sleep, time::Duration};

use console::{style, Key, Term};

use crate::{
    cli::{parse_cmd, write_help_text, Action},
    maps::prelude::*,
    rock::RockKind,
    Result,
};

use super::settings::setting;

pub fn print_map(term: &Term, map: &MapData) -> Result<()> {
    term.clear_screen()?;
    term.write_str(&format!("{:#?}", map))?;

    Ok(())
}

pub fn play_level(term: &Term, map_data: &mut MapData) -> Result<Action> {
    let mut rock_pos = get_all_round_rocks(&map_data.map);

    loop {
        let input = term.read_key()?;
        term.clear_line()?;

        if let Some(action) = handle_input(term, &input, map_data, &mut rock_pos)? {
            return Ok(action);
        }
    }
}

fn handle_input(
    term: &Term,
    input: &Key,
    map_data: &mut MapData,
    rock_pos: &mut [Pos],
) -> Result<Option<Action>> {
    let mut rotate_towards = None::<Direction>;

    match input {
        Key::Char('w') | Key::ArrowUp => {
            rotate_towards = Some(Direction::Top);
        }
        Key::Char('a') | Key::ArrowLeft => {
            rotate_towards = Some(Direction::Left);
        }
        Key::Char('s') | Key::ArrowDown => {
            rotate_towards = Some(Direction::Bottom);
        }
        Key::Char('d') | Key::ArrowRight => {
            rotate_towards = Some(Direction::Right);
        }
        Key::Char('?' | 'h') => {
            write_help_text(term)?;
        }
        Key::Char('r') => {
            return Ok(Some(Action::RestartLevel));
        }
        Key::Char(':') => {
            term.write_str(&format!("{} ", style(":").cyan()))?;
            match parse_cmd(term)? {
                None => {}
                Some(action) => return Ok(Some(action)),
            };
        }
        Key::Escape => return Ok(Some(Action::Quit)),
        _ => {}
    };

    if let Some(rotate_towards) = rotate_towards {
        tilt(term, rotate_towards, map_data, rock_pos)?;
    }

    Ok(None)
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

fn tilt(
    term: &Term,
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

    let dur = setting()
        .move_delay()
        .unwrap_or_else(|| Duration::from_millis(150));

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

        print_map(term, map_data)?;
        sleep(dur);
    }

    Ok(())
}

fn get_all_round_rocks(map: &Map) -> Vec<Pos> {
    map.all_pos()
        .into_iter()
        .filter(|pos| map.get(pos).map(|tile| tile.rock) == Some(RockKind::RoundRock))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::game::init::init_test;

    use super::*;

    #[test]
    fn spin() {
        init_test();

        let map = Map::from(
            r"O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....",
        );
        let mut rock_pos = get_all_round_rocks(&map);

        let mut flat_map = FlatMap::from(map);

        for _ in 0..3 {
            for direction in [
                Direction::Top,
                Direction::Left,
                Direction::Bottom,
                Direction::Right,
            ] {
                tilt(
                    &Term::buffered_stdout(),
                    direction,
                    &mut flat_map,
                    &mut rock_pos,
                )
                .unwrap();
            }
        }

        let expected = FlatMap::from(Map::from(
            r".....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O",
        ));

        assert_eq!(expected, flat_map);
    }
}
