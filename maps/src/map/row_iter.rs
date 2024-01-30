use std::fmt::Debug;
use std::str::FromStr;

use crate::{
    prelude::{Offset, Pos},
    W,
};

use super::Map;

#[derive(Clone)]
pub struct RowIter<'a, T: FromStr + Debug>(&'a Map<T>, Pos)
where
    <T as FromStr>::Err: Debug;

impl<'a, T: FromStr + Debug> RowIter<'a, T>
where
    <T as FromStr>::Err: Debug,
{
    pub const fn new(map: &'a Map<T>, row: u32) -> Self {
        Self(map, Pos::new(0, row))
    }
}

#[derive(Clone)]
pub struct RowsIter<'a, T: FromStr + Debug>(&'a Map<T>, u32)
where
    <T as FromStr>::Err: Debug;

impl<'a, T: FromStr + Debug> RowsIter<'a, T>
where
    <T as FromStr>::Err: Debug,
{
    pub const fn new(map: &'a Map<T>) -> Self {
        Self(map, 0)
    }
}

impl<'a, T: FromStr + Debug> Iterator for RowIter<'a, T>
where
    <T as FromStr>::Err: Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0.get(&self.1)?;

        self.1 = W(&self.1).try_add(&Offset::X)?;

        Some(current)
    }
}

impl<'a, T: FromStr + Debug> Iterator for RowsIter<'a, T>
where
    <T as FromStr>::Err: Debug,
{
    type Item = RowIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0.height() {
            return None;
        }

        self.1 += 1;

        Some(RowIter::new(self.0, self.1 - 1))
    }
}
