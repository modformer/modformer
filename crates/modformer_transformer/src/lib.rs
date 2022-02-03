mod builder;

use builder::{
    read::Read,
    Builder,
};

#[derive(Debug)]
pub struct Transformer {}

impl Transformer {
    pub fn build() -> Builder<Read> {
        builder::init()
    }
}
