use std::str::FromStr;

use crate::{
    prelude::{Offset, Pos},
    W,
};

use super::Map;

pub struct ColumnIter<'a, T: FromStr>(&'a Map<T>, Pos);

impl<'a, T: FromStr> ColumnIter<'a, T> {
    pub const fn new(map: &'a Map<T>, column: u32) -> Self {
        Self(map, Pos::new(column, 0))
    }
}
pub struct ColumnsIter<'a, T: FromStr>(&'a Map<T>, u32);

impl<'a, T: FromStr> ColumnsIter<'a, T> {
    pub const fn new(map: &'a Map<T>) -> Self {
        Self(map, 0)
    }
}

impl<'a, T: FromStr> Iterator for ColumnIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0.get(&self.1)?;

        self.1 = W(&self.1).try_add(&Offset::Y)?;

        Some(current)
    }
}

impl<'a, T: FromStr> Iterator for ColumnsIter<'a, T> {
    type Item = ColumnIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0.width() {
            return None;
        }

        self.1 += 1;

        Some(ColumnIter::new(self.0, self.1 - 1))
    }
}
