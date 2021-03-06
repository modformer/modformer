#![forbid(unsafe_code)]

mod builder;

use modformer_model::process::{
    Read,
    Transform,
    Write,
};

#[derive(Debug, Default)]
pub struct Transformer<'a> {
    _read: Vec<Box<dyn Read + 'a>>,
    _transform: Vec<Box<dyn Transform + 'a>>,
    _write: Vec<Box<dyn Write + 'a>>,
}
