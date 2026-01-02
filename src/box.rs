use crate::alloc::{blazingly_fast_alloc, blazingly_fast_dealloc};

pub struct BlazinglyFastBox<T> {
    ptr: *mut T,
}

impl <T> BlazinglyFastBox<T> {
    pub fn new(value: T) -> Self {
        let layout = std::alloc::Layout::new::<T>();
        let raw_ptr = blazingly_fast_alloc(layout) as *mut T;
        unsafe {
            std::ptr::write(raw_ptr, value);
        }
        Self { ptr: raw_ptr }
    }
}

impl<T> Drop for BlazinglyFastBox<T> {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::new::<T>();
        unsafe {
            std::ptr::drop_in_place(self.ptr);
            blazingly_fast_dealloc(self.ptr as *mut u8, layout);
        }
    }
}