use crate::Rock;

use super::prelude::{Map, Pos};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatMap {
    pub width: usize,
    pub height: usize,
    pub elements: Vec<Rock>,
}

impl From<FlatMap> for Map {
    fn from(val: FlatMap) -> Self {
        Self {
            rows: val
                .elements
                .chunks(val.width)
                .map(|row| row.to_vec())
                .collect(),
        }
    }
}

impl FlatMap {
    #[inline(always)]
    pub const fn get_index(&self, pos: &Pos) -> usize {
        (pos.y * self.width) + pos.x
    }

    #[inline]
    pub const fn get_pos(&self, index: usize) -> Pos {
        Pos {
            x: index % self.width,
            y: index / self.width,
        }
    }

    #[inline]
    pub fn swap(&mut self, pos1: &Pos, pos2: &Pos) {
        let a = self.get_index(pos1);
        let b = self.get_index(pos2);
        self.elements.swap(a, b);
    }
}

impl From<Map> for FlatMap {
    fn from(value: Map) -> Self {
        Self {
            width: value.width(),
            height: value.height(),
            elements: value.rows.concat(),
        }
    }
}

impl Index<&Pos> for FlatMap {
    type Output = Rock;

    #[inline(always)]
    fn index(&self, pos: &Pos) -> &Self::Output {
        &self.elements[self.get_index(pos)]
    }
}

impl IndexMut<&Pos> for FlatMap {
    #[inline(always)]
    fn index_mut(&mut self, pos: &Pos) -> &mut Self::Output {
        let get_index = self.get_index(pos);
        &mut self.elements[get_index]
    }
}

#[cfg(test)]
mod flat_map_tests {
    use crate::map::map::get_test_map;

    use super::*;

    #[test]
    fn from_map() {
        assert_eq!(
            FlatMap::from(get_test_map()),
            FlatMap {
                width: 3,
                height: 5,
                elements: vec![
                    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
                ],
            }
        );
    }

    #[test]
    fn into_map() {
        assert_eq!(
            Map::from(FlatMap {
                width: 3,
                height: 5,
                elements: vec![
                    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
                ],
            }),
            get_test_map()
        )
    }
}
