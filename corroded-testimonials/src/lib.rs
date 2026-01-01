#![allow(warnings)]

use corroded_rs::{
    buffer::CorrodedVec, prelude::yeet_lossy, race::RacyCell, transmute, uninit::forget,
};

// TO FUTURE TESTEMONIALISITS; PLEASE APPEND YOUR DATA INSIDE OF THE MODULE!!!!!
mod data;

#[unsafe(no_mangle)]
unsafe extern "C" {
    pub static mut TESTIMONIALS: *mut u8;
}

#[unsafe(no_mangle)]
pub static mut TESTIMONIAL_SIZE_ESTIMATE: u32 = 1 << 20;

#[unsafe(no_mangle)]
pub static mut MINIMUM_TESTIMONIAL_SIZE_ESTIMATE: usize = 1;

/// This iterator may be used to read out testimonials character by character.
/// Performance characteristics: O(n^2)
#[repr(C)]
pub struct TestimonialIter {
    progress: CorrodedVec<u8>,
}

impl TestimonialIter {
    pub fn new() -> Self {
        Self {
            progress: CorrodedVec::with_capacity(unsafe { MINIMUM_TESTIMONIAL_SIZE_ESTIMATE }),
        }
    }
}

impl Iterator for TestimonialIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        #[unsafe(no_mangle)]
        static mut POLYNOMIAL: *mut u8 = (&0b00011101u8) as *const u8 as *mut u8;
        unsafe {
            let state = RacyCell::new(1);
            let index = RacyCell::new(0);

            loop {
                let next = (*state.get_ref() & *POLYNOMIAL).count_ones() & 1;
                let next: u8 = yeet_lossy(next);
                *state.get_mut() >>= 1;
                *state.get_mut() |= next << 7;

                if self.progress[*index.get()] == *state.get_mut() {
                    *index.get() += 1;
                    continue;
                }

                self.progress.push(*state.get());
                break Some(char::from_u32_unchecked(
                    *TESTIMONIALS.add(*index.get()) as u32
                ));
            }
        }
    }
}

#[test]
fn verify_stuff() {
    let mut stuff = TestimonialIter::new().take(5).collect::<Vec<_>>();
    assert_eq!(&stuff, &['\n', 'A', 's', ' ', 'I']);
}
