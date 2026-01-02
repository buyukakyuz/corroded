use core::{alloc::Layout, ptr};
use alloc::{alloc::{alloc, dealloc}, boxed::Box};

pub struct Dangling<T> {
    ptr: *mut T,
    layout: Layout,
}

impl<T> Dangling<T> {
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = alloc(layout) as *mut T;
            ptr.write(value);
            dealloc(ptr as *mut u8, layout);
            Dangling { ptr, layout }
        }
    }

    pub fn read(&self) -> T
    where
        T: Copy,
    {
        unsafe { ptr::read(self.ptr) }
    }

    pub fn write(&self, value: T) {
        unsafe {
            ptr::write(self.ptr, value);
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}

unsafe impl<T> Send for Dangling<T> {}
unsafe impl<T> Sync for Dangling<T> {}

pub fn double_free<T>(val: T) {
    let boxed = Box::new(val);
    let ptr = Box::into_raw(boxed);
    unsafe {
        drop(Box::from_raw(ptr));
        drop(Box::from_raw(ptr));
    }
}

pub fn leak<T>(val: T) -> *mut T {
    Box::into_raw(Box::new(val))
}

pub fn leak_ref<T>(val: T) -> &'static mut T {
    Box::leak(Box::new(val))
}

pub fn read_freed<T: Copy>(ptr: *mut T) -> T {
    unsafe { ptr::read(ptr) }
}

pub fn write_freed<T>(ptr: *mut T, val: T) {
    unsafe {
        ptr::write(ptr, val);
    }
}

pub fn alloc_garbage<T>() -> *mut T {
    let layout = Layout::new::<T>();
    unsafe { alloc(layout) as *mut T }
}

pub fn free<T>(ptr: *mut T) {
    let layout = Layout::new::<T>();
    unsafe {
        dealloc(ptr as *mut u8, layout);
    }
}

pub struct ArbitraryAccess;

impl ArbitraryAccess {
    pub fn read<T: Copy>(addr: usize) -> T {
        unsafe { ptr::read(addr as *const T) }
    }

    pub fn write<T>(addr: usize, val: T) {
        unsafe {
            ptr::write(addr as *mut T, val);
        }
    }
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub fn remove_segfaults() {
    unsafe {
        libc::sigaction(
            libc::SIGSEGV,
            &libc::sigaction {
                sa_sigaction: handler as *const () as usize,
                sa_mask: core::mem::zeroed::<libc::sigset_t>(),
                sa_flags: libc::SA_ONSTACK | libc::SA_SIGINFO,
                sa_restorer: None,
            },
            core::ptr::null_mut(),
        );
    }

    extern "C" fn handler(
        _signo: libc::c_int,
        info: *mut libc::siginfo_t,
        context: *mut libc::ucontext_t,
    ) {
        unsafe {
            let ctx = &mut *context;
            let info = &mut *info;
            let fault_addr = info.si_addr() as usize;

            let masked_off = fault_addr & (!0xFFF);
            let ret = libc::mmap(
                masked_off as *mut libc::c_void,
                4096,
                libc::PROT_WRITE | libc::PROT_READ,
                libc::MAP_ANONYMOUS | libc::MAP_FIXED,
                0,
                0,
            );

            if ret as isize != -1 {
                return;
            }

            let old_rip = ctx.uc_mcontext.gregs[libc::REG_RIP as usize] as usize;
            let array = core::slice::from_raw_parts(old_rip as *mut u8, 15);
            let mut out = lde::X64.iter(array, old_rip as u64);

            let Some((opcode, _)) = out.next() else {
                libc::abort();
            };

            ctx.uc_mcontext.gregs[libc::REG_RIP as usize] += opcode.len() as i64;
        }
    }
}
