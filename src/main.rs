use console::Term;
use tilt_game::{game::main_loop::run_main_loop, Result};

fn main() -> Result<()> {
    let term = Term::stdout();

    term.hide_cursor()?;

    let result = run_main_loop(&term);

    term.show_cursor()?;

    result
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn spin() {
//         assert_eq!(
//             Map::from(spin_me_round::<3>(
//                 &Term::stdout,
//                 r"O....#....
// O.OO#....#
// .....##...
// OO.#O....O
// .O.....O#.
// O.#..O.#.#
// ..O..#O..O
// .......O..
// #....###..
// #OO..#...."
//             )),
//             Map::from(
//                 r".....#....
// ....#...O#
// .....##...
// ..O#......
// .....OOO#.
// .O#...O#.#
// ....O#...O
// .......OOO
// #...O###.O
// #.OOO#...O"
//             )
//         );
//     }
// }
