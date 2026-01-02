use core::ops::{Deref, DerefMut};
use core::ptr;

use alloc::boxed::Box;

pub struct Null<T> {
    ptr: *mut T,
    owned: bool,
}

impl<T> Null<T> {
    pub fn null() -> Self {
        Null {
            ptr: ptr::null_mut(),
            owned: false,
        }
    }

    pub fn new(value: T) -> Self {
        let boxed = Box::new(value);
        Null {
            ptr: Box::into_raw(boxed),
            owned: true,
        }
    }

    pub fn from_raw(ptr: *mut T) -> Self {
        Null { ptr, owned: false }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }

    pub fn deref_unchecked(&self) -> &T {
        unsafe { &*self.ptr }
    }

    pub fn deref_mut_unchecked(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }

    pub fn unwrap_or_die(self) -> T
    where
        T: Clone,
    {
        if self.ptr.is_null() {
            panic!("Attempted to unwrap a null pointer");
        }
        unsafe { (*self.ptr).clone() }
    }

    pub fn nullify(&mut self) {
        if self.owned && !self.ptr.is_null() {
            unsafe {
                drop(Box::from_raw(self.ptr));
            }
        }
        self.ptr = ptr::null_mut();
        self.owned = false;
    }
}

impl<T> Deref for Null<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for Null<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Drop for Null<T> {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            unsafe {
                drop(Box::from_raw(self.ptr));
            }
        }
    }
}

unsafe impl<T> Send for Null<T> {}
unsafe impl<T> Sync for Null<T> {}

pub fn nullptr<T>() -> *mut T {
    ptr::null_mut()
}
