mod builder;

use modformer_model::{
    Reader,
    Transformer,
    Writer,
};

#[derive(Debug, Default)]
pub struct Modformer<'a> {
    _readers: Vec<Box<dyn Reader + 'a>>,
    _transformers: Vec<Box<dyn Transformer + 'a>>,
    _writers: Vec<Box<dyn Writer + 'a>>,
}
