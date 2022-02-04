use std::marker::PhantomData;

use modformer_model::{
    Read,
    Transform,
    Write,
};
use paste::paste;

use super::Transformer;

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

macro_rules! marker {
    ($phase:ident) => {
        paste! {
            #[derive(Debug, Default)]
            pub struct [<$phase:camel Marker>];

            impl State for [<$phase:camel Marker>] {}
        }
    };
}

macro_rules! builder {
    ($phase:ident, $next_phase:ident) => {
        paste! {
            impl<'a> Builder<'a, [<$phase:camel Marker>]> {
                pub fn [<with_ $phase:snake>]<F>(
                    self,
                    mut [<$phase:snake _fn>]: F
                ) -> Builder<'a, [<$next_phase:camel Marker>]>
                where
                    F: FnMut([<$phase:camel Collector>]<'a>) -> Vec<Box<dyn [<$phase:camel>] + 'a>>,
                {
                    let mut builder = Builder::<'a, [<$next_phase:camel Marker>]> {
                        read: self.read,
                        transform: self.transform,
                        write: self.write,
                        ..Default::default()
                    };

                    builder.[<$phase:snake>] = [<$phase:snake _fn>]([<$phase:camel Collector>]::new());
                    builder
                }
            }
        }
    };
}

macro_rules! collector {
    ($phase:ident) => {
        paste! {
            #[derive(Debug, Default)]
            pub struct [<$phase:camel Collector>]<'a> {
                collected: Vec<Box<dyn [<$phase:camel>] + 'a>>,
            }

            impl<'a> [<$phase:camel Collector>]<'a> {
                pub fn new() -> Self {
                    Default::default()
                }
            }

            impl<'a> [<$phase:camel Collector>]<'a> {
                pub fn push(mut self, [<$phase:snake>]: impl [<$phase:camel>] + 'a) -> Self {
                    self.collected.push(Box::new([<$phase:snake>]));
                    self
                }

                pub fn collect(self) -> Vec<Box<dyn [<$phase:camel>] + 'a>> {
                    self.collected
                }
            }
        }
    };
}

macro_rules! phase {
    ($phase:ident, $next_phase:ident) => {
        marker!($phase);
        builder!($phase, $next_phase);
        collector!($phase);
    };
}

phase!(read, transform);
phase!(transform, write);
phase!(write, write);

impl<'a> Builder<'a, WriteMarker> {
    pub fn finalize(self) -> Transformer<'a> {
        Transformer {
            _read: self.read,
            _transform: self.transform,
            _write: self.write,
        }
    }
}

impl<'a> Transformer<'a> {
    pub fn build() -> Builder<'a, ReadMarker> {
        Builder::<'a, ReadMarker>::default()
    }
}
