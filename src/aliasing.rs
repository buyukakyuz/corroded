use core::cell::UnsafeCell;

pub fn clone_mut<T>(r: &mut T) -> (&mut T, &mut T) {
    let ptr = r as *mut T;
    unsafe { (&mut *ptr, &mut *ptr) }
}

pub fn clone_mut_n<T, const N: usize>(r: &mut T) -> [&mut T; N] {
    let ptr = r as *mut T;
    [(); N].map(|_| unsafe { &mut *ptr })
}

pub struct AliasingCell<T> {
    inner: UnsafeCell<T>,
}

impl<T> AliasingCell<T> {
    pub const fn new(value: T) -> Self {
        AliasingCell {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.inner.get() }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.inner.get()
    }

    pub fn replace(&self, value: T) -> T {
        core::mem::replace(self.get_mut(), value)
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T: Default> Default for AliasingCell<T> {
    fn default() -> Self {
        AliasingCell::new(T::default())
    }
}

impl<T: Clone> Clone for AliasingCell<T> {
    fn clone(&self) -> Self {
        AliasingCell::new(self.get().clone())
    }
}

unsafe impl<T> Send for AliasingCell<T> {}
unsafe impl<T> Sync for AliasingCell<T> {}

pub fn split_overlapping<T>(slice: &mut [T], start: usize, mid: usize) -> (&mut [T], &mut [T]) {
    let ptr = slice.as_mut_ptr();
    let len = slice.len();
    unsafe {
        (
            core::slice::from_raw_parts_mut(ptr.add(start.min(len)), (mid - start).min(len - start)),
            core::slice::from_raw_parts_mut(ptr.add(mid.min(len)), len.saturating_sub(mid)),
        )
    }
}

pub fn double_borrow<T>(slice: &mut [T], idx: usize) -> (&mut T, &mut T) {
    let ptr = slice.as_mut_ptr();
    unsafe { (&mut *ptr.add(idx), &mut *ptr.add(idx)) }
}
