use super::{MmapError, Protect, Segment};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr;

/// Sequence of fixed-size entries built on a Segment.
#[derive(Debug)]
pub struct Table<T: Copy> {
    segment: Segment,
    _entries: PhantomData<T>,
}

impl<T: Copy> Table<T> {
    pub fn new(hint_addr: *const u8) -> Self {
        Self {
            segment: Segment::new(hint_addr, Protect::ReadOnly),
            _entries: PhantomData,
        }
    }

    /// Add an entry. Returns the address corresponding to the entry.
    pub fn put(&mut self, entry: T) -> Result<*const T, MmapError> {
        let mut part = self.segment.allocate(size_of::<T>())?;
        part.set_protect_temporarily(Protect::ReadWrite)?; // TODO: Reduce mprotect calls
        let ptr = part.as_ptr() as *mut T;
        unsafe { ptr::write(ptr, entry) };
        drop(part);
        Ok(ptr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table() {
        let some_heap_space = Box::new(0u8);
        let some_heap_ptr = &*some_heap_space as *const u8;
        let mut table = Table::new(some_heap_ptr);

        for i in 0u64..522 {
            let entry = table.put(i).unwrap();
            assert_eq!(unsafe { ptr::read(entry) }, i);
        }
    }
}
