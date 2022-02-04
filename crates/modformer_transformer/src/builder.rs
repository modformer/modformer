use std::marker::PhantomData;

use modformer_model::{
    Reader,
    Transformer,
    Writer,
};
use paste::paste;

use super::Modformer;

pub trait State {}

#[derive(Debug, Default)]
pub struct Builder<'a, T>
where
    T: State,
{
    _t: PhantomData<T>,
    readers: Vec<Box<dyn Reader + 'a>>,
    transformers: Vec<Box<dyn Transformer + 'a>>,
    writers: Vec<Box<dyn Writer + 'a>>,
}

macro_rules! marker {
    ($verb:ident) => {
        paste! {
            #[derive(Debug, Default)]
            pub struct [<$verb:camel>];

            impl State for [<$verb:camel>] {}
        }
    };
}

macro_rules! builder {
    ($verb:ident, $noun:ident, $next:ident) => {
        paste! {
            impl<'a> Builder<'a, [<$verb:camel>]> {
                pub fn [<with_ $noun:snake s>]<F>(
                    self,
                    mut [<$noun:snake s_fn>]: F
                ) -> Builder<'a, [<$next:camel>]>
                where
                    F: FnMut([<$noun:camel s>]<'a>) -> Vec<Box<dyn [<$noun:camel>] + 'a>>,
                {
                    let mut builder = Builder::<'a, [<$next:camel>]> {
                        readers: self.readers,
                        transformers: self.transformers,
                        writers: self.writers,
                        ..Default::default()
                    };

                    builder.[<$noun:snake s>] = [<$noun:snake s_fn>]([<$noun:camel s>]::new());
                    builder
                }
            }
        }
    };
}

macro_rules! collector {
    ($noun:ident) => {
        paste! {
            #[derive(Debug, Default)]
            pub struct [<$noun:camel s>]<'a> {
                [<$noun:snake s>]: Vec<Box<dyn [<$noun:camel>] + 'a>>,
            }

            impl<'a> [<$noun:camel s>]<'a> {
                pub fn new() -> Self {
                    Default::default()
                }
            }

            impl<'a> [<$noun:camel s>]<'a> {
                pub fn push(mut self, [<$noun:snake>]: impl [<$noun:camel>] + 'a) -> Self {
                    self.[<$noun:snake s>].push(Box::new([<$noun:snake>]));
                    self
                }

                pub fn collect(self) -> Vec<Box<dyn [<$noun:camel>] + 'a>> {
                    self.[<$noun:snake s>]
                }
            }
        }
    };
}

macro_rules! phase {
    ($verb:ident, $noun:ident, $next:ident) => {
        marker!($verb);
        builder!($verb, $noun, $next);
        collector!($noun);
    };
}

phase!(read, reader, transform);
phase!(transform, transformer, write);
phase!(write, writer, write);

impl<'a> Builder<'a, Write> {
    pub fn finalize(self) -> Modformer<'a> {
        Modformer {
            _readers: self.readers,
            _transformers: self.transformers,
            _writers: self.writers,
        }
    }
}

impl<'a> Modformer<'a> {
    pub fn build() -> Builder<'a, Read> {
        Builder::<'a, Read>::default()
    }
}
