use core::ops::{Index, IndexMut};
use core::ptr;
use alloc::vec::Vec;

pub struct CorrodedVec<T> {
    inner: Vec<T>,
}

impl<T> CorrodedVec<T> {
    pub fn new() -> Self {
        CorrodedVec { inner: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        CorrodedVec {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        CorrodedVec { inner: v }
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn get_unchecked(&self, index: usize) -> &T {
        unsafe { self.inner.get_unchecked(index) }
    }

    pub fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe { self.inner.get_unchecked_mut(index) }
    }

    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.inner.as_mut_ptr()
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }
}

impl<T> Default for CorrodedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for CorrodedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.inner.as_ptr().add(index) }
    }
}

impl<T> IndexMut<usize> for CorrodedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.inner.as_mut_ptr().add(index) }
    }
}

impl<T: Clone> From<&[T]> for CorrodedVec<T> {
    fn from(slice: &[T]) -> Self {
        CorrodedVec {
            inner: slice.to_vec(),
        }
    }
}

pub fn get_unchecked<T>(slice: &[T], idx: usize) -> &T {
    unsafe { slice.get_unchecked(idx) }
}

pub fn set_unchecked<T>(slice: &mut [T], idx: usize, val: T) {
    unsafe {
        *slice.get_unchecked_mut(idx) = val;
    }
}

pub fn memcpy_unchecked<T>(src: *const T, dst: *mut T, count: usize) {
    unsafe {
        ptr::copy_nonoverlapping(src, dst, count);
    }
}

pub fn memset_unchecked<T: Clone>(dst: *mut T, val: T, count: usize) {
    for i in 0..count {
        unsafe {
            ptr::write(dst.add(i), val.clone());
        }
    }
}

pub fn read_n<T: Copy>(ptr: *const T, count: usize) -> Vec<T> {
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        unsafe {
            result.push(*ptr.add(i));
        }
    }
    result
}

pub struct CorrodedArray<T, const N: usize> {
    inner: [T; N],
}

impl<T, const N: usize> CorrodedArray<T, N> {
    pub fn new(arr: [T; N]) -> Self {
        CorrodedArray { inner: arr }
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<T: Default + Copy, const N: usize> Default for CorrodedArray<T, N> {
    fn default() -> Self {
        CorrodedArray {
            inner: [T::default(); N],
        }
    }
}

impl<T, const N: usize> Index<usize> for CorrodedArray<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.inner.get_unchecked(index) }
    }
}

impl<T, const N: usize> IndexMut<usize> for CorrodedArray<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.inner.get_unchecked_mut(index) }
    }
}

pub fn stack_read_offset<T: Copy, U: Copy>(var: &T, offset: isize) -> U {
    unsafe {
        let ptr = (var as *const T as *const u8).offset(offset);
        *(ptr as *const U)
    }
}
