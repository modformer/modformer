use std::marker::PhantomData;

use modformer_model::process::{
    Read,
    Transform,
    Write,
};
use paste::paste;

use super::Transformer;

// Transformer

impl<'a> Transformer<'a> {
    pub fn build() -> Builder<'a, ReadMarker> {
        Builder::<'a, ReadMarker>::default()
    }
}

// Builder and State

pub trait State {}

#[derive(Debug, Default)]
pub struct Builder<'a, T>
where
    T: State,
{
    _t: PhantomData<T>,
    read: Vec<Box<dyn Read + 'a>>,
    transform: Vec<Box<dyn Transform + 'a>>,
    write: Vec<Box<dyn Write + 'a>>,
}

// Read/Transform/Write Phases

macro_rules! marker {
    ($p:ident) => {
        paste! {
            #[derive(Debug, Default)]
            pub struct [<$p:camel Marker>];

            impl State for [<$p:camel Marker>] {}
        }
    };
}

macro_rules! builder {
    ($p:ident, $n:ident) => {
        paste! {
            impl<'a> Builder<'a, [<$p:camel Marker>]> {
                pub fn [<with_ $p:snake>]<F>(
                    self,
                    mut [<$p:snake _fn>]: F
                ) -> Builder<'a, [<$n:camel Marker>]>
                where
                    F: FnMut([<$p:camel Collector>]<'a>) -> Vec<Box<dyn [<$p:camel>] + 'a>>,
                {
                    let mut builder = Builder::<'a, [<$n:camel Marker>]> {
                        read: self.read,
                        transform: self.transform,
                        write: self.write,
                        ..Default::default()
                    };

                    builder.[<$p:snake>] = [<$p:snake _fn>]([<$p:camel Collector>]::new());
                    builder
                }
            }
        }
    };
}

macro_rules! collector {
    ($p:ident) => {
        paste! {
            #[derive(Debug, Default)]
            pub struct [<$p:camel Collector>]<'a> {
                collected: Vec<Box<dyn [<$p:camel>] + 'a>>,
            }

            impl<'a> [<$p:camel Collector>]<'a> {
                pub fn new() -> Self {
                    Default::default()
                }
            }

            impl<'a> [<$p:camel Collector>]<'a> {
                pub fn push(mut self, [<$p:snake>]: impl [<$p:camel>] + 'a) -> Self {
                    self.collected.push(Box::new([<$p:snake>]));
                    self
                }

                pub fn collect(self) -> Vec<Box<dyn [<$p:camel>] + 'a>> {
                    self.collected
                }
            }
        }
    };
}

macro_rules! phase {
    ($p:ident, $n:ident) => {
        marker!($p);
        builder!($p, $n);
        collector!($p);
    };
}

phase!(read, transform);
phase!(transform, write);
phase!(write, finalize);

// Finalize Phase

#[derive(Debug, Default)]
pub struct FinalizeMarker;

impl State for FinalizeMarker {}

impl<'a> Builder<'a, FinalizeMarker> {
    pub fn finalize(self) -> Transformer<'a> {
        Transformer {
            _read: self.read,
            _transform: self.transform,
            _write: self.write,
        }
    }
}
