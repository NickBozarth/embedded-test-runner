#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::debug::{EXIT_SUCCESS, ExitStatus};
use panic_halt as _;

fn exit(status: ExitStatus) -> ! {
    cortex_m_semihosting::debug::exit(status);
    loop {}
}


#[entry]
fn main() -> ! {
    exit(EXIT_SUCCESS);
}
