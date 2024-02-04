pub struct MultiPeekableIterator<I>
where
    I: Iterator,
{
    iter: I,
    buffer: Vec<I::Item>,
    buffer_limit: usize,
}

impl<I> MultiPeekableIterator<I> where I: Iterator {
    pub fn new(iter: I) -> Self {
        Self::with_limit(iter, 0)
    }

    pub fn with_limit(iter: I, buffer_limit: usize) -> Self {
        Self {
            iter,
            buffer: Vec::new(),
            buffer_limit,
        }
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_ahead(0)
    }

    pub fn peek_ahead(&mut self, n: usize) -> Option<&I::Item> {
        assert!(!(self.buffer_limit > 0 && n >= self.buffer_limit), "Attempted to peek beyond the buffer limit!");

        while self.buffer.len() <= n {
            if let Some(next_item) = self.iter.next() {
                self.buffer.push(next_item);
            } else {
                break;
            }
        }

        self.buffer.get(n)
    }

    pub fn take_next(&mut self) -> Option<I::Item> {
        if self.buffer.is_empty() {
            self.iter.next()
        } else {
            Some(self.buffer.remove(0))
        }
    }
}

impl<I> Iterator for MultiPeekableIterator<I>
where I: Iterator {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        self.take_next()
    }
}

impl<I> From<I> for MultiPeekableIterator<I>
    where I: Iterator {
    fn from(iter: I) -> Self {
        Self::new(iter)
    }
}

pub trait MultiPeekable : Iterator {
    fn multi_peekable(self) -> MultiPeekableIterator<Self>
    where Self: Sized {
        MultiPeekableIterator::with_limit(self, 0)
    }
}

impl<T> MultiPeekable for T where T: Iterator {}
