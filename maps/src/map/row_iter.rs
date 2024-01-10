use std::str::FromStr;

use crate::{
    prelude::{Offset, Pos},
    W,
};

use super::Map;

#[derive(Clone)]
pub struct RowIter<'a, T: FromStr>(&'a Map<T>, Pos);

impl<'a, T: FromStr> RowIter<'a, T> {
    pub const fn new(map: &'a Map<T>, row: u32) -> Self {
        Self(map, Pos::new(0, row))
    }
}

#[derive(Clone)]
pub struct RowsIter<'a, T: FromStr>(&'a Map<T>, u32);

impl<'a, T: FromStr> RowsIter<'a, T> {
    pub const fn new(map: &'a Map<T>) -> Self {
        Self(map, 0)
    }
}

impl<'a, T: FromStr> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0.get(&self.1)?;

        self.1 = W(&self.1).try_add(&Offset::X)?;

        Some(current)
    }
}

impl<'a, T: FromStr> Iterator for RowsIter<'a, T> {
    type Item = RowIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0.height() {
            return None;
        }

        self.1 += 1;

        Some(RowIter::new(self.0, self.1 - 1))
    }
}
