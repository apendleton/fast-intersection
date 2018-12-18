#[macro_use]
extern crate cpp;

cpp!{{
    #include "SIMDCompressionAndIntersection/include/intersection.h"
}}

#[allow(non_snake_case)]
#[inline]
pub unsafe fn SIMDintersection(
        set1: *const u32,
        length1: usize,
        set2: *const u32,
        length2: usize,
        out: *mut u32
    ) -> usize {

    cpp!([
        set1 as "uint32_t*",
        length1 as "size_t",
        set2 as "uint32_t*",
        length2 as "size_t",
        out as "uint32_t*"
    ] -> usize as "size_t" {
            return SIMDCompressionLib::SIMDintersection(set1, length1, set2, length2, out);
    })
}

#[allow(non_snake_case)]
#[inline]
pub unsafe fn SIMDintersection_avx2(
        set1: *const u32,
        length1: usize,
        set2: *const u32,
        length2: usize,
        out: *mut u32
    ) -> usize {

    cpp!([
        set1 as "uint32_t*",
        length1 as "size_t",
        set2 as "uint32_t*",
        length2 as "size_t",
        out as "uint32_t*"
    ] -> usize as "size_t" {
            return SIMDCompressionLib::SIMDintersection_avx2(set1, length1, set2, length2, out);
    })
}
