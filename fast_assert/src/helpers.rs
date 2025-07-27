
/// This function is marked as `#[cold]` to hint to the compiler that it's
/// rarely executed. The compiler uses this to optimize the call site,
/// keeping the "hot path" (where the assertion succeeds) as lean as possible.
#[cold]
#[track_caller]
pub fn assert_failed_default(condition: &'static str) -> ! {
    panic!("assertion failed: {}", condition);
}

/// A cold function for assertions with custom messages.
///
/// This function is generic over a closure `F`.
/// `F: FnOnce()` means it accepts any closure that can be called once
/// and takes no arguments.
///
/// The panic logic is provided by the caller via this closure.
#[cold]
#[track_caller]
pub fn assert_failed_custom<F>(msg_fn: F)
where
    F: FnOnce(),
{
    // We simply call the closure, which contains the panic!.
    msg_fn();
}
