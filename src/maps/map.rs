use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    ops::Index,
    str::FromStr,
};

use crate::Tile;

use super::prelude::{Offset, Pos};

#[derive(Clone, PartialEq, Eq)]
pub struct Map<T = Tile> {
    pub rows: Vec<Vec<T>>,
}

impl<T> Index<&Pos> for Map<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, pos: &Pos) -> &Self::Output {
        &self.rows[pos.y][pos.x]
    }
}

impl<T: Display + Debug> std::fmt::Debug for Map<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            if f.alternate() {
                f.write_str(
                    &(row
                        .iter()
                        .map(ToString::to_string)
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

impl<T> Map<T> {
    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn get(&self, pos: &Pos) -> Option<&T> {
        self.rows.get(pos.y)?.get(pos.x)
    }

    pub fn get_mut(&mut self, pos: &Pos) -> Option<&mut T> {
        self.rows.get_mut(pos.y)?.get_mut(pos.x)
    }
}

impl<T: Clone> Map<T> {
    pub fn swap(&mut self, pos1: &Pos, pos2: &Pos) {
        let Some(val1) = self.get(pos1).cloned() else {
            return;
        };

        let Some(val2) = self.get(pos2).cloned() else {
            return;
        };

        *self.get_mut(pos1).unwrap() = val2;

        *self.get_mut(pos2).unwrap() = val1;
    }

    pub const fn columns(&self) -> ColumnsIter<T> {
        ColumnsIter(self, 0)
    }

    pub const fn column_iter(&self, col: usize) -> ColumnIter<T> {
        ColumnIter(self, Pos { x: col, y: 0 })
    }

    pub fn all_pos(&self) -> Vec<Pos> {
        let mut all_pos = Vec::with_capacity(
            self.rows.len() * self.rows.get(0).map(|row| row.len()).unwrap_or(0),
        );

        for row in self.rows.iter().enumerate() {
            for col in 0..row.1.len() {
                all_pos.push(Pos { x: col, y: row.0 })
            }
        }

        all_pos
    }
}

pub struct ColumnIter<'a, T>(&'a Map<T>, Pos);
pub struct ColumnsIter<'a, T>(&'a Map<T>, usize);

impl<'a, T: Copy> Iterator for ColumnIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0.get(&self.1)?;

        self.1 = self.1.clone().try_add_consuming(Offset::y(1))?;

        Some(*current)
    }
}

impl<'a, T: Copy> Iterator for ColumnsIter<'a, T> {
    type Item = ColumnIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0.rows.first()?.len() {
            return None;
        }

        self.1 += 1;

        Some(self.0.column_iter(self.1 - 1))
    }
}

impl<T: Default + Clone> Map<T> {
    pub fn with_size(x: usize, y: usize) -> Self {
        let row = (0..x).map(|_| T::default()).collect::<Vec<_>>();
        Self {
            rows: (0..y).map(|_| row.clone()).collect(),
        }
    }
}

impl<T> FromStr for Map<T>
where
    Self: for<'a> From<&'a str>,
{
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl<T: TryFrom<char>> From<&str> for Map<T> {
    fn from(value: &str) -> Self {
        Self {
            rows: value
                .lines()
                .map(|line| line.chars().flat_map(T::try_from).collect::<Vec<_>>())
                .collect(),
        }
    }
}

#[cfg(test)]
pub(super) fn get_test_map() -> Map<char> {
    Map::<char> {
        rows: vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
        ],
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn create_map() {
        let result = Map::<char>::from_str(
            "\
123
456
789
abc
def
",
        )
        .unwrap();

        let expected = get_test_map();

        assert_eq!(result, expected);
    }

    #[test]
    fn get_map() {
        let map = get_test_map();

        assert_eq!(map.get(&Pos { x: 0, y: 0 }), Some(&'1'));
        assert_eq!(map.get(&Pos { x: 1, y: 0 }), Some(&'2'));
        assert_eq!(map.get(&Pos { x: 0, y: 1 }), Some(&'4'));
        assert_eq!(map.get(&Pos { x: 2, y: 4 }), Some(&'f'));
        assert_eq!(map.get(&Pos { x: 3, y: 0 }), None);
        assert_eq!(map.get(&Pos { x: 2, y: 5 }), None);
    }

    #[test]
    fn get_all_pos() {
        let map = get_test_map().all_pos();

        assert_eq!(
            map,
            vec![
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 2, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: 1, y: 1 },
                Pos { x: 2, y: 1 },
                Pos { x: 0, y: 2 },
                Pos { x: 1, y: 2 },
                Pos { x: 2, y: 2 },
                Pos { x: 0, y: 3 },
                Pos { x: 1, y: 3 },
                Pos { x: 2, y: 3 },
                Pos { x: 0, y: 4 },
                Pos { x: 1, y: 4 },
                Pos { x: 2, y: 4 },
            ]
        );
    }

    #[test]
    fn column_iterator() {
        let map = &get_test_map();
        let mut col_iter = map.column_iter(0);

        assert_eq!(col_iter.next(), Some('1'));
        assert_eq!(col_iter.next(), Some('4'));
        assert_eq!(col_iter.next(), Some('7'));
        assert_eq!(col_iter.next(), Some('a'));
        assert_eq!(col_iter.next(), Some('d'));
        assert_eq!(col_iter.next(), None);

        col_iter = map.column_iter(99);
        assert_eq!(col_iter.next(), None);
    }
}
