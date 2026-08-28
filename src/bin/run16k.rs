#![no_std]
#![no_main]


use embedded_test_runner::embedded_utils::{EXIT_SUCCESS, exit, init_heap};
use cortex_m_rt::entry;



#[entry]
fn main() -> ! {
    init_heap();
    exit(EXIT_SUCCESS);
}
