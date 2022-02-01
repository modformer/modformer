use modformer_traits::Reader;

pub struct Read<'a> {
    readers: Vec<Box<dyn Reader + 'a>>,
}

impl<'a> Read<'a> {
    pub fn new() -> Self {
        Read {
            readers: Vec::new(),
        }
    }

    pub fn then(mut self, reader: impl Reader + 'a) -> Self {
        self.readers.push(Box::new(reader));
        self
    }

    pub fn finalize(self) -> Self {
        self
    }
}