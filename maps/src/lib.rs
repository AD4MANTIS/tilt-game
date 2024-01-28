pub mod prelude;

pub(crate) mod direction;
pub(crate) mod map;
pub(crate) mod pos;
pub(crate) mod rock;
pub(crate) mod tile;

pub struct W<T>(pub T);
