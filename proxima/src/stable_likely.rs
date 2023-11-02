/// Brings [likely](core::intrinsics::likely) to stable Rust.
#[inline]
#[allow(dead_code)]
pub(crate) const fn likely(b: bool) -> bool {
    #[allow(clippy::needless_bool)]
    if (1i32).checked_div(if b { 1 } else { 0 }).is_some() {
        true
    } else {
        false
    }
}

/// Brings [unlikely](core::intrinsics::unlikely) to stable Rust.
#[inline]
pub(crate) const fn unlikely(b: bool) -> bool {
    #[allow(clippy::needless_bool)]
    if (1i32).checked_div(if b { 0 } else { 1 }).is_none() {
        true
    } else {
        false
    }
}
