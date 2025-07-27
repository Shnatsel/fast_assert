#[cfg(test)]
mod tests {
    use fast_assert::fast_assert;
    use std::panic::{self, PanicHookInfo};

    /// Verifies that the reported source code location for the panic is in THIS file,
    /// and not somewhere in fast_assert internals.
    /// Causes the process to abort with "panicked while panicking" message on failure.
    fn verify_source_code_location<F: FnOnce()>(f: F) {
        // Set a custom panic hook.
        let default_hook = panic::take_hook();
        panic::set_hook(Box::new(|info: &PanicHookInfo| {
            // Extract the location from PanicInfo.
            if let Some(location) = info.location() {
                // Verify that the reported location is THIS file,
                // not the internals of fast_assert, which would be useless
                assert_eq!(location.file(), "test_crate/src/fast_assert_macro_tests.rs");            
            }
        }));

        // Run the user's code.
        f();

        // Restore the default panic hook.
        panic::set_hook(default_hook);
    }

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

    #[test]
    #[should_panic]
    fn reported_location_simple() {
        verify_source_code_location(|| fast_assert!(100 < 0));
    }

    #[test]
    #[should_panic]
    fn reported_location_custom_message() {
        let x = 100;
        let y = 0;
        verify_source_code_location(|| {
            fast_assert!(
                x < y,
                "x ({}) should be less than y ({})", x, y
            );
        });
    }
}
