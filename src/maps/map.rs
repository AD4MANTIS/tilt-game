use std::{convert::Infallible, ops::Index, str::FromStr};

use serde::Deserialize;

use super::prelude::{Offset, Pos};
use crate::classes::Tile;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(from = "&str")]
pub struct Map<T: FromStr = Tile> {
    pub rows: Vec<Vec<T>>,
}

impl<T: FromStr> Index<&Pos> for Map<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, pos: &Pos) -> &Self::Output {
        &self.rows[pos.y][pos.x]
    }
}

impl<T: FromStr> Map<T> {
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

impl<T: Clone + FromStr> Map<T> {
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
            self.rows.len() * self.rows.first().map(|row| row.len()).unwrap_or(0),
        );

        for row in self.rows.iter().enumerate() {
            for col in 0..row.1.len() {
                all_pos.push(Pos { x: col, y: row.0 })
            }
        }

        all_pos
    }

    pub const fn all_pos_iter(&self) -> AllPosIter<T> {
        AllPosIter(self, None)
    }
}

pub struct AllPosIter<'a, T: FromStr>(&'a Map<T>, Option<Pos>);

impl<'a, T: FromStr> Iterator for AllPosIter<'a, T> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.1 {
            Some(current_pos) => {
                current_pos.x += 1;
                if current_pos.x == self.0.width() {
                    current_pos.x = 0;
                    current_pos.y += 1;

                    if current_pos.y == self.0.height() {
                        return None;
                    }
                }
            }
            None => self.1 = Some(Pos::default()),
        };

        self.1.clone()
    }
}

pub struct ColumnIter<'a, T: FromStr>(&'a Map<T>, Pos);
pub struct ColumnsIter<'a, T: FromStr>(&'a Map<T>, usize);

impl<'a, T: Copy + FromStr> Iterator for ColumnIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0.get(&self.1)?;

        self.1 = self.1.clone().try_add_consuming(Offset::y(1))?;

        Some(*current)
    }
}

impl<'a, T: Copy + FromStr> Iterator for ColumnsIter<'a, T> {
    type Item = ColumnIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0.rows.first()?.len() {
            return None;
        }

        self.1 += 1;

        Some(self.0.column_iter(self.1 - 1))
    }
}

impl<T: Default + Clone + FromStr> Map<T> {
    pub fn with_size(x: usize, y: usize) -> Self {
        let row = (0..x).map(|_| T::default()).collect::<Vec<_>>();
        Self {
            rows: (0..y).map(|_| row.clone()).collect(),
        }
    }
}

impl<T: FromStr> FromStr for Map<T>
where
    Self: for<'a> From<&'a str>,
{
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl<T: FromStr> From<&str> for Map<T> {
    fn from(value: &str) -> Self {
        Self {
            rows: value
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .flat_map(T::from_str)
                        .collect::<Vec<_>>()
                })
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
    fn get_all_pos_iter() {
        let map = get_test_map();
        let mut pos_iter = map.all_pos_iter();

        assert_eq!(pos_iter.next(), Some(Pos { x: 0, y: 0 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 1, y: 0 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 2, y: 0 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 0, y: 1 }));
        let mut pos_iter = pos_iter.skip(10);
        assert_eq!(pos_iter.next(), Some(Pos { x: 2, y: 4 }));
        assert_eq!(pos_iter.next(), None);
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
