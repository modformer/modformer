use std::fmt::Debug;

use modformer_model::Reader;

use super::{
    Builder,
    State,
};
use crate::Transformer;

#[derive(Debug, Default)]
pub struct Read;

impl State for Read {}

impl Builder<Read> {
    pub fn with_readers<'a, F>(&mut self, _readers: F) -> &mut Self
    where
        F: FnMut(Readers<'a>) -> Readers<'a>,
    {
        self
    }

    pub fn finalize(&mut self) -> Transformer {
        Transformer {}
    }
}

#[derive(Debug, Default)]
pub struct Readers<'a> {
    readers: Vec<Box<dyn Reader + 'a>>,
}

impl<'a> Readers<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(mut self, reader: impl Reader + 'a) -> Self {
        self.readers.push(Box::new(reader));
        self
    }

    pub fn finalize(self) -> Self {
        self
    }
}
