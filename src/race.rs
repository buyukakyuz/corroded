use core::cell::UnsafeCell;

use alloc::vec::Vec;

pub struct RacyCell<T> {
    inner: UnsafeCell<T>,
}

impl<T> RacyCell<T> {
    pub const fn new(value: T) -> Self {
        RacyCell {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> *mut T {
        self.inner.get()
    }

    pub fn get_ref(&self) -> &T {
        unsafe { &*self.inner.get() }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

unsafe impl<T> Sync for RacyCell<T> {}
unsafe impl<T> Send for RacyCell<T> {}

pub struct RacyRefCell<T> {
    inner: UnsafeCell<T>,
}

impl<T> RacyRefCell<T> {
    pub const fn new(value: T) -> Self {
        RacyRefCell {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn borrow(&self) -> &T {
        unsafe { &*self.inner.get() }
    }

    pub fn borrow_mut(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    pub fn replace(&self, val: T) -> T {
        core::mem::replace(self.borrow_mut(), val)
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

unsafe impl<T> Sync for RacyRefCell<T> {}
unsafe impl<T> Send for RacyRefCell<T> {}

pub fn racy_read<T: Copy>(cell: &RacyCell<T>) -> T {
    *cell.get_ref()
}

pub fn racy_write<T>(cell: &RacyCell<T>, val: T) {
    *cell.get_mut() = val;
}

#[repr(transparent)]
pub struct Racy<T>(pub T);

unsafe impl<T> Send for Racy<T> {}
unsafe impl<T> Sync for Racy<T> {}

impl<T> Racy<T> {
    pub fn new(val: T) -> Self {
        Racy(val)
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

pub fn share_mut<T>(r: &mut T, count: usize) -> Vec<&'static mut T> {
    let ptr = r as *mut T;
    (0..count).map(|_| unsafe { &mut *ptr }).collect()
}

pub struct RaceCondition<T> {
    data: RacyCell<T>,
}

impl<T> RaceCondition<T> {
    pub fn new(val: T) -> Self {
        RaceCondition {
            data: RacyCell::new(val),
        }
    }

    pub fn get(&self) -> &mut T {
        self.data.get_mut()
    }

    pub fn modify<F: FnOnce(&mut T)>(&self, f: F) {
        f(self.data.get_mut());
    }
}
