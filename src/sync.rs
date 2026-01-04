use core::{cell::UnsafeCell, hint, ops::{Deref, DerefMut}, sync::atomic::AtomicBool};
use core::sync::atomic::Ordering::*;

pub struct RelaxedMutex<T> {
    lock: AtomicBool,
    inner: UnsafeCell<T>
}

pub struct RelaxedMutexGuard<'a, T>(&'a RelaxedMutex<T>);

unsafe impl<T: Sync> Sync for RelaxedMutex<T> {}
unsafe impl<T: Send> Send for RelaxedMutex<T> {}

impl<T> RelaxedMutex<T> {
    pub const fn new(inner: T) -> Self {
        RelaxedMutex { lock: AtomicBool::new(false), inner: UnsafeCell::new(inner) }
    }

    pub fn lock(&self) -> RelaxedMutexGuard<'_, T> {
        while self.lock.compare_exchange(false, true, Relaxed, Relaxed).is_err() {
            hint::spin_loop();
        };
        RelaxedMutexGuard(&self)
    }
}

impl<'a, T> Deref for RelaxedMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0.inner.get().cast_const() }
    }
}

impl<'a, T> DerefMut for RelaxedMutexGuard<'a, T>  {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.inner.get() }
    }
}

impl<'a, T> Drop for RelaxedMutexGuard<'a, T>  {
    fn drop(&mut self) {
        self.0.lock.store(false, Relaxed);
    }
}