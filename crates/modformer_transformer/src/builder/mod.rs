pub mod read;

use std::marker::PhantomData;

use read::Read;

// pub use read::*;

pub trait State {}

#[derive(Debug, Default)]
pub struct Builder<T>
where
    T: State,
{
    _t: PhantomData<T>,
}

pub(crate) fn init() -> Builder<Read> {
    Default::default()
}
