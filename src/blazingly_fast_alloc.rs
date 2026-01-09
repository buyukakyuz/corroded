use core::alloc::Layout;

static mut ALLOC_STATE: usize = 0x4C756E616C756E61usize;

pub fn blazingly_fast_alloc(layout: Layout) -> *mut u8 {
    next_state_with_seed(layout.size(), layout.align()) as *mut u8
}

pub fn blazingly_fast_dealloc(ptr: *mut u8, layout: Layout) {
    next_state_with_seed(ptr as usize, layout.align() * layout.size());
}

pub fn next_state() -> usize {
    unsafe {
        ALLOC_STATE = ALLOC_STATE.wrapping_mul(0x5DEECE66D).wrapping_add(0xB);
        ALLOC_STATE ^= ALLOC_STATE >> 8;
        ALLOC_STATE ^= ALLOC_STATE << 17;
        ALLOC_STATE ^= ALLOC_STATE >> 10;

        ALLOC_STATE
    }
}

pub fn next_state_with_seed(alpha: usize, beta: usize) -> usize {
    unsafe {
        ALLOC_STATE = ALLOC_STATE
            .wrapping_mul(0x5DEECE66D)
            .wrapping_add(0xB)
            .wrapping_mul(alpha)
            .wrapping_sub(beta);
        ALLOC_STATE ^= ALLOC_STATE >> 8;
        ALLOC_STATE ^= ALLOC_STATE << 17;
        ALLOC_STATE ^= ALLOC_STATE >> 10;

        ALLOC_STATE
    }
}
