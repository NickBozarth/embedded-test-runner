#[macro_export]
macro_rules! new_test {
    ($name:expr, $callback:expr) => (
        $crate::test_case::TestCase::new($name, $callback)
    );

    // Closures should not be accepted here
    ($callback:ident) => (
        $crate::test_case::TestCase::new(
            ::core::stringify!($callback),
            $callback
        )
    );
}




#[macro_export]
macro_rules! test_assert {
    ($conditional:expr) => {
        $crate::test_assert!(
            $conditional,
            ::core::concat!(
                "Assert failed condition [",
                ::core::stringify!($conditional),
                "]"
            )
        )
    };

    ($conditional:expr, $fail_msg:expr) => {
        if ($conditional) {
            $crate::test_case::TestResultStatus::Passed
        } else {
            $crate::test_case::TestResultStatus::Failed($fail_msg.into())
        }
    };
}

#[macro_export]
macro_rules! test_assert_eq {
    ($left:expr, $right:expr) => {
        $crate::test_assert!($left == $right)
    };

    ($left:expr, $right:expr, $fail_msg:expr) => {
        $crate::test_assert!($left == $right, $fail_msg)
    };
}

#[macro_export]
macro_rules! test_assert_ne {
    ($left:expr, $right:expr) => {
        $crate::test_assert!($left != $right)
    };

    ($left:expr, $right:expr, $fail_msg:expr) => {
        $crate::test_assert!($left != $right, $fail_msg)
    };
}
