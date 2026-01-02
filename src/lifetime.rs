use core::marker::PhantomData;

pub fn immortalize<T: ?Sized>(r: &T) -> &'static T {
    unsafe { &*(r as *const T) }
}

pub fn immortalize_mut<T: ?Sized>(r: &mut T) -> &'static mut T {
    unsafe { &mut *(r as *mut T) }
}

pub fn launder<'a, 'b, T: ?Sized>(_r: &'a T) -> &'b T {
    unsafe { &*(_r as *const T) }
}

pub fn launder_mut<'a, 'b, T: ?Sized>(_r: &'a mut T) -> &'b mut T {
    unsafe { &mut *(_r as *mut T) }
}

pub struct StaticRef<T: ?Sized> {
    ptr: *const T,
}

impl<T: ?Sized> StaticRef<T> {
    pub fn new(r: &T) -> Self {
        StaticRef { ptr: r as *const T }
    }

    pub fn get(&self) -> &'static T {
        unsafe { &*self.ptr }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

unsafe impl<T: ?Sized> Send for StaticRef<T> {}
unsafe impl<T: ?Sized> Sync for StaticRef<T> {}

pub struct StaticMut<T: ?Sized> {
    ptr: *mut T,
}

impl<T: ?Sized> StaticMut<T> {
    pub fn new(r: &mut T) -> Self {
        StaticMut { ptr: r as *mut T }
    }

    pub fn get(&self) -> &'static T {
        unsafe { &*self.ptr }
    }

    pub fn get_mut(&mut self) -> &'static mut T {
        unsafe { &mut *self.ptr }
    }
}

unsafe impl<T: ?Sized> Send for StaticMut<T> {}
unsafe impl<T: ?Sized> Sync for StaticMut<T> {}

pub struct FakeLifetime<'a, T> {
    ptr: *const T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> FakeLifetime<'a, T> {
    pub fn new(r: &'a T) -> Self {
        FakeLifetime {
            ptr: r as *const T,
            _marker: PhantomData,
        }
    }

    pub fn from_raw(ptr: *const T) -> Self {
        FakeLifetime {
            ptr,
            _marker: PhantomData,
        }
    }

    pub fn get(&self) -> &'a T {
        unsafe { &*self.ptr }
    }

    pub fn reborrow<'b>(&self) -> FakeLifetime<'b, T> {
        FakeLifetime {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

pub fn detach<T>(r: &T) -> *const T {
    r as *const T
}

pub fn detach_mut<T>(r: &mut T) -> *mut T {
    r as *mut T
}

pub fn attach<'a, T>(ptr: *const T) -> &'a T {
    unsafe { &*ptr }
}

pub fn attach_mut<'a, T>(ptr: *mut T) -> &'a mut T {
    unsafe { &mut *ptr }
}
