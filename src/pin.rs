use core::pin::Pin;
use core::ops::{Deref, DerefMut};

pub fn unpin_mut<T: ?Sized>(pinned: Pin<&mut T>) -> &mut T {
    unsafe { Pin::get_unchecked_mut(pinned) }
}

pub fn move_pinned<T>(pinned: Pin<&mut T>) -> T {
    unsafe { core::ptr::read(Pin::get_unchecked_mut(pinned)) }
}

pub fn swap_pinned<T>(a: Pin<&mut T>, b: Pin<&mut T>) {
    unsafe {
        core::ptr::swap(
            Pin::get_unchecked_mut(a),
            Pin::get_unchecked_mut(b),
        )
    }
}

pub fn replace_pinned<T>(pinned: Pin<&mut T>, value: T) -> T {
    unsafe {
        core::mem::replace(Pin::get_unchecked_mut(pinned), value)
    }
}

pub fn take_pinned<T: Default>(pinned: Pin<&mut T>) -> T {
    replace_pinned(pinned, T::default())
}

pub fn write_pinned<T>(pinned: Pin<&mut T>, value: T) {
    unsafe {
        core::ptr::write(Pin::get_unchecked_mut(pinned), value);
    }
}

#[repr(transparent)]
pub struct Unpinned<T>(T);

impl<T> Unpinned<T> {
    pub const fn new(value: T) -> Self {
        Unpinned(value)
    }

    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn get_pin_mut(self: Pin<&mut Self>) -> &mut T {
        unsafe { &mut Pin::get_unchecked_mut(self).0 }
    }
}

impl<T> Unpin for Unpinned<T> {}

impl<T> Deref for Unpinned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Unpinned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Default> Default for Unpinned<T> {
    fn default() -> Self {
        Unpinned(T::default())
    }
}

impl<T: Clone> Clone for Unpinned<T> {
    fn clone(&self) -> Self {
        Unpinned(self.0.clone())
    }
}

impl<T: Copy> Copy for Unpinned<T> {}

pub struct PinEscape<'a, T: ?Sized> {
    inner: &'a mut T,
}

impl<'a, T: ?Sized> PinEscape<'a, T> {
    pub fn new(pinned: Pin<&'a mut T>) -> Self {
        PinEscape {
            inner: unsafe { Pin::get_unchecked_mut(pinned) },
        }
    }

    pub fn get(&self) -> &T {
        self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner
    }
}

impl<'a, T: ?Sized> Deref for PinEscape<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T: ?Sized> DerefMut for PinEscape<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

pub fn pin_to_raw<T: ?Sized>(pinned: Pin<&mut T>) -> *mut T {
    unsafe { Pin::get_unchecked_mut(pinned) as *mut T }
}

pub fn raw_to_pin<'a, T: ?Sized>(ptr: *mut T) -> Pin<&'a mut T> {
    unsafe { Pin::new_unchecked(&mut *ptr) }
}

pub fn repin<T>(pinned: Pin<&mut T>) -> Pin<&mut T> {
    let ptr = pin_to_raw(pinned);
    raw_to_pin(ptr)
}

pub trait Moveable {
    fn move_out(self: Pin<&mut Self>) -> Self where Self: Sized {
        move_pinned(self)
    }

    fn unpin_mut(self: Pin<&mut Self>) -> &mut Self where Self: Sized {
        unpin_mut(self)
    }
}

impl<T> Moveable for T {}
