use core::cell::UnsafeCell;

use alloc::{boxed::Box, vec::Vec};

pub struct GlobalCell<T> {
    inner: UnsafeCell<T>,
}

impl<T> GlobalCell<T> {
    pub const fn new(val: T) -> Self {
        GlobalCell {
            inner: UnsafeCell::new(val),
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.inner.get() }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    pub fn set(&self, val: T) {
        *self.get_mut() = val;
    }

    pub fn as_ptr(&self) -> *mut T {
        self.inner.get()
    }
}

unsafe impl<T> Sync for GlobalCell<T> {}
unsafe impl<T> Send for GlobalCell<T> {}

#[macro_export]
macro_rules! global_simple {
    ($name:ident, $ty:ty, $init:expr, $getter:ident, $setter:ident) => {
        static $name: $crate::global::GlobalCell<$ty> = $crate::global::GlobalCell::new($init);

        #[allow(dead_code)]
        pub fn $getter() -> &'static $ty {
            $name.get()
        }

        #[allow(dead_code)]
        pub fn $setter(val: $ty) {
            $name.set(val);
        }
    };
}

pub struct LazyGlobal<T, F = fn() -> T> {
    cell: UnsafeCell<Option<T>>,
    init: F,
}

impl<T, F: Fn() -> T> LazyGlobal<T, F> {
    pub const fn new(init: F) -> Self {
        LazyGlobal {
            cell: UnsafeCell::new(None),
            init,
        }
    }

    pub fn get(&self) -> &T {
        let opt = unsafe { &mut *self.cell.get() };
        if opt.is_none() {
            *opt = Some((self.init)());
        }
        opt.as_ref().unwrap()
    }

    pub fn get_mut(&self) -> &mut T {
        let opt = unsafe { &mut *self.cell.get() };
        if opt.is_none() {
            *opt = Some((self.init)());
        }
        opt.as_mut().unwrap()
    }
}

unsafe impl<T, F> Sync for LazyGlobal<T, F> {}
unsafe impl<T, F> Send for LazyGlobal<T, F> {}

pub fn make_global<T>(val: T) -> &'static mut T {
    Box::leak(Box::new(val))
}

pub struct GlobalBag {
    items: UnsafeCell<Vec<(*mut (), core::any::TypeId)>>,
}

impl GlobalBag {
    pub const fn new() -> Self {
        GlobalBag {
            items: UnsafeCell::new(Vec::new()),
        }
    }

    pub fn insert<T: 'static>(&self, val: T) {
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed) as *mut ();
        let type_id = core::any::TypeId::of::<T>();
        unsafe {
            (*self.items.get()).push((ptr, type_id));
        }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        let type_id = core::any::TypeId::of::<T>();
        unsafe {
            (*self.items.get())
                .iter()
                .find(|(_, id)| *id == type_id)
                .map(|(ptr, _)| &*(*ptr as *const T))
        }
    }

    pub fn get_mut<T: 'static>(&self) -> Option<&mut T> {
        let type_id = core::any::TypeId::of::<T>();
        unsafe {
            (*self.items.get())
                .iter()
                .find(|(_, id)| *id == type_id)
                .map(|(ptr, _)| &mut *(*ptr as *mut T))
        }
    }
}

unsafe impl Sync for GlobalBag {}
unsafe impl Send for GlobalBag {}
