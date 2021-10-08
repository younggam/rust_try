macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        unsafe {
            let b: $base = std::mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}
pub(crate) use offset_of;
