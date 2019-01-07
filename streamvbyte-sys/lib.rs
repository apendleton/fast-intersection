use std::mem::size_of;

extern {
    pub fn streamvbyte_encode(
        in_: *const u32,
        length: u32,
        out: *mut u8
    ) -> usize;
    pub fn streamvbyte_decode(
        in_: *const u8,
        out: *mut u32,
        length: u32,
    ) -> usize;
    pub fn streamvbyte_encode_0124(
        in_: *const u32,
        length: u32,
        out: *mut u8
    ) -> usize;
    pub fn streamvbyte_decode_0124(
        in_: *const u8,
        out: *mut u32,
        length: u32,
    ) -> usize;
    pub fn streamvbyte_delta_encode(
        in_: *const u32,
        length: u32,
        out: *mut u8,
        prev: u32,
    ) -> usize;
    pub fn streamvbyte_delta_decode(
        in_: *const u8,
        out: *mut u32,
        length: u32,
        prev: u32,
    ) -> usize;
}

// this is a port rather than a wrapper, because it's declared as inline on the C side
#[inline]
pub fn streamvbyte_max_compressedbytes(length: usize) -> usize {
    // number of control bytes:
    let cb = (length + 3) / 4;
    // maximum number of control bytes:
    let db = length * size_of::<u32>();
    cb + db
}
