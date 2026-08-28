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
