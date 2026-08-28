extern crate cortex_m;

use core::{mem::MaybeUninit, panic::PanicInfo};
use cortex_m_semihosting::debug::ExitStatus;
pub use cortex_m_semihosting::debug::{EXIT_FAILURE, EXIT_SUCCESS};
use embedded_alloc::LlffHeap;


#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();
pub fn init_heap() {
    // We dont quite use all of the ram for heap
    //  bc we still need room for static & stack
    #[cfg(feature =  "ram-16k")]
    const HEAP_SIZE: usize = 1024 * 10;
    #[cfg(feature = "ram-64k")]
    const HEAP_SIZE: usize = 1024 * 50;

    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    // This call is safe if size is valid
    unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE); }
}


pub fn exit(status: ExitStatus) -> ! {
    cortex_m_semihosting::debug::exit(status);
    loop {}
}


#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
