#![no_std]
#![no_main]


use core::num::ParseIntError;

use cortex_m_semihosting::hprintln;
use embedded_test_runner::{embedded_utils::{EXIT_SUCCESS, exit, init_heap}, new_test, test_case::{TestCase, TestResultStatus}};
use cortex_m_rt::entry;


fn test_me() {

}

fn test_me_result() -> Result<i32, &'static str> {
    Err("something went wrong")
}

fn test_me_test_result() -> TestResultStatus {
    TestResultStatus::Failed("nah thats not happening".into())
}


#[entry]
fn main() -> ! {
    init_heap();

    let tc1 = TestCase::new("test1", test_me);
    let tc2 = TestCase::new("test2", test_me_result);

    let tc3 = new_test!("test3", test_me);
    let tc4 = new_test!(test_me);
    let tc5 = new_test!(test_me_test_result);

    let res = tc5.run();
    hprintln!("{res}");
    let res = tc4.run();
    hprintln!("{res}");


    exit(EXIT_SUCCESS);
}
