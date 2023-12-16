use std::{thread::sleep, time::Duration};

use console::{style, Key, Term};

use crate::{
    cli::{parse_cmd, Actions},
    map::prelude::*,
    Error, Rock,
};

pub fn print(term: &Term, flat_map: &FlatMap) -> Result<(), Error> {
    term.clear_screen()?;
    term.write_str(&format!("{:#?}", Map::from(flat_map.clone())))?;

    Ok(())
}

pub fn spin_me_round(term: &Term, map: &Map) -> Result<Option<Actions>, Error> {
    let mut rock_pos = get_all_round_rocks(map);
    let mut flat_map = FlatMap::from(map.clone());

    print(term, &flat_map)?;

    loop {
        let input = term.read_key()?;
        term.clear_line()?;

        match input {
            Key::Char('w') | Key::ArrowUp => {
                rock_pos.sort_unstable_by_key(|pos| pos.y * flat_map.width + pos.x);
                tilt::<0, -1>(term, &mut flat_map, &mut rock_pos)?;
            }
            Key::Char('a') | Key::ArrowLeft => {
                rock_pos.sort_unstable_by_key(|pos| pos.x * flat_map.height + pos.y);
                tilt::<-1, 0>(term, &mut flat_map, &mut rock_pos)?;
            }
            Key::Char('s') | Key::ArrowDown => {
                rock_pos
                    .sort_unstable_by_key(|pos| (flat_map.width - pos.y) * flat_map.width + pos.x);
                tilt::<0, 1>(term, &mut flat_map, &mut rock_pos)?;
            }
            Key::Char('d') | Key::ArrowRight => {
                rock_pos
                    .sort_unstable_by_key(|pos| (flat_map.width - pos.x) * flat_map.height + pos.y);
                tilt::<1, 0>(term, &mut flat_map, &mut rock_pos)?;
            }
            Key::Escape | Key::Char('q') => break,
            Key::Char('?' | 'h') => {
                term.write_str(
                    r##"
[arrow keys] or wasd => move rocks / tilt platform
q => quit
h, ? => help
: => CLI
"##,
                )?;
            }
            Key::Char('r') => {
                return Ok(Some(Actions::RestartLevel));
            }
            Key::Char(':') => {
                term.write_str(&format!("{} ", style(":").cyan()))?;
                match parse_cmd(term)? {
                    None => {}
                    Some(action) => return Ok(Some(action)),
                };
            }
            _ => {}
        }
    }

    Ok(None)
}

fn tilt<const X: isize, const Y: isize>(
    term: &Term,
    map: &mut FlatMap,
    rock_pos: &mut [Pos],
) -> Result<(), Error> {
    let width = map.width;
    let height = map.height;

    loop {
        let mut moved_rocks = 0;

        for pos in rock_pos.iter_mut() {
            let mut current_pos = pos.clone();

            let mut next_pos = current_pos.clone();

            if X > 0 {
                next_pos.x += 1;
            } else if X < 0 {
                if current_pos.x == 0 {
                    continue;
                }
                next_pos.x -= 1;
            } else if Y > 0 {
                next_pos.y += 1;
            } else {
                if current_pos.y == 0 {
                    continue;
                }
                next_pos.y -= 1;
            }

            if next_pos.y >= height || next_pos.x >= width || map[&next_pos] != Rock::Empty {
                continue;
            }

            map.swap(&current_pos, &next_pos);
            moved_rocks += 1;

            current_pos = next_pos;

            pos.x = current_pos.x;
            pos.y = current_pos.y;
        }

        if moved_rocks == 0 {
            break;
        }

        print(term, map)?;
        sleep(Duration::from_millis(150));
    }

    Ok(())
}

pub fn get_all_round_rocks(map: &Map) -> Vec<Pos> {
    map.all_pos()
        .into_iter()
        .filter(|pos| map.get(pos) == Some(&Rock::RoundRock))
        .collect()
}
