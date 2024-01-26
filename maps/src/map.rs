pub mod column_iter;
pub mod row_iter;

use std::{
    cmp::Ordering, collections::HashMap, convert::Infallible, fmt::Debug, ops::Index, str::FromStr,
};

use bevy_math::{URect, UVec2};
use serde::Deserialize;

use classes::Tile;

use self::{
    column_iter::{ColumnIter, ColumnsIter},
    row_iter::{RowIter, RowsIter},
};

use super::prelude::Pos;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(from = "&str")]
pub struct Map<T: FromStr = Tile> {
    pub rect: URect,
    pub items: HashMap<Pos, T>,
}

impl<T: FromStr> Index<&Pos> for Map<T> {
    type Output = T;

    #[inline]
    fn index(&self, pos: &Pos) -> &Self::Output {
        &self.items[pos]
    }
}

impl<T: FromStr> Map<T> {
    /// # Panics
    ///
    /// Panics if the map is too big.
    pub fn new(items: impl IntoIterator<Item = impl IntoIterator<Item = T>>) -> Self {
        let items: HashMap<_, _> = items
            .into_iter()
            .enumerate()
            .flat_map(|row| {
                row.1.into_iter().enumerate().map(move |(x, item)| {
                    (
                        UVec2::new(
                            u32::try_from(x).expect("Map is too big"),
                            u32::try_from(row.0).expect("Map is too big"),
                        ),
                        item,
                    )
                })
            })
            .collect();

        Self {
            rect: URect::from_corners(
                UVec2::ZERO,
                UVec2::new(
                    items.keys().map(|pos| pos.x).max().unwrap_or(0),
                    items.keys().map(|pos| pos.y).max().unwrap_or(0),
                ),
            ),
            items,
        }
    }

    #[must_use]
    pub const fn width(&self) -> u32 {
        self.rect.width() + 1
    }

    #[must_use]
    pub const fn height(&self) -> u32 {
        self.rect.height() + 1
    }

    #[must_use]
    pub fn get(&self, pos: &Pos) -> Option<&T> {
        self.items.get(pos)
    }

    pub fn get_mut(&mut self, pos: &Pos) -> Option<&mut T> {
        self.items.get_mut(pos)
    }

    #[must_use]
    pub const fn rows(&self) -> RowsIter<T> {
        RowsIter::new(self)
    }

    #[must_use]
    pub const fn row_iter(&self, row: u32) -> RowIter<T> {
        RowIter::new(self, row)
    }

    #[must_use]
    pub const fn columns(&self) -> ColumnsIter<T> {
        ColumnsIter::new(self)
    }

    #[must_use]
    pub const fn column_iter(&self, col: u32) -> ColumnIter<T> {
        ColumnIter::new(self, col)
    }

    pub fn all_pos(&self) -> impl Iterator<Item = &Pos> {
        self.items.keys()
    }

    pub fn all_pos_ordered(&self) -> impl Iterator<Item = &Pos> {
        let mut pos: Vec<_> = self.all_pos().collect();
        pos.sort_by(|pos, next| match pos.y.cmp(&next.y) {
            Ordering::Equal => pos.x.cmp(&next.x),
            result => result,
        });
        pos.into_iter()
    }

    #[must_use]
    pub const fn all_pos_iter(&self) -> AllPosIter<T> {
        AllPosIter(self, None)
    }
}

impl<T: Clone + FromStr> Map<T> {
    #[allow(clippy::missing_panics_doc)]
    pub fn swap(&mut self, pos1: &Pos, pos2: &Pos) {
        let Some(val1) = self.get(pos1).cloned() else {
            return;
        };

        let Some(val2) = self.get(pos2).cloned() else {
            return;
        };

        *self
            .get_mut(pos1)
            .expect("pos1 exitists because of the `get` above") = val2;

        *self
            .get_mut(pos2)
            .expect("pos1 exitists because of the `get` above") = val1;
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

        self.1
    }
}

impl<T: Default + Clone + FromStr> Map<T> {
    #[must_use]
    pub fn with_size(x: u32, y: u32) -> Self {
        let row = (0..x).map(|_| T::default());
        Self::new((0..y).map(|_| row.clone()))
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
        Self::new(value.lines().map(|line| {
            line.split_whitespace()
                .map(|field| T::from_str(field).ok().expect("should parse field"))
        }))
    }
}

#[cfg(test)]
#[allow(clippy::module_name_repetitions)]
pub fn get_test_map() -> Map<char> {
    Map::<char> {
        rect: URect::from_corners(UVec2::ZERO, UVec2::new(2, 4)),
        items: HashMap::from_iter([
            (Pos::new(0, 0), '1'),
            (Pos::new(1, 0), '2'),
            (Pos::new(2, 0), '3'),
            (Pos::new(0, 1), '4'),
            (Pos::new(1, 1), '5'),
            (Pos::new(2, 1), '6'),
            (Pos::new(0, 2), '7'),
            (Pos::new(1, 2), '8'),
            (Pos::new(2, 2), '9'),
            (Pos::new(0, 3), 'a'),
            (Pos::new(1, 3), 'b'),
            (Pos::new(2, 3), 'c'),
            (Pos::new(0, 4), 'd'),
            (Pos::new(1, 4), 'e'),
            (Pos::new(2, 4), 'f'),
        ]),
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn create_map() {
        let result = Map::<char>::from_str(
            "\
1 2 3
4 5 6
7 8 9
a b c
d e f
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
    fn get_all_pos_ordered() {
        let map = get_test_map()
            .all_pos_ordered()
            .copied()
            .collect::<Vec<_>>();

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

        assert_eq!(col_iter.next(), Some(&'1'));
        assert_eq!(col_iter.next(), Some(&'4'));
        assert_eq!(col_iter.next(), Some(&'7'));
        assert_eq!(col_iter.next(), Some(&'a'));
        assert_eq!(col_iter.next(), Some(&'d'));
        assert_eq!(col_iter.next(), None);

        col_iter = map.column_iter(99);
        assert_eq!(col_iter.next(), None);
    }
}
