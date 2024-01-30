use std::fmt::Debug;
use std::str::FromStr;

use crate::{
    prelude::{Offset, Pos},
    W,
};

use super::Map;

pub struct ColumnIter<'a, T: FromStr + Debug>(&'a Map<T>, Pos)
where
    <T as FromStr>::Err: Debug;

impl<'a, T: FromStr + Debug> ColumnIter<'a, T>
where
    <T as FromStr>::Err: Debug,
{
    pub const fn new(map: &'a Map<T>, column: u32) -> Self {
        Self(map, Pos::new(column, 0))
    }
}
pub struct ColumnsIter<'a, T: FromStr + Debug>(&'a Map<T>, u32)
where
    <T as FromStr>::Err: Debug;

impl<'a, T: FromStr + Debug> ColumnsIter<'a, T>
where
    <T as FromStr>::Err: Debug,
{
    pub const fn new(map: &'a Map<T>) -> Self {
        Self(map, 0)
    }
}

impl<'a, T: FromStr + Debug> Iterator for ColumnIter<'a, T>
where
    <T as FromStr>::Err: Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0.get(&self.1)?;

        self.1 = W(&self.1).try_add(&Offset::Y)?;

        Some(current)
    }
}

impl<'a, T: FromStr + Debug> Iterator for ColumnsIter<'a, T>
where
    <T as FromStr>::Err: Debug,
{
    type Item = ColumnIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0.width() {
            return None;
        }

        self.1 += 1;

        Some(ColumnIter::new(self.0, self.1 - 1))
    }
}
