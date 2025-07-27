#[doc(hidden)] // private implementation detail, required to make macros work from other crates
pub mod helpers;

/// A reimplementation of assert! that uses a closure to defer all
/// panic-related work, including argument creation, to the cold path.
#[macro_export]
macro_rules! fast_assert {
    // Rule 1: Handles calls with only a condition, like my_assert!(x == y).
    // It also accepts an optional trailing comma, like my_assert!(x == y,).
    ($cond:expr $(,)?) => {
        if !$cond {
            // If the condition is false, panic with a default message.
            // The stringify! macro converts the expression `$cond` into a string literal,
            // so the error message includes the exact code that failed.
            $crate::helpers::assert_failed_default(stringify!($cond));
        }
    };
    // Rule 2: Handles calls with a condition and a custom message,
    // like my_assert!(x == y, "x should be equal to y, but was {}", x).
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            // We pass a closure to the cold function.
            // No code inside this closure will be generated in the hot path.
            $crate::helpers::assert_failed_custom(|| {
                panic!($($arg)+);
            });
        }
    };
}

/// We only run basic sanity checks here. The really interesting tests are in a separate crate in this workspace.
/// This is because getting macros to work when instantiated in the same file is easy,
/// but getting them to work across crates is harder.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holds() {
        fast_assert!(0 < 100);
    }

    #[test]
    #[should_panic]
    fn fails() {
        fast_assert!(100 < 0);
    }

    #[test]
    fn holds_custom_message() {
        let x = 0;
        let y = 100;
        fast_assert!(
            x < y,
            "x ({}) should be less than y ({})", x, y
        );
    }

    #[test]
    #[should_panic]
    fn fails_custom_message() {
        let x = 100;
        let y = 0;
        fast_assert!(
            x < y,
            "x ({}) should be less than y ({})", x, y
        );
    }
}
