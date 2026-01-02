use core::mem;

pub fn yeet<A, B>(a: A) -> B {
    unsafe { mem::transmute_copy(&a) }
}

pub fn yeet_lossy<A, B>(a: A) -> B {
    unsafe {
        let ptr = &a as *const A as *const B;
        let result = ptr.read_unaligned();
        mem::forget(a);
        result
    }
}

pub fn view_as<T, U>(val: &T) -> &U {
    unsafe { &*(val as *const T as *const U) }
}

pub fn view_as_mut<T, U>(val: &mut T) -> &mut U {
    unsafe { &mut *(val as *mut T as *mut U) }
}

pub fn float_to_bits(f: f32) -> u32 {
    yeet(f)
}

pub fn bits_to_float(bits: u32) -> f32 {
    yeet(bits)
}

pub fn double_to_bits(f: f64) -> u64 {
    yeet(f)
}

pub fn bits_to_double(bits: u64) -> f64 {
    yeet(bits)
}

pub fn int_to_ptr<T>(addr: usize) -> *mut T {
    addr as *mut T
}

pub fn ptr_to_int<T>(ptr: *const T) -> usize {
    ptr as usize
}

pub fn as_bytes<T>(val: &T) -> &[u8] {
    unsafe { core::slice::from_raw_parts(val as *const T as *const u8, mem::size_of::<T>()) }
}

pub fn as_bytes_mut<T>(val: &mut T) -> &mut [u8] {
    unsafe { core::slice::from_raw_parts_mut(val as *mut T as *mut u8, mem::size_of::<T>()) }
}

pub fn from_bytes<T>(bytes: &[u8]) -> &T {
    assert!(
        bytes.len() >= mem::size_of::<T>(),
        "Not enough bytes for type"
    );
    unsafe { &*(bytes.as_ptr() as *const T) }
}

pub fn from_bytes_mut<T>(bytes: &mut [u8]) -> &mut T {
    assert!(
        bytes.len() >= mem::size_of::<T>(),
        "Not enough bytes for type"
    );
    unsafe { &mut *(bytes.as_mut_ptr() as *mut T) }
}

#[repr(transparent)]
pub struct FnData<F> {
    data: usize,
    _phantom: core::marker::PhantomData<F>,
}

impl<F> FnData<F> {
    pub fn new(f: F) -> Self
    where
        F: Copy,
    {
        FnData {
            data: unsafe { *(&f as *const F as *const usize) },
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn get(&self) -> F
    where
        F: Copy,
    {
        unsafe { *(&self.data as *const usize as *const F) }
    }

    pub fn addr(&self) -> usize {
        self.data
    }
}
