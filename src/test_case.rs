use core::{error::Error, fmt::{Debug, Display}};

use alloc::{boxed::Box, string::{String, ToString}};



// Tests will only ever be called once
pub struct TestCase  {
    name: String,
    callback: Box<dyn FnOnce() -> TestResultStatus>
}


impl TestCase {
    pub fn new<F, R>(name: impl Into<String>, callback: F) -> Self
    where
        F: FnOnce() -> R + 'static,
        R: IntoTestResultStatus
    {
        Self {
            name: name.into(),
            callback: Box::new(|| callback().into_test_result())
        }
    }

    pub fn run(self) -> TestResult {
        TestResult {
            test_name: self.name,
            test_stat: (self.callback)()
        }
    }
}



// A return type that testable functions can return
pub struct TestResult {
    test_name: String,
    test_stat: TestResultStatus
}

impl Display for TestResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:\n {}", self.test_name, self.test_stat)
    }
}


pub enum TestResultStatus {
    Passed,
    Failed(String)
}

impl Display for TestResultStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Passed => write!(f, "✓ Passed"),
            Self::Failed(err) => write!(f, "✗ Failed: {err}")
        }
    }
}



// A trait to convert the output of functions into the same type
//  as to have the same trait bound for all testable functions
//  FnOnce() -> TestResult
pub trait IntoTestResultStatus {
    fn into_test_result(self) -> TestResultStatus;
}

impl IntoTestResultStatus for () {
    fn into_test_result(self) -> TestResultStatus {
        TestResultStatus::Passed
    }
}

impl<T, E> IntoTestResultStatus for Result<T, E>
where E: Into<String>
{
    fn into_test_result(self) -> TestResultStatus {
        match self {
            Ok(_) => TestResultStatus::Passed,
            Err(err) => TestResultStatus::Failed(err.into())
        }
    }
}

impl IntoTestResultStatus for TestResultStatus {
    fn into_test_result(self) -> TestResultStatus {
        self
    }
}
