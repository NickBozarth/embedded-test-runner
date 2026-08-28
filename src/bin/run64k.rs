#![no_std]
#![no_main]


use core::num::ParseIntError;

use cortex_m_semihosting::hprintln;
use embedded_test_runner::{embedded_utils::{EXIT_SUCCESS, exit, init_heap}, new_test, test_assert, test_assert_eq, test_assert_ne, test_case::{TestCase, TestResultStatus}};
use cortex_m_rt::entry;


fn test_me() {

}

fn test_me_result() -> Result<i32, &'static str> {
    Err("something went wrong")
}

fn test_me_test_result() -> TestResultStatus {
    let y = 10;
    let x = test_assert!(y == 2);
    let z = test_assert_ne!(10, 10);
    z
}


#[entry]
fn main() -> ! {
    init_heap();

    let tc1 = new_test!(test_me_test_result);
    hprintln!("{}", tc1.run());

    exit(EXIT_SUCCESS);
}
