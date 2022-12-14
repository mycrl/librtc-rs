use libc::*;

#[repr(i32)]
pub enum FilterMode {
    FilterNone = 0,     // Point sample; Fastest.
    FilterLinear = 1,   // Filter horizontally only.
    FilterBilinear = 2, // Faster than box, but lower quality scaling down.
    FilterBox = 3,      // Highest quality.
}

#[repr(i32)]
pub enum RotationMode {
    Rotate0 = 0,     // No rotation.
    Rotate90 = 90,   // Rotate 90 degrees clockwise.
    Rotate180 = 180, // Rotate 180 degrees.
    Rotate270 = 270, // Rotate 270 degrees clockwise.
}

extern "C" {
    // Compute a hash for specified memory. Seed of 5381 recommended.
    #[link_name = "HashDjb2"]
    pub fn hash_djb2(src: *const u8, count: u64, seed: u32) -> u32;

    // Hamming Distance
    #[link_name = "ComputeHammingDistance"]
    pub fn compute_hamming_distance(src_a: *const u8, src_b: *const u8, count: c_int) -> u64;

    // Scan an opaque argb image and return fourcc based on alpha offset.
    // Returns FOURCC_ARGB, FOURCC_BGRA, or 0 if unknown.
    #[link_name = "ARGBDetect"]
    pub fn argb_detect(argb: *const u8, stride_argb: c_int, width: c_int, height: c_int) -> u32;

    // Sum Square Error - used to compute Mean Square Error or PSNR.
    #[link_name = "ComputeSumSquareError"]
    pub fn compute_sum_square_error(src_a: *const u8, src_b: *const u8, count: c_int) -> u64;

    #[link_name = "ComputeSumSquareErrorPlane"]
    pub fn compute_sum_square_error_plane(src_a: *const u8,
                                          stride_a: c_int,
                                          src_b: *const u8,
                                          stride_b: c_int,
                                          width: c_int,
                                          height: c_int)
                                          -> u64;

    #[link_name = "SumSquareErrorToPsnr"]
    pub fn sum_square_error_to_psnr(sse: u64, count: u64) -> c_double;

    #[link_name = "CalcFramePsnr"]
    pub fn calc_frame_psnr(src_a: *const u8,
                           stride_a: c_int,
                           src_b: *const u8,
                           stride_b: c_int,
                           width: c_int,
                           height: c_int)
                           -> c_double;

    #[link_name = "I420Psnr"]
    pub fn i420_psnr(src_y_a: *const u8,
                     stride_y_a: c_int,
                     src_u_a: *const u8,
                     stride_u_a: c_int,
                     src_v_a: *const u8,
                     stride_v_a: c_int,
                     src_y_b: *const u8,
                     stride_y_b: c_int,
                     src_u_b: *const u8,
                     stride_u_b: c_int,
                     src_v_b: *const u8,
                     stride_v_b: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_double;

    #[link_name = "CalcFrameSsim"]
    pub fn calc_frame_ssim(src_a: *const u8,
                           stride_a: c_int,
                           src_b: *const u8,
                           stride_b: c_int,
                           width: c_int,
                           height: c_int)
                           -> c_double;

    #[link_name = "I420Ssim"]
    pub fn i420_ssim(src_y_a: *const u8,
                     stride_y_a: c_int,
                     src_u_a: *const u8,
                     stride_u_a: c_int,
                     src_v_a: *const u8,
                     stride_v_a: c_int,
                     src_y_b: *const u8,
                     stride_y_b: c_int,
                     src_u_b: *const u8,
                     stride_u_b: c_int,
                     src_v_b: *const u8,
                     stride_v_b: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_double;

    #[link_name = "ARGBCopy"]
    pub fn argb_copy(src_argb: *const u8,
                     src_stride_argb: c_int,
                     dst_argb: *const u8,
                     dst_stride_argb: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "I420ToARGB"]
    pub fn i420_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToABGR"]
    pub fn i420_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "J420ToARGB"]
    pub fn j420_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "J420ToABGR"]
    pub fn j420_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H420ToARGB"]
    pub fn h420_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H420ToABGR"]
    pub fn h420_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U420ToARGB"]
    pub fn u420_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U420ToABGR"]
    pub fn u420_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I422ToARGB"]
    pub fn i422_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I422ToABGR"]
    pub fn i422_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "J422ToARGB"]
    pub fn j422_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "J422ToABGR"]
    pub fn j422_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H422ToARGB"]
    pub fn h422_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H422ToABGR"]
    pub fn h422_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U422ToARGB"]
    pub fn u422_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U422ToABGR"]
    pub fn u422_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I444ToARGB"]
    pub fn i444_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I444ToABGR"]
    pub fn i444_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "J444ToARGB"]
    pub fn j444_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "J444ToABGR"]
    pub fn j444_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H444ToARGB"]
    pub fn h444_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H444ToABGR"]
    pub fn h444_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U444ToARGB"]
    pub fn u444_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U444ToABGR"]
    pub fn u444_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I444ToRGB24"]
    pub fn i444_to_rgb24(src_y: *const u8,
                         src_stride_y: c_int,
                         src_u: *const u8,
                         src_stride_u: c_int,
                         src_v: *const u8,
                         src_stride_v: c_int,
                         dst_rgb24: *const u8,
                         dst_stride_rgb24: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "I444ToRAW"]
    pub fn i444_to_raw(src_y: *const u8,
                       src_stride_y: c_int,
                       src_u: *const u8,
                       src_stride_u: c_int,
                       src_v: *const u8,
                       src_stride_v: c_int,
                       dst_raw: *const u8,
                       dst_stride_raw: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "I010ToARGB"]
    pub fn i010_to_argb(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I010ToABGR"]
    pub fn i010_to_abgr(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H010ToARGB"]
    pub fn h010_to_argb(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H010ToABGR"]
    pub fn h010_to_abgr(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U010ToARGB"]
    pub fn u010_to_argb(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U010ToABGR"]
    pub fn u010_to_abgr(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I210ToARGB"]
    pub fn i210_to_argb(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I210ToABGR"]
    pub fn i210_to_abgr(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H210ToARGB"]
    pub fn h210_to_argb(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H210ToABGR"]
    pub fn h210_to_abgr(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U210ToARGB"]
    pub fn u210_to_argb(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U210ToABGR"]
    pub fn u210_to_abgr(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420AlphaToARGB"]
    pub fn i420_alpha_to_argb(src_y: *const u8,
                              src_stride_y: c_int,
                              src_u: *const u8,
                              src_stride_u: c_int,
                              src_v: *const u8,
                              src_stride_v: c_int,
                              src_a: *const u8,
                              src_stride_a: c_int,
                              dst_argb: *const u8,
                              dst_stride_argb: c_int,
                              width: c_int,
                              height: c_int,
                              attenuate: c_int)
                              -> c_int;

    #[link_name = "I420AlphaToABGR"]
    pub fn i420_alpha_to_abgr(src_y: *const u8,
                              src_stride_y: c_int,
                              src_u: *const u8,
                              src_stride_u: c_int,
                              src_v: *const u8,
                              src_stride_v: c_int,
                              src_a: *const u8,
                              src_stride_a: c_int,
                              dst_abgr: *const u8,
                              dst_stride_abgr: c_int,
                              width: c_int,
                              height: c_int,
                              attenuate: c_int)
                              -> c_int;

    #[link_name = "I422AlphaToARGB"]
    pub fn i422_alpha_to_argb(src_y: *const u8,
                              src_stride_y: c_int,
                              src_u: *const u8,
                              src_stride_u: c_int,
                              src_v: *const u8,
                              src_stride_v: c_int,
                              src_a: *const u8,
                              src_stride_a: c_int,
                              dst_argb: *const u8,
                              dst_stride_argb: c_int,
                              width: c_int,
                              height: c_int,
                              attenuate: c_int)
                              -> c_int;

    #[link_name = "I422AlphaToABGR"]
    pub fn i422_alpha_to_abgr(src_y: *const u8,
                              src_stride_y: c_int,
                              src_u: *const u8,
                              src_stride_u: c_int,
                              src_v: *const u8,
                              src_stride_v: c_int,
                              src_a: *const u8,
                              src_stride_a: c_int,
                              dst_abgr: *const u8,
                              dst_stride_abgr: c_int,
                              width: c_int,
                              height: c_int,
                              attenuate: c_int)
                              -> c_int;

    #[link_name = "I444AlphaToARGB"]
    pub fn i444_alpha_to_argb(src_y: *const u8,
                              src_stride_y: c_int,
                              src_u: *const u8,
                              src_stride_u: c_int,
                              src_v: *const u8,
                              src_stride_v: c_int,
                              src_a: *const u8,
                              src_stride_a: c_int,
                              dst_argb: *const u8,
                              dst_stride_argb: c_int,
                              width: c_int,
                              height: c_int,
                              attenuate: c_int)
                              -> c_int;

    #[link_name = "I444AlphaToABGR"]
    pub fn i444_alpha_to_abgr(src_y: *const u8,
                              src_stride_y: c_int,
                              src_u: *const u8,
                              src_stride_u: c_int,
                              src_v: *const u8,
                              src_stride_v: c_int,
                              src_a: *const u8,
                              src_stride_a: c_int,
                              dst_abgr: *const u8,
                              dst_stride_abgr: c_int,
                              width: c_int,
                              height: c_int,
                              attenuate: c_int)
                              -> c_int;

    #[link_name = "I400ToARGB"]
    pub fn i400_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "J400ToARGB"]
    pub fn j400_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "NV12ToARGB"]
    pub fn nv12_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_uv: *const u8,
                        src_stride_uv: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "NV21ToARGB"]
    pub fn nv21_to_argb(src_y: *const u8,
                        src_stride_y: c_int,
                        src_vu: *const u8,
                        src_stride_vu: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "NV12ToABGR"]
    pub fn nv12_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_uv: *const u8,
                        src_stride_uv: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "NV21ToABGR"]
    pub fn nv21_to_abgr(src_y: *const u8,
                        src_stride_y: c_int,
                        src_vu: *const u8,
                        src_stride_vu: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "NV12ToRGB24"]
    pub fn nv12_to_rgb24(src_y: *const u8,
                         src_stride_y: c_int,
                         src_uv: *const u8,
                         src_stride_uv: c_int,
                         dst_rgb24: *const u8,
                         dst_stride_rgb24: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "NV21ToRGB24"]
    pub fn nv21_to_rgb24(src_y: *const u8,
                         src_stride_y: c_int,
                         src_vu: *const u8,
                         src_stride_vu: c_int,
                         dst_rgb24: *const u8,
                         dst_stride_rgb24: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "NV21ToYUV24"]
    pub fn nv21_to_yuv24(src_y: *const u8,
                         src_stride_y: c_int,
                         src_vu: *const u8,
                         src_stride_vu: c_int,
                         dst_yuv24: *const u8,
                         dst_stride_yuv24: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "NV12ToRAW"]
    pub fn nv12_to_raw(src_y: *const u8,
                       src_stride_y: c_int,
                       src_uv: *const u8,
                       src_stride_uv: c_int,
                       dst_raw: *const u8,
                       dst_stride_raw: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "NV21ToRAW"]
    pub fn nv21_to_raw(src_y: *const u8,
                       src_stride_y: c_int,
                       src_vu: *const u8,
                       src_stride_vu: c_int,
                       dst_raw: *const u8,
                       dst_stride_raw: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "YUY2ToARGB"]
    pub fn yuy2_to_argb(src_yuy2: *const u8,
                        src_stride_yuy2: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "UYVYToARGB"]
    pub fn uyvy_to_argb(src_uyvy: *const u8,
                        src_stride_uyvy: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I010ToAR30"]
    pub fn i010_to_ar30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H010ToAR30"]
    pub fn h010_to_ar30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I010ToAB30"]
    pub fn i010_to_ab30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ab30: *const u8,
                        dst_stride_ab30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H010ToAB30"]
    pub fn h010_to_ab30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ab30: *const u8,
                        dst_stride_ab30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U010ToAR30"]
    pub fn u010_to_ar30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U010ToAB30"]
    pub fn u010_to_ab30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ab30: *const u8,
                        dst_stride_ab30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I210ToAR30"]
    pub fn i210_to_ar30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I210ToAB30"]
    pub fn i210_to_ab30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ab30: *const u8,
                        dst_stride_ab30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H210ToAR30"]
    pub fn h210_to_ar30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H210ToAB30"]
    pub fn h210_to_ab30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ab30: *const u8,
                        dst_stride_ab30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U210ToAR30"]
    pub fn u210_to_ar30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "U210ToAB30"]
    pub fn u210_to_ab30(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_ab30: *const u8,
                        dst_stride_ab30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "BGRAToARGB"]
    pub fn bgra_to_argb(src_bgra: *const u8,
                        src_stride_bgra: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ABGRToARGB"]
    pub fn abgr_to_argb(src_abgr: *const u8,
                        src_stride_abgr: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "RGBAToARGB"]
    pub fn rgba_to_argb(src_rgba: *const u8,
                        src_stride_rgba: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "RGB24ToARGB"]
    pub fn rgb24_to_argb(src_rgb24: *const u8,
                         src_stride_rgb24: c_int,
                         dst_argb: *const u8,
                         dst_stride_argb: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "RAWToARGB"]
    pub fn raw_to_argb(src_raw: *const u8,
                       src_stride_raw: c_int,
                       dst_argb: *const u8,
                       dst_stride_argb: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "RAWToRGBA"]
    pub fn raw_to_rgba(src_raw: *const u8,
                       src_stride_raw: c_int,
                       dst_rgba: *const u8,
                       dst_stride_rgba: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "RGB565ToARGB"]
    pub fn rgb565_to_argb(src_rgb565: *const u8,
                          src_stride_rgb565: c_int,
                          dst_argb: *const u8,
                          dst_stride_argb: c_int,
                          width: c_int,
                          height: c_int)
                          -> c_int;

    #[link_name = "ARGB1555ToARGB"]
    pub fn argb1555_to_argb(src_argb1555: *const u8,
                            src_stride_argb1555: c_int,
                            dst_argb: *const u8,
                            dst_stride_argb: c_int,
                            width: c_int,
                            height: c_int)
                            -> c_int;

    #[link_name = "ARGB4444ToARGB"]
    pub fn argb4444_to_argb(src_argb4444: *const u8,
                            src_stride_argb4444: c_int,
                            dst_argb: *const u8,
                            dst_stride_argb: c_int,
                            width: c_int,
                            height: c_int)
                            -> c_int;

    #[link_name = "AR30ToARGB"]
    pub fn ar30_to_argb(src_ar30: *const u8,
                        src_stride_ar30: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AR30ToABGR"]
    pub fn ar30_to_abgr(src_ar30: *const u8,
                        src_stride_ar30: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AR30ToAB30"]
    pub fn ar30_to_ab30(src_ar30: *const u8,
                        src_stride_ar30: c_int,
                        dst_ab30: *const u8,
                        dst_stride_ab30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AR64ToARGB"]
    pub fn ar64_to_argb(src_ar64: *const u16,
                        src_stride_ar64: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AB64ToARGB"]
    pub fn ab60_to_argb(src_ab64: *const u16,
                        src_stride_ab64: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AR64ToAB64"]
    pub fn ar64_to_ab60(src_ar64: *const u16,
                        src_stride_ar64: c_int,
                        dst_ab64: *const u16,
                        dst_stride_ab64: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "MJPGToARGB"]
    pub fn mjpg_to_argb(sample: *const u8,
                        sample_size: usize,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        src_width: c_int,
                        src_height: c_int,
                        dst_width: c_int,
                        dst_height: c_int)
                        -> c_int;

    #[link_name = "Android420ToARGB"]
    pub fn android420_to_argb(src_y: *const u8,
                              src_stride_y: c_int,
                              src_u: *const u8,
                              src_stride_u: c_int,
                              src_v: *const u8,
                              src_stride_v: c_int,
                              src_pixel_stride_uv: c_int,
                              dst_argb: *const u8,
                              dst_stride_argb: c_int,
                              width: c_int,
                              height: c_int)
                              -> c_int;

    #[link_name = "Android420ToABGR"]
    pub fn android420_to_abgr(src_y: *const u8,
                              src_stride_y: c_int,
                              src_u: *const u8,
                              src_stride_u: c_int,
                              src_v: *const u8,
                              src_stride_v: c_int,
                              src_pixel_stride_uv: c_int,
                              dst_abgr: *const u8,
                              dst_stride_abgr: c_int,
                              width: c_int,
                              height: c_int)
                              -> c_int;

    #[link_name = "NV12ToRGB565"]
    pub fn nv12_to_rgb565(src_y: *const u8,
                          src_stride_y: c_int,
                          src_uv: *const u8,
                          src_stride_uv: c_int,
                          dst_rgb565: *const u8,
                          dst_stride_rgb565: c_int,
                          width: c_int,
                          height: c_int)
                          -> c_int;

    #[link_name = "I422ToBGRA"]
    pub fn i422_to_bgra(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_bgra: *const u8,
                        dst_stride_bgra: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I422ToRGBA"]
    pub fn i422_to_rgba(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_rgba: *const u8,
                        dst_stride_rgba: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToBGRA"]
    pub fn i420_to_bgra(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_bgra: *const u8,
                        dst_stride_bgra: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToRGBA"]
    pub fn i420_to_rgba(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_rgba: *const u8,
                        dst_stride_rgba: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToRGB24"]
    pub fn i420_to_rgb24(src_y: *const u8,
                         src_stride_y: c_int,
                         src_u: *const u8,
                         src_stride_u: c_int,
                         src_v: *const u8,
                         src_stride_v: c_int,
                         dst_rgb24: *const u8,
                         dst_stride_rgb24: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "I420ToRAW"]
    pub fn i420_to_raw(src_y: *const u8,
                       src_stride_y: c_int,
                       src_u: *const u8,
                       src_stride_u: c_int,
                       src_v: *const u8,
                       src_stride_v: c_int,
                       dst_raw: *const u8,
                       dst_stride_raw: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "H420ToRGB24"]
    pub fn h420_to_rgb24(src_y: *const u8,
                         src_stride_y: c_int,
                         src_u: *const u8,
                         src_stride_u: c_int,
                         src_v: *const u8,
                         src_stride_v: c_int,
                         dst_rgb24: *const u8,
                         dst_stride_rgb24: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "H420ToRAW"]
    pub fn h420_to_raw(src_y: *const u8,
                       src_stride_y: c_int,
                       src_u: *const u8,
                       src_stride_u: c_int,
                       src_v: *const u8,
                       src_stride_v: c_int,
                       dst_raw: *const u8,
                       dst_stride_raw: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "J420ToRGB24"]
    pub fn j420_to_rgb24(src_y: *const u8,
                         src_stride_y: c_int,
                         src_u: *const u8,
                         src_stride_u: c_int,
                         src_v: *const u8,
                         src_stride_v: c_int,
                         dst_rgb24: *const u8,
                         dst_stride_rgb24: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "J420ToRAW"]
    pub fn j420_to_raw(src_y: *const u8,
                       src_stride_y: c_int,
                       src_u: *const u8,
                       src_stride_u: c_int,
                       src_v: *const u8,
                       src_stride_v: c_int,
                       dst_raw: *const u8,
                       dst_stride_raw: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "I422ToRGB24"]
    pub fn i422_to_rgb24(src_y: *const u8,
                         src_stride_y: c_int,
                         src_u: *const u8,
                         src_stride_u: c_int,
                         src_v: *const u8,
                         src_stride_v: c_int,
                         dst_rgb24: *const u8,
                         dst_stride_rgb24: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "I422ToRAW"]
    pub fn i422_to_raw(src_y: *const u8,
                       src_stride_y: c_int,
                       src_u: *const u8,
                       src_stride_u: c_int,
                       src_v: *const u8,
                       src_stride_v: c_int,
                       dst_raw: *const u8,
                       dst_stride_raw: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "I420ToRGB565"]
    pub fn i420_to_rgb565(src_y: *const u8,
                          src_stride_y: c_int,
                          src_u: *const u8,
                          src_stride_u: c_int,
                          src_v: *const u8,
                          src_stride_v: c_int,
                          dst_rgb565: *const u8,
                          dst_stride_rgb565: c_int,
                          width: c_int,
                          height: c_int)
                          -> c_int;

    #[link_name = "J420ToRGB565"]
    pub fn j420_to_rgb565(src_y: *const u8,
                          src_stride_y: c_int,
                          src_u: *const u8,
                          src_stride_u: c_int,
                          src_v: *const u8,
                          src_stride_v: c_int,
                          dst_rgb565: *const u8,
                          dst_stride_rgb565: c_int,
                          width: c_int,
                          height: c_int)
                          -> c_int;

    #[link_name = "H420ToRGB565"]
    pub fn h420_to_rgb565(src_y: *const u8,
                          src_stride_y: c_int,
                          src_u: *const u8,
                          src_stride_u: c_int,
                          src_v: *const u8,
                          src_stride_v: c_int,
                          dst_rgb565: *const u8,
                          dst_stride_rgb565: c_int,
                          width: c_int,
                          height: c_int)
                          -> c_int;

    #[link_name = "I422ToRGB565"]
    pub fn i422_to_rgb565(src_y: *const u8,
                          src_stride_y: c_int,
                          src_u: *const u8,
                          src_stride_u: c_int,
                          src_v: *const u8,
                          src_stride_v: c_int,
                          dst_rgb565: *const u8,
                          dst_stride_rgb565: c_int,
                          width: c_int,
                          height: c_int)
                          -> c_int;

    #[link_name = "I420ToRGB565Dither"]
    pub fn i420_to_rgb565_dither(src_y: *const u8,
                                 src_stride_y: c_int,
                                 src_u: *const u8,
                                 src_stride_u: c_int,
                                 src_v: *const u8,
                                 src_stride_v: c_int,
                                 dst_rgb565: *const u8,
                                 dst_stride_rgb565: c_int,
                                 dither4x4: *const u8,
                                 width: c_int,
                                 height: c_int)
                                 -> c_int;

    #[link_name = "I420ToARGB1555"]
    pub fn i420_to_argb1555(src_y: *const u8,
                            src_stride_y: c_int,
                            src_u: *const u8,
                            src_stride_u: c_int,
                            src_v: *const u8,
                            src_stride_v: c_int,
                            dst_argb1555: *const u8,
                            dst_stride_argb1555: c_int,
                            width: c_int,
                            height: c_int)
                            -> c_int;

    #[link_name = "I420ToARGB4444"]
    pub fn i420_to_argb4444(src_y: *const u8,
                            src_stride_y: c_int,
                            src_u: *const u8,
                            src_stride_u: c_int,
                            src_v: *const u8,
                            src_stride_v: c_int,
                            dst_argb4444: *const u8,
                            dst_stride_argb4444: c_int,
                            width: c_int,
                            height: c_int)
                            -> c_int;

    #[link_name = "I420ToAR30"]
    pub fn i420_to_ar30(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToAB30"]
    pub fn i420_to_ab30(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_ab30: *const u8,
                        dst_stride_ab30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H420ToAR30"]
    pub fn h420_to_ar30(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H420ToAB30"]
    pub fn h420_to_ab30(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_ab30: *const u8,
                        dst_stride_ab30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ConvertToARGB"]
    pub fn convert_to_argb(sample: *const u8,
                           sample_size: usize,
                           dst_argb: *const u8,
                           dst_stride_argb: c_int,
                           crop_x: c_int,
                           crop_y: c_int,
                           src_width: c_int,
                           src_height: c_int,
                           crop_width: c_int,
                           crop_height: c_int,
                           rotation: RotationMode,
                           fourcc: u32)
                           -> c_int;

    #[link_name = "ARGBToBGRA"]
    pub fn argb_to_bgra(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_bgra: *const u8,
                        dst_stride_bgra: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToABGR"]
    pub fn argb_to_abgr(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToRGBA"]
    pub fn argb_to_rgba(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_rgba: *const u8,
                        dst_stride_rgba: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ABGRToAR30"]
    pub fn abgr_to_ar30(src_abgr: *const u8,
                        src_stride_abgr: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToAR30"]
    pub fn argb_to_ar30(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToRGB24"]
    pub fn argb_to_rgb24(src_argb: *const u8,
                         src_stride_argb: c_int,
                         dst_rgb24: *const u8,
                         dst_stride_rgb24: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "ARGBToRAW"]
    pub fn argb_to_raw(src_argb: *const u8,
                       src_stride_argb: c_int,
                       dst_raw: *const u8,
                       dst_stride_raw: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "ARGBToRGB565"]
    pub fn argb_to_rgb565(src_argb: *const u8,
                          src_stride_argb: c_int,
                          dst_rgb565: *const u8,
                          dst_stride_rgb565: c_int,
                          width: c_int,
                          height: c_int)
                          -> c_int;

    #[link_name = "ARGBToRGB565Dither"]
    pub fn argb_to_rgb565_dither(src_argb: *const u8,
                                 src_stride_argb: c_int,
                                 dst_rgb565: *const u8,
                                 dst_stride_rgb565: c_int,
                                 dither4x4: *const u8,
                                 width: c_int,
                                 height: c_int)
                                 -> c_int;

    #[link_name = "ARGBToARGB1555"]
    pub fn argb_to_argb1555(src_argb: *const u8,
                            src_stride_argb: c_int,
                            dst_argb1555: *const u8,
                            dst_stride_argb1555: c_int,
                            width: c_int,
                            height: c_int)
                            -> c_int;

    #[link_name = "ARGBToARGB4444"]
    pub fn argb_to_argb4444(src_argb: *const u8,
                            src_stride_argb: c_int,
                            dst_argb4444: *const u8,
                            dst_stride_argb4444: c_int,
                            width: c_int,
                            height: c_int)
                            -> c_int;

    #[link_name = "ARGBToI444"]
    pub fn argb_to_i444(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToAR64"]
    pub fn argb_to_ar64(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_ar64: *const u16,
                        dst_stride_ar64: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToAB64"]
    pub fn argb_to_ab60(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_ab64: *const u16,
                        dst_stride_ab64: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToI422"]
    pub fn argb_to_i422(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToI420"]
    pub fn argb_to_i420(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToJ420"]
    pub fn argb_to_j420(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_yj: *const u8,
                        dst_stride_yj: c_int,
                        dst_uj: *const u8,
                        dst_stride_uj: c_int,
                        dst_vj: *const u8,
                        dst_stride_vj: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToJ422"]
    pub fn argb_to_j422(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_yj: *const u8,
                        dst_stride_yj: c_int,
                        dst_uj: *const u8,
                        dst_stride_uj: c_int,
                        dst_vj: *const u8,
                        dst_stride_vj: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToJ400"]
    pub fn argb_to_j400(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_yj: *const u8,
                        dst_stride_yj: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ABGRToJ420"]
    pub fn abgr_to_j420(src_abgr: *const u8,
                        src_stride_abgr: c_int,
                        dst_yj: *const u8,
                        dst_stride_yj: c_int,
                        dst_uj: *const u8,
                        dst_stride_uj: c_int,
                        dst_vj: *const u8,
                        dst_stride_vj: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ABGRToJ422"]
    pub fn abgr_to_j422(src_abgr: *const u8,
                        src_stride_abgr: c_int,
                        dst_yj: *const u8,
                        dst_stride_yj: c_int,
                        dst_uj: *const u8,
                        dst_stride_uj: c_int,
                        dst_vj: *const u8,
                        dst_stride_vj: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ABGRToJ400"]
    pub fn abgr_to_j400(src_abgr: *const u8,
                        src_stride_abgr: c_int,
                        dst_yj: *const u8,
                        dst_stride_yj: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "RGBAToJ400"]
    pub fn rgba_to_j400(src_rgba: *const u8,
                        src_stride_rgba: c_int,
                        dst_yj: *const u8,
                        dst_stride_yj: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToI400"]
    pub fn argb_to_i400(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToG"]
    pub fn argb_to_g(src_argb: *const u8,
                     src_stride_argb: c_int,
                     dst_g: *const u8,
                     dst_stride_g: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "ARGBToNV12"]
    pub fn argb_to_nv12(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_uv: *const u8,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToNV21"]
    pub fn argb_to_nv21(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_vu: *const u8,
                        dst_stride_vu: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ABGRToNV12"]
    pub fn abgr_to_nv12(src_abgr: *const u8,
                        src_stride_abgr: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_uv: *const u8,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ABGRToNV21"]
    pub fn abgr_to_nv21(src_abgr: *const u8,
                        src_stride_abgr: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_vu: *const u8,
                        dst_stride_vu: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToYUY2"]
    pub fn argb_to_yuy2(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_yuy2: *const u8,
                        dst_stride_yuy2: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToUYVY"]
    pub fn argb_to_uyvy(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_uyvy: *const u8,
                        dst_stride_uyvy: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "RAWToJNV21"]
    pub fn raw_to_j_nv21(src_raw: *const u8,
                         src_stride_raw: c_int,
                         dst_y: *const u8,
                         dst_stride_y: c_int,
                         dst_vu: *const u8,
                         dst_stride_vu: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "I420ToI010"]
    pub fn i420_to_i010(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToI012"]
    pub fn i420_to_i012(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToI422"]
    pub fn i420_to_i422(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I420 to I444.
    #[link_name = "I420ToI444"]
    pub fn i420_to_i444(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I400Copy"]
    pub fn i400_copy(src_y: *const u8,
                     src_stride_y: c_int,
                     dst_y: *const u8,
                     dst_stride_y: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "I420ToNV12"]
    pub fn i420_to_nv12(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_uv: *const u8,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToNV21"]
    pub fn i420_to_nv21(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_vu: *const u8,
                        dst_stride_vu: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToYUY2"]
    pub fn i420_to_yuy2(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_yuy2: *const u8,
                        dst_stride_yuy2: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToUYVY"]
    pub fn i420_to_uyvy(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_uyvy: *const u8,
                        dst_stride_uyvy: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ConvertFromI420"]
    pub fn convert_from_i420(y: *const u8,
                             y_stride: c_int,
                             u: *const u8,
                             u_stride: c_int,
                             v: *const u8,
                             v_stride: c_int,
                             dst_sample: *const u8,
                             dst_sample_stride: c_int,
                             width: c_int,
                             height: c_int,
                             fourcc: u32)
                             -> c_int;

    // Convert I444 to I420.
    #[link_name = "I444ToI420"]
    pub fn i444_to_i420(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I444 to NV12.
    #[link_name = "I444ToNV12"]
    pub fn i444_to_nv12(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_uv: *const u8,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I444 to NV21.
    #[link_name = "I444ToNV21"]
    pub fn i444_to_nv21(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_vu: *const u8,
                        dst_stride_vu: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I422 to I420.
    #[link_name = "I422ToI420"]
    pub fn i422_to_i420(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I422 to I444.
    #[link_name = "I422ToI444"]
    pub fn i422_to_i444(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I422 to I210.
    #[link_name = "I422ToI210"]
    pub fn i422_to_i210(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert MM21 to NV12.
    #[link_name = "MM21ToNV12"]
    pub fn mm21_to_nv12(src_y: *const u8,
                        src_stride_y: c_int,
                        src_uv: *const u8,
                        src_stride_uv: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_uv: *const u8,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert MM21 to I420.
    #[link_name = "MM21ToI420"]
    pub fn mm21_to_i420(src_y: *const u8,
                        src_stride_y: c_int,
                        src_uv: *const u8,
                        src_stride_uv: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert MM21 to YUY2
    #[link_name = "MM21ToYUY2"]
    pub fn mm21_to_yuy2(src_y: *const u8,
                        src_stride_y: c_int,
                        src_uv: *const u8,
                        src_stride_uv: c_int,
                        dst_yuy2: *const u8,
                        dst_stride_yuy2: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert MT2T to P010
    #[link_name = "MT2TToP010"]
    pub fn m_t2_t_to_p010(src_y: *const u16,
                          src_stride_y: c_int,
                          src_uv: *const u16,
                          src_stride_uv: c_int,
                          dst_y: *const u16,
                          dst_stride_y: c_int,
                          dst_uv: *const u16,
                          dst_stride_uv: c_int,
                          width: c_int,
                          height: c_int)
                          -> c_int;

    // Convert I422 to NV21.
    #[link_name = "I422ToNV21"]
    pub fn i422_to_nv21(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_vu: *const u8,
                        dst_stride_vu: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Copy I420 to I420.
    #[link_name = "I420Copy"]
    pub fn i420_copy(src_y: *const u8,
                     src_stride_y: c_int,
                     src_u: *const u8,
                     src_stride_u: c_int,
                     src_v: *const u8,
                     src_stride_v: c_int,
                     dst_y: *const u8,
                     dst_stride_y: c_int,
                     dst_u: *const u8,
                     dst_stride_u: c_int,
                     dst_v: *const u8,
                     dst_stride_v: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    // Copy I010 to I010
    #[link_name = "I010Copy"]
    pub fn i010_copy(src_y: *const u16,
                     src_stride_y: c_int,
                     src_u: *const u16,
                     src_stride_u: c_int,
                     src_v: *const u16,
                     src_stride_v: c_int,
                     dst_y: *const u16,
                     dst_stride_y: c_int,
                     dst_u: *const u16,
                     dst_stride_u: c_int,
                     dst_v: *const u16,
                     dst_stride_v: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    // Convert 10 bit YUV to 8 bit
    #[link_name = "I010ToI420"]
    pub fn i010_to_i420(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I210ToI420"]
    pub fn i210_to_i420(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I210ToI422"]
    pub fn i210_to_i422(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I410ToI444"]
    pub fn i410_to_i444(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I012ToI420"]
    pub fn i012_to_i420(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I212ToI422"]
    pub fn i212_to_i422(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I412ToI444"]
    pub fn i412_to_i444(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I410ToI010"]
    pub fn i410_to_i010(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I210ToI010"]
    pub fn i210_to_i010(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I010 to I410
    #[link_name = "I010ToI410"]
    pub fn i010_to_i410(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I210 to I410
    #[link_name = "I210ToI410"]
    pub fn i210_to_i410(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I010 to P010
    #[link_name = "I010ToP010"]
    pub fn i010_to_p010(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_uv: *const u16,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I210 to P210
    #[link_name = "I210ToP210"]
    pub fn i210_to_p210(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_uv: *const u16,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I012ToP012"]
    pub fn i012_to_p012(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_uv: *const u16,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I212ToP212"]
    pub fn i212_to_p212(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_uv: *const u16,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I400ToI420"]
    pub fn i400_to_i420(src_y: *const u8,
                        src_stride_y: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I400ToNV21"]
    pub fn i400_to_nv21(src_y: *const u8,
                        src_stride_y: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_vu: *const u8,
                        dst_stride_vu: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "NV12ToI420"]
    pub fn nv12_to_i420(src_y: *const u8,
                        src_stride_y: c_int,
                        src_uv: *const u8,
                        src_stride_uv: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "NV21ToI420"]
    pub fn nv21_to_i420(src_y: *const u8,
                        src_stride_y: c_int,
                        src_vu: *const u8,
                        src_stride_vu: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "NV12ToNV24"]
    pub fn nv12_to_n_v24(src_y: *const u8,
                         src_stride_y: c_int,
                         src_uv: *const u8,
                         src_stride_uv: c_int,
                         dst_y: *const u8,
                         dst_stride_y: c_int,
                         dst_uv: *const u8,
                         dst_stride_uv: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "NV16ToNV24"]
    pub fn nv16_to_n_v24(src_y: *const u8,
                         src_stride_y: c_int,
                         src_uv: *const u8,
                         src_stride_uv: c_int,
                         dst_y: *const u8,
                         dst_stride_y: c_int,
                         dst_uv: *const u8,
                         dst_stride_uv: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "P010ToP410"]
    pub fn p010_to_p410(src_y: *const u16,
                        src_stride_y: c_int,
                        src_uv: *const u16,
                        src_stride_uv: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_uv: *const u16,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "P210ToP410"]
    pub fn p210_to_p410(src_y: *const u16,
                        src_stride_y: c_int,
                        src_uv: *const u16,
                        src_stride_uv: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_uv: *const u16,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "YUY2ToI420"]
    pub fn yuy2_to_i420(src_yuy2: *const u8,
                        src_stride_yuy2: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "UYVYToI420"]
    pub fn uyvy_to_i420(src_uyvy: *const u8,
                        src_stride_uyvy: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AYUVToNV12"]
    pub fn ayuv_to_nv12(src_ayuv: *const u8,
                        src_stride_ayuv: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_uv: *const u8,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AYUVToNV21"]
    pub fn ayuv_to_nv21(src_ayuv: *const u8,
                        src_stride_ayuv: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_vu: *const u8,
                        dst_stride_vu: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "Android420ToI420"]
    pub fn android420_to_i420(src_y: *const u8,
                              src_stride_y: c_int,
                              src_u: *const u8,
                              src_stride_u: c_int,
                              src_v: *const u8,
                              src_stride_v: c_int,
                              src_pixel_stride_uv: c_int,
                              dst_y: *const u8,
                              dst_stride_y: c_int,
                              dst_u: *const u8,
                              dst_stride_u: c_int,
                              dst_v: *const u8,
                              dst_stride_v: c_int,
                              width: c_int,
                              height: c_int)
                              -> c_int;

    #[link_name = "BGRAToI420"]
    pub fn bgra_to_i420(src_bgra: *const u8,
                        src_stride_bgra: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ABGRToI420"]
    pub fn abgr_to_i420(src_abgr: *const u8,
                        src_stride_abgr: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "RGBAToI420"]
    pub fn rgba_to_i420(src_rgba: *const u8,
                        src_stride_rgba: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "RGB24ToI420"]
    pub fn rgb24_to_i420(src_rgb24: *const u8,
                         src_stride_rgb24: c_int,
                         dst_y: *const u8,
                         dst_stride_y: c_int,
                         dst_u: *const u8,
                         dst_stride_u: c_int,
                         dst_v: *const u8,
                         dst_stride_v: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "RGB24ToJ420"]
    pub fn rgb24_to_j420(src_rgb24: *const u8,
                         src_stride_rgb24: c_int,
                         dst_y: *const u8,
                         dst_stride_y: c_int,
                         dst_u: *const u8,
                         dst_stride_u: c_int,
                         dst_v: *const u8,
                         dst_stride_v: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "RAWToI420"]
    pub fn raw_to_i420(src_raw: *const u8,
                       src_stride_raw: c_int,
                       dst_y: *const u8,
                       dst_stride_y: c_int,
                       dst_u: *const u8,
                       dst_stride_u: c_int,
                       dst_v: *const u8,
                       dst_stride_v: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "RAWToJ420"]
    pub fn raw_to_j420(src_raw: *const u8,
                       src_stride_raw: c_int,
                       dst_y: *const u8,
                       dst_stride_y: c_int,
                       dst_u: *const u8,
                       dst_stride_u: c_int,
                       dst_v: *const u8,
                       dst_stride_v: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "RGB565ToI420"]
    pub fn rgb565_to_i420(src_rgb565: *const u8,
                          src_stride_rgb565: c_int,
                          dst_y: *const u8,
                          dst_stride_y: c_int,
                          dst_u: *const u8,
                          dst_stride_u: c_int,
                          dst_v: *const u8,
                          dst_stride_v: c_int,
                          width: c_int,
                          height: c_int)
                          -> c_int;

    #[link_name = "ARGB1555ToI420"]
    pub fn argb1555_to_i420(src_argb1555: *const u8,
                            src_stride_argb1555: c_int,
                            dst_y: *const u8,
                            dst_stride_y: c_int,
                            dst_u: *const u8,
                            dst_stride_u: c_int,
                            dst_v: *const u8,
                            dst_stride_v: c_int,
                            width: c_int,
                            height: c_int)
                            -> c_int;

    #[link_name = "ARGB4444ToI420"]
    pub fn argb4444_to_i420(src_argb4444: *const u8,
                            src_stride_argb4444: c_int,
                            dst_y: *const u8,
                            dst_stride_y: c_int,
                            dst_u: *const u8,
                            dst_stride_u: c_int,
                            dst_v: *const u8,
                            dst_stride_v: c_int,
                            width: c_int,
                            height: c_int)
                            -> c_int;

    #[link_name = "RGB24ToJ400"]
    pub fn rgb24_to_j400(src_rgb24: *const u8,
                         src_stride_rgb24: c_int,
                         dst_yj: *const u8,
                         dst_stride_yj: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "RAWToJ400"]
    pub fn raw_to_j400(src_raw: *const u8,
                       src_stride_raw: c_int,
                       dst_yj: *const u8,
                       dst_stride_yj: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "MJPGToI420"]
    pub fn mjpg_to_i420(sample: *const u8,
                        sample_size: usize,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        src_width: c_int,
                        src_height: c_int,
                        dst_width: c_int,
                        dst_height: c_int)
                        -> c_int;

    #[link_name = "MJPGToNV21"]
    pub fn mjpg_to_nv21(sample: *const u8,
                        sample_size: usize,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_vu: *const u8,
                        dst_stride_vu: c_int,
                        src_width: c_int,
                        src_height: c_int,
                        dst_width: c_int,
                        dst_height: c_int)
                        -> c_int;

    #[link_name = "MJPGToNV12"]
    pub fn mjpg_to_nv12(sample: *const u8,
                        sample_size: usize,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_uv: *const u8,
                        dst_stride_uv: c_int,
                        src_width: c_int,
                        src_height: c_int,
                        dst_width: c_int,
                        dst_height: c_int)
                        -> c_int;

    #[link_name = "MJPGSize"]
    pub fn mjpg_size(sample: *const u8,
                     sample_size: usize,
                     width: *const c_int,
                     height: *const c_int)
                     -> c_int;

    #[link_name = "ConvertToI420"]
    pub fn convert_to_i420(sample: *const u8,
                           sample_size: usize,
                           dst_y: *const u8,
                           dst_stride_y: c_int,
                           dst_u: *const u8,
                           dst_stride_u: c_int,
                           dst_v: *const u8,
                           dst_stride_v: c_int,
                           crop_x: c_int,
                           crop_y: c_int,
                           src_width: c_int,
                           src_height: c_int,
                           crop_width: c_int,
                           crop_height: c_int,
                           rotation: RotationMode,
                           fourcc: u32)
                           -> c_int;

    #[link_name = "CopyPlane"]
    pub fn copy_plane(src_y: *const u8,
                      src_stride_y: c_int,
                      dst_y: *const u8,
                      dst_stride_y: c_int,
                      width: c_int,
                      height: c_int);

    #[link_name = "CopyPlane_16"]
    pub fn copy_plane_16(src_y: *const u16,
                         src_stride_y: c_int,
                         dst_y: *const u16,
                         dst_stride_y: c_int,
                         width: c_int,
                         height: c_int);

    #[link_name = "Convert16To8Plane"]
    pub fn convert16_to8_plane(src_y: *const u16,
                               src_stride_y: c_int,
                               dst_y: *const u8,
                               dst_stride_y: c_int,
                               scale: c_int,
                               width: c_int,
                               height: c_int);

    #[link_name = "Convert8To16Plane"]
    pub fn convert8_to16_plane(src_y: *const u8,
                               src_stride_y: c_int,
                               dst_y: *const u16,
                               dst_stride_y: c_int,
                               scale: c_int,
                               width: c_int,
                               height: c_int);

    #[link_name = "SetPlane"]
    pub fn set_plane(dst_y: *const u8,
                     dst_stride_y: c_int,
                     width: c_int,
                     height: c_int,
                     value: u32);

    #[link_name = "DetilePlane"]
    pub fn detile_plane(src_y: *const u8,
                        src_stride_y: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        width: c_int,
                        height: c_int,
                        tile_height: c_int)
                        -> c_int;

    #[link_name = "DetilePlane_16"]
    pub fn detile_plane_16(src_y: *const u16,
                           src_stride_y: c_int,
                           dst_y: *const u16,
                           dst_stride_y: c_int,
                           width: c_int,
                           height: c_int,
                           tile_height: c_int)
                           -> c_int;

    #[link_name = "DetileSplitUVPlane"]
    pub fn detile_split_uv_plane(src_uv: *const u8,
                                 src_stride_uv: c_int,
                                 dst_u: *const u8,
                                 dst_stride_u: c_int,
                                 dst_v: *const u8,
                                 dst_stride_v: c_int,
                                 width: c_int,
                                 height: c_int,
                                 tile_height: c_int);

    #[link_name = "DetileToYUY2"]
    pub fn detile_to_yuy2(src_y: *const u8,
                          src_stride_y: c_int,
                          src_uv: *const u8,
                          src_stride_uv: c_int,
                          dst_yuy2: *const u8,
                          dst_stride_yuy2: c_int,
                          width: c_int,
                          height: c_int,
                          tile_height: c_int);

    #[link_name = "SplitUVPlane"]
    pub fn split_uv_plane(src_uv: *const u8,
                          src_stride_uv: c_int,
                          dst_u: *const u8,
                          dst_stride_u: c_int,
                          dst_v: *const u8,
                          dst_stride_v: c_int,
                          width: c_int,
                          height: c_int);

    #[link_name = "MergeUVPlane"]
    pub fn merge_uv_plane(src_u: *const u8,
                          src_stride_u: c_int,
                          src_v: *const u8,
                          src_stride_v: c_int,
                          dst_uv: *const u8,
                          dst_stride_uv: c_int,
                          width: c_int,
                          height: c_int);

    #[link_name = "SplitUVPlane_16"]
    pub fn split_uv_plane_16(src_uv: *const u16,
                             src_stride_uv: c_int,
                             dst_u: *const u16,
                             dst_stride_u: c_int,
                             dst_v: *const u16,
                             dst_stride_v: c_int,
                             width: c_int,
                             height: c_int,
                             depth: c_int);

    #[link_name = "MergeUVPlane_16"]
    pub fn merge_uv_plane_16(src_u: *const u16,
                             src_stride_u: c_int,
                             src_v: *const u16,
                             src_stride_v: c_int,
                             dst_uv: *const u16,
                             dst_stride_uv: c_int,
                             width: c_int,
                             height: c_int,
                             depth: c_int);

    #[link_name = "ConvertToMSBPlane_16"]
    pub fn convert_to_msb_plane_16(src_y: *const u16,
                                   src_stride_y: c_int,
                                   dst_y: *const u16,
                                   dst_stride_y: c_int,
                                   width: c_int,
                                   height: c_int,
                                   depth: c_int);

    #[link_name = "ConvertToLSBPlane_16"]
    pub fn convert_to_lsb_plane_16(src_y: *const u16,
                                   src_stride_y: c_int,
                                   dst_y: *const u16,
                                   dst_stride_y: c_int,
                                   width: c_int,
                                   height: c_int,
                                   depth: c_int);

    #[link_name = "HalfMergeUVPlane"]
    pub fn half_merge_uv_plane(src_u: *const u8,
                               src_stride_u: c_int,
                               src_v: *const u8,
                               src_stride_v: c_int,
                               dst_uv: *const u8,
                               dst_stride_uv: c_int,
                               width: c_int,
                               height: c_int);

    #[link_name = "SwapUVPlane"]
    pub fn swap_uv_plane(src_uv: *const u8,
                         src_stride_uv: c_int,
                         dst_vu: *const u8,
                         dst_stride_vu: c_int,
                         width: c_int,
                         height: c_int);

    #[link_name = "SplitRGBPlane"]
    pub fn split_rgb_plane(src_rgb: *const u8,
                           src_stride_rgb: c_int,
                           dst_r: *const u8,
                           dst_stride_r: c_int,
                           dst_g: *const u8,
                           dst_stride_g: c_int,
                           dst_b: *const u8,
                           dst_stride_b: c_int,
                           width: c_int,
                           height: c_int);

    #[link_name = "MergeRGBPlane"]
    pub fn merge_rgb_plane(src_r: *const u8,
                           src_stride_r: c_int,
                           src_g: *const u8,
                           src_stride_g: c_int,
                           src_b: *const u8,
                           src_stride_b: c_int,
                           dst_rgb: *const u8,
                           dst_stride_rgb: c_int,
                           width: c_int,
                           height: c_int);

    #[link_name = "SplitARGBPlane"]
    pub fn split_argb_plane(src_argb: *const u8,
                            src_stride_argb: c_int,
                            dst_r: *const u8,
                            dst_stride_r: c_int,
                            dst_g: *const u8,
                            dst_stride_g: c_int,
                            dst_b: *const u8,
                            dst_stride_b: c_int,
                            dst_a: *const u8,
                            dst_stride_a: c_int,
                            width: c_int,
                            height: c_int);

    #[link_name = "MergeARGBPlane"]
    pub fn merge_argb_plane(src_r: *const u8,
                            src_stride_r: c_int,
                            src_g: *const u8,
                            src_stride_g: c_int,
                            src_b: *const u8,
                            src_stride_b: c_int,
                            src_a: *const u8,
                            src_stride_a: c_int,
                            dst_argb: *const u8,
                            dst_stride_argb: c_int,
                            width: c_int,
                            height: c_int);

    #[link_name = "MergeXR30Plane"]
    pub fn merge_xr30_plane(src_r: *const u16,
                            src_stride_r: c_int,
                            src_g: *const u16,
                            src_stride_g: c_int,
                            src_b: *const u16,
                            src_stride_b: c_int,
                            dst_ar30: *const u8,
                            dst_stride_ar30: c_int,
                            width: c_int,
                            height: c_int,
                            depth: c_int);

    #[link_name = "MergeAR64Plane"]
    pub fn merge_ar64_plane(src_r: *const u16,
                            src_stride_r: c_int,
                            src_g: *const u16,
                            src_stride_g: c_int,
                            src_b: *const u16,
                            src_stride_b: c_int,
                            src_a: *const u16,
                            src_stride_a: c_int,
                            dst_ar64: *const u16,
                            dst_stride_ar64: c_int,
                            width: c_int,
                            height: c_int,
                            depth: c_int);

    #[link_name = "MergeARGB16To8Plane"]
    pub fn merge_argb16_to8_plane(src_r: *const u16,
                                  src_stride_r: c_int,
                                  src_g: *const u16,
                                  src_stride_g: c_int,
                                  src_b: *const u16,
                                  src_stride_b: c_int,
                                  src_a: *const u16,
                                  src_stride_a: c_int,
                                  dst_argb: *const u8,
                                  dst_stride_argb: c_int,
                                  width: c_int,
                                  height: c_int,
                                  depth: c_int);

    #[link_name = "I400ToI400"]
    pub fn i400_to_i400(src_y: *const u8,
                        src_stride_y: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I422Copy"]
    pub fn i422_copy(src_y: *const u8,
                     src_stride_y: c_int,
                     src_u: *const u8,
                     src_stride_u: c_int,
                     src_v: *const u8,
                     src_stride_v: c_int,
                     dst_y: *const u8,
                     dst_stride_y: c_int,
                     dst_u: *const u8,
                     dst_stride_u: c_int,
                     dst_v: *const u8,
                     dst_stride_v: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "I444Copy"]
    pub fn i444_copy(src_y: *const u8,
                     src_stride_y: c_int,
                     src_u: *const u8,
                     src_stride_u: c_int,
                     src_v: *const u8,
                     src_stride_v: c_int,
                     dst_y: *const u8,
                     dst_stride_y: c_int,
                     dst_u: *const u8,
                     dst_stride_u: c_int,
                     dst_v: *const u8,
                     dst_stride_v: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "I210Copy"]
    pub fn i210_copy(src_y: *const u16,
                     src_stride_y: c_int,
                     src_u: *const u16,
                     src_stride_u: c_int,
                     src_v: *const u16,
                     src_stride_v: c_int,
                     dst_y: *const u16,
                     dst_stride_y: c_int,
                     dst_u: *const u16,
                     dst_stride_u: c_int,
                     dst_v: *const u16,
                     dst_stride_v: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "NV12Copy"]
    pub fn nv12_copy(src_y: *const u8,
                     src_stride_y: c_int,
                     src_uv: *const u8,
                     src_stride_uv: c_int,
                     dst_y: *const u8,
                     dst_stride_y: c_int,
                     dst_uv: *const u8,
                     dst_stride_uv: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "NV21Copy"]
    pub fn nv21_copy(src_y: *const u8,
                     src_stride_y: c_int,
                     src_vu: *const u8,
                     src_stride_vu: c_int,
                     dst_y: *const u8,
                     dst_stride_y: c_int,
                     dst_vu: *const u8,
                     dst_stride_vu: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "YUY2ToI422"]
    pub fn yuy2_to_i422(src_yuy2: *const u8,
                        src_stride_yuy2: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "UYVYToI422"]
    pub fn uyvy_to_i422(src_uyvy: *const u8,
                        src_stride_uyvy: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "YUY2ToNV12"]
    pub fn yuy2_to_nv12(src_yuy2: *const u8,
                        src_stride_yuy2: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_uv: *const u8,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "UYVYToNV12"]
    pub fn uyvy_to_nv12(src_uyvy: *const u8,
                        src_stride_uyvy: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_uv: *const u8,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "NV21ToNV12"]
    pub fn nv21_to_nv12(src_y: *const u8,
                        src_stride_y: c_int,
                        src_vu: *const u8,
                        src_stride_vu: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_uv: *const u8,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "YUY2ToY"]
    pub fn yuy2_to_y(src_yuy2: *const u8,
                     src_stride_yuy2: c_int,
                     dst_y: *const u8,
                     dst_stride_y: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "UYVYToY"]
    pub fn uyvy_to_y(src_uyvy: *const u8,
                     src_stride_uyvy: c_int,
                     dst_y: *const u8,
                     dst_stride_y: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "I420ToI400"]
    pub fn i420_to_i400(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420Mirror"]
    pub fn i420_mirror(src_y: *const u8,
                       src_stride_y: c_int,
                       src_u: *const u8,
                       src_stride_u: c_int,
                       src_v: *const u8,
                       src_stride_v: c_int,
                       dst_y: *const u8,
                       dst_stride_y: c_int,
                       dst_u: *const u8,
                       dst_stride_u: c_int,
                       dst_v: *const u8,
                       dst_stride_v: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "I400Mirror"]
    pub fn i400_mirror(src_y: *const u8,
                       src_stride_y: c_int,
                       dst_y: *const u8,
                       dst_stride_y: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "NV12Mirror"]
    pub fn nv12_mirror(src_y: *const u8,
                       src_stride_y: c_int,
                       src_uv: *const u8,
                       src_stride_uv: c_int,
                       dst_y: *const u8,
                       dst_stride_y: c_int,
                       dst_uv: *const u8,
                       dst_stride_uv: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "ARGBMirror"]
    pub fn argb_mirror(src_argb: *const u8,
                       src_stride_argb: c_int,
                       dst_argb: *const u8,
                       dst_stride_argb: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "RGB24Mirror"]
    pub fn rgb24_mirror(src_rgb24: *const u8,
                        src_stride_rgb24: c_int,
                        dst_rgb24: *const u8,
                        dst_stride_rgb24: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "MirrorPlane"]
    pub fn mirror_plane(src_y: *const u8,
                        src_stride_y: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        width: c_int,
                        height: c_int);

    #[link_name = "MirrorUVPlane"]
    pub fn mirror_uv_plane(src_uv: *const u8,
                           src_stride_uv: c_int,
                           dst_uv: *const u8,
                           dst_stride_uv: c_int,
                           width: c_int,
                           height: c_int);

    #[link_name = "RAWToRGB24"]
    pub fn raw_to_rgb24(src_raw: *const u8,
                        src_stride_raw: c_int,
                        dst_rgb24: *const u8,
                        dst_stride_rgb24: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420Rect"]
    pub fn i420_rect(dst_y: *const u8,
                     dst_stride_y: c_int,
                     dst_u: *const u8,
                     dst_stride_u: c_int,
                     dst_v: *const u8,
                     dst_stride_v: c_int,
                     x: c_int,
                     y: c_int,
                     width: c_int,
                     height: c_int,
                     value_y: c_int,
                     value_u: c_int,
                     value_v: c_int)
                     -> c_int;

    #[link_name = "ARGBRect"]
    pub fn argb_rect(dst_argb: *const u8,
                     dst_stride_argb: c_int,
                     dst_x: c_int,
                     dst_y: c_int,
                     width: c_int,
                     height: c_int,
                     value: u32)
                     -> c_int;

    #[link_name = "ARGBGrayTo"]
    pub fn argb_gray_to(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBGray"]
    pub fn argb_gray(dst_argb: *const u8,
                     dst_stride_argb: c_int,
                     dst_x: c_int,
                     dst_y: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "ARGBSepia"]
    pub fn argb_sepia(dst_argb: *const u8,
                      dst_stride_argb: c_int,
                      dst_x: c_int,
                      dst_y: c_int,
                      width: c_int,
                      height: c_int)
                      -> c_int;

    #[link_name = "ARGBColorTable"]
    pub fn argb_color_table(dst_argb: *const u8,
                            dst_stride_argb: c_int,
                            table_argb: *const u8,
                            dst_x: c_int,
                            dst_y: c_int,
                            width: c_int,
                            height: c_int)
                            -> c_int;

    #[link_name = "RGBColorTable"]
    pub fn rgb_color_table(dst_argb: *const u8,
                           dst_stride_argb: c_int,
                           table_argb: *const u8,
                           dst_x: c_int,
                           dst_y: c_int,
                           width: c_int,
                           height: c_int)
                           -> c_int;

    #[link_name = "ARGBLumaColorTable"]
    pub fn argb_luma_color_table(src_argb: *const u8,
                                 src_stride_argb: c_int,
                                 dst_argb: *const u8,
                                 dst_stride_argb: c_int,
                                 luma: *const u8,
                                 width: c_int,
                                 height: c_int)
                                 -> c_int;

    #[link_name = "ARGBPolynomial"]
    pub fn argb_polynomial(src_argb: *const u8,
                           src_stride_argb: c_int,
                           dst_argb: *const u8,
                           dst_stride_argb: c_int,
                           poly: *const c_float,
                           width: c_int,
                           height: c_int)
                           -> c_int;

    #[link_name = "HalfFloatPlane"]
    pub fn half_float_plane(src_y: *const u16,
                            src_stride_y: c_int,
                            dst_y: *const u16,
                            dst_stride_y: c_int,
                            scale: c_float,
                            width: c_int,
                            height: c_int)
                            -> c_int;

    #[link_name = "ByteToFloat"]
    pub fn byte_to_float(src_y: *const u8,
                         dst_y: *const c_float,
                         scale: c_float,
                         width: c_int)
                         -> c_int;

    #[link_name = "ARGBQuantize"]
    pub fn argb_quantize(dst_argb: *const u8,
                         dst_stride_argb: c_int,
                         scale: c_int,
                         interval_size: c_int,
                         interval_offset: c_int,
                         dst_x: c_int,
                         dst_y: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "ARGBCopyAlpha"]
    pub fn argb_copy_alpha(src_argb: *const u8,
                           src_stride_argb: c_int,
                           dst_argb: *const u8,
                           dst_stride_argb: c_int,
                           width: c_int,
                           height: c_int)
                           -> c_int;

    #[link_name = "ARGBExtractAlpha"]
    pub fn argb_extract_alpha(src_argb: *const u8,
                              src_stride_argb: c_int,
                              dst_a: *const u8,
                              dst_stride_a: c_int,
                              width: c_int,
                              height: c_int)
                              -> c_int;

    #[link_name = "ARGBCopyYToAlpha"]
    pub fn argb_copy_y_to_alpha(src_y: *const u8,
                                src_stride_y: c_int,
                                dst_argb: *const u8,
                                dst_stride_argb: c_int,
                                width: c_int,
                                height: c_int)
                                -> c_int;

    #[link_name = "ARGBBlend"]
    pub fn argb_blend(src_argb0: *const u8,
                      src_stride_argb0: c_int,
                      src_argb1: *const u8,
                      src_stride_argb1: c_int,
                      dst_argb: *const u8,
                      dst_stride_argb: c_int,
                      width: c_int,
                      height: c_int)
                      -> c_int;

    #[link_name = "BlendPlane"]
    pub fn blend_plane(src_y0: *const u8,
                       src_stride_y0: c_int,
                       src_y1: *const u8,
                       src_stride_y1: c_int,
                       alpha: *const u8,
                       alpha_stride: c_int,
                       dst_y: *const u8,
                       dst_stride_y: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "I420Blend"]
    pub fn i420_blend(src_y0: *const u8,
                      src_stride_y0: c_int,
                      src_u0: *const u8,
                      src_stride_u0: c_int,
                      src_v0: *const u8,
                      src_stride_v0: c_int,
                      src_y1: *const u8,
                      src_stride_y1: c_int,
                      src_u1: *const u8,
                      src_stride_u1: c_int,
                      src_v1: *const u8,
                      src_stride_v1: c_int,
                      alpha: *const u8,
                      alpha_stride: c_int,
                      dst_y: *const u8,
                      dst_stride_y: c_int,
                      dst_u: *const u8,
                      dst_stride_u: c_int,
                      dst_v: *const u8,
                      dst_stride_v: c_int,
                      width: c_int,
                      height: c_int)
                      -> c_int;

    #[link_name = "ARGBMultiply"]
    pub fn argb_multiply(src_argb0: *const u8,
                         src_stride_argb0: c_int,
                         src_argb1: *const u8,
                         src_stride_argb1: c_int,
                         dst_argb: *const u8,
                         dst_stride_argb: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "ARGBAdd"]
    pub fn argb_add(src_argb0: *const u8,
                    src_stride_argb0: c_int,
                    src_argb1: *const u8,
                    src_stride_argb1: c_int,
                    dst_argb: *const u8,
                    dst_stride_argb: c_int,
                    width: c_int,
                    height: c_int)
                    -> c_int;

    #[link_name = "ARGBSubtract"]
    pub fn argb_subtract(src_argb0: *const u8,
                         src_stride_argb0: c_int,
                         src_argb1: *const u8,
                         src_stride_argb1: c_int,
                         dst_argb: *const u8,
                         dst_stride_argb: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "I422ToYUY2"]
    pub fn i422_to_yuy2(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_yuy2: *const u8,
                        dst_stride_yuy2: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I422ToUYVY"]
    pub fn i422_to_uyvy(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_uyvy: *const u8,
                        dst_stride_uyvy: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBAttenuate"]
    pub fn argb_attenuate(src_argb: *const u8,
                          src_stride_argb: c_int,
                          dst_argb: *const u8,
                          dst_stride_argb: c_int,
                          width: c_int,
                          height: c_int)
                          -> c_int;

    #[link_name = "ARGBUnattenuate"]
    pub fn argb_unattenuate(src_argb: *const u8,
                            src_stride_argb: c_int,
                            dst_argb: *const u8,
                            dst_stride_argb: c_int,
                            width: c_int,
                            height: c_int)
                            -> c_int;

    #[link_name = "ARGBComputeCumulativeSum"]
    pub fn argb_compute_cumulative_sum(src_argb: *const u8,
                                       src_stride_argb: c_int,
                                       dst_cumsum: *const i32,
                                       dst_stride32_cumsum: c_int,
                                       width: c_int,
                                       height: c_int)
                                       -> c_int;

    #[link_name = "ARGBBlur"]
    pub fn argb_blur(src_argb: *const u8,
                     src_stride_argb: c_int,
                     dst_argb: *const u8,
                     dst_stride_argb: c_int,
                     dst_cumsum: *const i32,
                     dst_stride32_cumsum: c_int,
                     width: c_int,
                     height: c_int,
                     radius: c_int)
                     -> c_int;

    #[link_name = "GaussPlane_F32"]
    pub fn gauss_plane_f32(src: *const c_float,
                           src_stride: c_int,
                           dst: *const c_float,
                           dst_stride: c_int,
                           width: c_int,
                           height: c_int)
                           -> c_int;

    #[link_name = "ARGBShade"]
    pub fn argb_shade(src_argb: *const u8,
                      src_stride_argb: c_int,
                      dst_argb: *const u8,
                      dst_stride_argb: c_int,
                      width: c_int,
                      height: c_int,
                      value: u32)
                      -> c_int;

    #[link_name = "InterpolatePlane"]
    pub fn interpolate_plane(src0: *const u8,
                             src_stride0: c_int,
                             src1: *const u8,
                             src_stride1: c_int,
                             dst: *const u8,
                             dst_stride: c_int,
                             width: c_int,
                             height: c_int,
                             interpolation: c_int)
                             -> c_int;

    #[link_name = "InterpolatePlane_16"]
    pub fn interpolate_plane_16(src0: *const u16,
                                src_stride0: c_int,
                                src1: *const u16,
                                src_stride1: c_int,
                                dst: *const u16,
                                dst_stride: c_int,
                                width: c_int,
                                height: c_int,
                                interpolation: c_int)
                                -> c_int;

    #[link_name = "ARGBInterpolate"]
    pub fn argb_interpolate(src_argb0: *const u8,
                            src_stride_argb0: c_int,
                            src_argb1: *const u8,
                            src_stride_argb1: c_int,
                            dst_argb: *const u8,
                            dst_stride_argb: c_int,
                            width: c_int,
                            height: c_int,
                            interpolation: c_int)
                            -> c_int;

    #[link_name = "I420Interpolate"]
    pub fn i420_interpolate(src0_y: *const u8,
                            src0_stride_y: c_int,
                            src0_u: *const u8,
                            src0_stride_u: c_int,
                            src0_v: *const u8,
                            src0_stride_v: c_int,
                            src1_y: *const u8,
                            src1_stride_y: c_int,
                            src1_u: *const u8,
                            src1_stride_u: c_int,
                            src1_v: *const u8,
                            src1_stride_v: c_int,
                            dst_y: *const u8,
                            dst_stride_y: c_int,
                            dst_u: *const u8,
                            dst_stride_u: c_int,
                            dst_v: *const u8,
                            dst_stride_v: c_int,
                            width: c_int,
                            height: c_int,
                            interpolation: c_int)
                            -> c_int;

    #[link_name = "ARGBAffineRow_C"]
    pub fn argb_affine_row_c(src_argb: *const u8,
                             src_argb_stride: c_int,
                             dst_argb: *const u8,
                             uv_dudv: *const c_float,
                             width: c_int);

    #[link_name = "ARGBAffineRow_SSE2"]
    pub fn argb_affine_row_sse2(src_argb: *const u8,
                                src_argb_stride: c_int,
                                dst_argb: *const u8,
                                uv_dudv: *const c_float,
                                width: c_int);

    #[link_name = "ARGBShuffle"]
    pub fn argb_shuffle(src_bgra: *const u8,
                        src_stride_bgra: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        shuffler: *const u8,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AR64Shuffle"]
    pub fn ar64_shuffle(src_ar64: *const u16,
                        src_stride_ar64: c_int,
                        dst_ar64: *const u16,
                        dst_stride_ar64: c_int,
                        shuffler: *const u8,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBSobelToPlane"]
    pub fn argb_sobel_to_plane(src_argb: *const u8,
                               src_stride_argb: c_int,
                               dst_y: *const u8,
                               dst_stride_y: c_int,
                               width: c_int,
                               height: c_int)
                               -> c_int;

    #[link_name = "ARGBSobel"]
    pub fn argb_sobel(src_argb: *const u8,
                      src_stride_argb: c_int,
                      dst_argb: *const u8,
                      dst_stride_argb: c_int,
                      width: c_int,
                      height: c_int)
                      -> c_int;

    #[link_name = "ARGBSobelXY"]
    pub fn argb_sobel_xy(src_argb: *const u8,
                         src_stride_argb: c_int,
                         dst_argb: *const u8,
                         dst_stride_argb: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "ARGBRotate"]
    pub fn argb_rotate(src_argb: *const u8,
                       src_stride_argb: c_int,
                       dst_argb: *const u8,
                       dst_stride_argb: c_int,
                       src_width: c_int,
                       src_height: c_int,
                       mode: RotationMode)
                       -> c_int;

    #[link_name = "I420Rotate"]
    pub fn i420_rotate(src_y: *const u8,
                       src_stride_y: c_int,
                       src_u: *const u8,
                       src_stride_u: c_int,
                       src_v: *const u8,
                       src_stride_v: c_int,
                       dst_y: *const u8,
                       dst_stride_y: c_int,
                       dst_u: *const u8,
                       dst_stride_u: c_int,
                       dst_v: *const u8,
                       dst_stride_v: c_int,
                       width: c_int,
                       height: c_int,
                       mode: RotationMode)
                       -> c_int;

    #[link_name = "I422Rotate"]
    pub fn i422_rotate(src_y: *const u8,
                       src_stride_y: c_int,
                       src_u: *const u8,
                       src_stride_u: c_int,
                       src_v: *const u8,
                       src_stride_v: c_int,
                       dst_y: *const u8,
                       dst_stride_y: c_int,
                       dst_u: *const u8,
                       dst_stride_u: c_int,
                       dst_v: *const u8,
                       dst_stride_v: c_int,
                       width: c_int,
                       height: c_int,
                       mode: RotationMode)
                       -> c_int;

    #[link_name = "I444Rotate"]
    pub fn i444_rotate(src_y: *const u8,
                       src_stride_y: c_int,
                       src_u: *const u8,
                       src_stride_u: c_int,
                       src_v: *const u8,
                       src_stride_v: c_int,
                       dst_y: *const u8,
                       dst_stride_y: c_int,
                       dst_u: *const u8,
                       dst_stride_u: c_int,
                       dst_v: *const u8,
                       dst_stride_v: c_int,
                       width: c_int,
                       height: c_int,
                       mode: RotationMode)
                       -> c_int;

    #[link_name = "NV12ToI420Rotate"]
    pub fn nv12_to_i420_rotate(src_y: *const u8,
                               src_stride_y: c_int,
                               src_uv: *const u8,
                               src_stride_uv: c_int,
                               dst_y: *const u8,
                               dst_stride_y: c_int,
                               dst_u: *const u8,
                               dst_stride_u: c_int,
                               dst_v: *const u8,
                               dst_stride_v: c_int,
                               width: c_int,
                               height: c_int,
                               mode: RotationMode)
                               -> c_int;

    #[link_name = "Android420ToI420Rotate"]
    pub fn android420_to_i420_rotate(src_y: *const u8,
                                     src_stride_y: c_int,
                                     src_u: *const u8,
                                     src_stride_u: c_int,
                                     src_v: *const u8,
                                     src_stride_v: c_int,
                                     src_pixel_stride_uv: c_int,
                                     dst_y: *const u8,
                                     dst_stride_y: c_int,
                                     dst_u: *const u8,
                                     dst_stride_u: c_int,
                                     dst_v: *const u8,
                                     dst_stride_v: c_int,
                                     width: c_int,
                                     height: c_int,
                                     rotation: RotationMode)
                                     -> c_int;

    #[link_name = "RotatePlane"]
    pub fn rotate_plane(src: *const u8,
                        src_stride: c_int,
                        dst: *const u8,
                        dst_stride: c_int,
                        width: c_int,
                        height: c_int,
                        mode: RotationMode)
                        -> c_int;

    #[link_name = "RotatePlane90"]
    pub fn rotate_plane90(src: *const u8,
                          src_stride: c_int,
                          dst: *const u8,
                          dst_stride: c_int,
                          width: c_int,
                          height: c_int);

    #[link_name = "RotatePlane180"]
    pub fn rotate_plane180(src: *const u8,
                           src_stride: c_int,
                           dst: *const u8,
                           dst_stride: c_int,
                           width: c_int,
                           height: c_int);

    #[link_name = "RotatePlane270"]
    pub fn rotate_plane270(src: *const u8,
                           src_stride: c_int,
                           dst: *const u8,
                           dst_stride: c_int,
                           width: c_int,
                           height: c_int);

    #[link_name = "SplitRotateUV"]
    pub fn split_rotate_uv(src_uv: *const u8,
                           src_stride_uv: c_int,
                           dst_u: *const u8,
                           dst_stride_u: c_int,
                           dst_v: *const u8,
                           dst_stride_v: c_int,
                           width: c_int,
                           height: c_int,
                           mode: RotationMode)
                           -> c_int;

    #[link_name = "SplitRotateUV90"]
    pub fn split_rotate_uv90(src: *const u8,
                             src_stride: c_int,
                             dst_a: *const u8,
                             dst_stride_a: c_int,
                             dst_b: *const u8,
                             dst_stride_b: c_int,
                             width: c_int,
                             height: c_int);

    #[link_name = "SplitRotateUV180"]
    pub fn split_rotate_uv180(src: *const u8,
                              src_stride: c_int,
                              dst_a: *const u8,
                              dst_stride_a: c_int,
                              dst_b: *const u8,
                              dst_stride_b: c_int,
                              width: c_int,
                              height: c_int);

    #[link_name = "SplitRotateUV270"]
    pub fn split_rotate_uv270(src: *const u8,
                              src_stride: c_int,
                              dst_a: *const u8,
                              dst_stride_a: c_int,
                              dst_b: *const u8,
                              dst_stride_b: c_int,
                              width: c_int,
                              height: c_int);

    #[link_name = "TransposePlane"]
    pub fn transpose_plane(src: *const u8,
                           src_stride: c_int,
                           dst: *const u8,
                           dst_stride: c_int,
                           width: c_int,
                           height: c_int);

    #[link_name = "SplitTransposeUV"]
    pub fn split_transpose_uv(src: *const u8,
                              src_stride: c_int,
                              dst_a: *const u8,
                              dst_stride_a: c_int,
                              dst_b: *const u8,
                              dst_stride_b: c_int,
                              width: c_int,
                              height: c_int);

    #[link_name = "ARGBScale"]
    pub fn argb_scale(src_argb: *const u8,
                      src_stride_argb: c_int,
                      src_width: c_int,
                      src_height: c_int,
                      dst_argb: *const u8,
                      dst_stride_argb: c_int,
                      dst_width: c_int,
                      dst_height: c_int,
                      filtering: FilterMode)
                      -> c_int;

    #[link_name = "ARGBScaleClip"]
    pub fn argb_scale_clip(src_argb: *const u8,
                           src_stride_argb: c_int,
                           src_width: c_int,
                           src_height: c_int,
                           dst_argb: *const u8,
                           dst_stride_argb: c_int,
                           dst_width: c_int,
                           dst_height: c_int,
                           clip_x: c_int,
                           clip_y: c_int,
                           clip_width: c_int,
                           clip_height: c_int,
                           filtering: FilterMode)
                           -> c_int;

    #[link_name = "YUVToARGBScaleClip"]
    pub fn yuv_to_argb_scale_clip(src_y: *const u8,
                                  src_stride_y: c_int,
                                  src_u: *const u8,
                                  src_stride_u: c_int,
                                  src_v: *const u8,
                                  src_stride_v: c_int,
                                  src_fourcc: u32,
                                  src_width: c_int,
                                  src_height: c_int,
                                  dst_argb: *const u8,
                                  dst_stride_argb: c_int,
                                  dst_fourcc: u32,
                                  dst_width: c_int,
                                  dst_height: c_int,
                                  clip_x: c_int,
                                  clip_y: c_int,
                                  clip_width: c_int,
                                  clip_height: c_int,
                                  filtering: FilterMode)
                                  -> c_int;

    #[link_name = "RGBScale"]
    pub fn rgb_scale(src_rgb: *const u8,
                     src_stride_rgb: c_int,
                     src_width: c_int,
                     src_height: c_int,
                     dst_rgb: *const u8,
                     dst_stride_rgb: c_int,
                     dst_width: c_int,
                     dst_height: c_int,
                     filtering: FilterMode)
                     -> c_int;

    #[link_name = "UVScale"]
    pub fn uv_scale(src_uv: *const u8,
                    src_stride_uv: c_int,
                    src_width: c_int,
                    src_height: c_int,
                    dst_uv: *const u8,
                    dst_stride_uv: c_int,
                    dst_width: c_int,
                    dst_height: c_int,
                    filtering: FilterMode)
                    -> c_int;

    #[link_name = "UVScale_16"]
    pub fn uv_scale_16(src_uv: *const u16,
                       src_stride_uv: c_int,
                       src_width: c_int,
                       src_height: c_int,
                       dst_uv: *const u16,
                       dst_stride_uv: c_int,
                       dst_width: c_int,
                       dst_height: c_int,
                       filtering: FilterMode)
                       -> c_int;

    #[link_name = "ScalePlane"]
    pub fn scale_plane(src: *const u8,
                       src_stride: c_int,
                       src_width: c_int,
                       src_height: c_int,
                       dst: *const u8,
                       dst_stride: c_int,
                       dst_width: c_int,
                       dst_height: c_int,
                       filtering: FilterMode);

    #[link_name = "ScalePlane_16"]
    pub fn scale_plane_16(src: *const u16,
                          src_stride: c_int,
                          src_width: c_int,
                          src_height: c_int,
                          dst: *const u16,
                          dst_stride: c_int,
                          dst_width: c_int,
                          dst_height: c_int,
                          filtering: FilterMode);

    #[link_name = "ScalePlane_12"]
    pub fn scale_plane_12(src: *const u16,
                          src_stride: c_int,
                          src_width: c_int,
                          src_height: c_int,
                          dst: *const u16,
                          dst_stride: c_int,
                          dst_width: c_int,
                          dst_height: c_int,
                          filtering: FilterMode);

    #[link_name = "I420Scale"]
    pub fn i420_scale(src_y: *const u8,
                      src_stride_y: c_int,
                      src_u: *const u8,
                      src_stride_u: c_int,
                      src_v: *const u8,
                      src_stride_v: c_int,
                      src_width: c_int,
                      src_height: c_int,
                      dst_y: *const u8,
                      dst_stride_y: c_int,
                      dst_u: *const u8,
                      dst_stride_u: c_int,
                      dst_v: *const u8,
                      dst_stride_v: c_int,
                      dst_width: c_int,
                      dst_height: c_int,
                      filtering: FilterMode)
                      -> c_int;

    #[link_name = "I420Scale_16"]
    pub fn i420_scale_16(src_y: *const u16,
                         src_stride_y: c_int,
                         src_u: *const u16,
                         src_stride_u: c_int,
                         src_v: *const u16,
                         src_stride_v: c_int,
                         src_width: c_int,
                         src_height: c_int,
                         dst_y: *const u16,
                         dst_stride_y: c_int,
                         dst_u: *const u16,
                         dst_stride_u: c_int,
                         dst_v: *const u16,
                         dst_stride_v: c_int,
                         dst_width: c_int,
                         dst_height: c_int,
                         filtering: FilterMode)
                         -> c_int;

    #[link_name = "I420Scale_12"]
    pub fn i420_scale_12(src_y: *const u16,
                         src_stride_y: c_int,
                         src_u: *const u16,
                         src_stride_u: c_int,
                         src_v: *const u16,
                         src_stride_v: c_int,
                         src_width: c_int,
                         src_height: c_int,
                         dst_y: *const u16,
                         dst_stride_y: c_int,
                         dst_u: *const u16,
                         dst_stride_u: c_int,
                         dst_v: *const u16,
                         dst_stride_v: c_int,
                         dst_width: c_int,
                         dst_height: c_int,
                         filtering: FilterMode)
                         -> c_int;

    #[link_name = "I444Scale"]
    pub fn i444_scale(src_y: *const u8,
                      src_stride_y: c_int,
                      src_u: *const u8,
                      src_stride_u: c_int,
                      src_v: *const u8,
                      src_stride_v: c_int,
                      src_width: c_int,
                      src_height: c_int,
                      dst_y: *const u8,
                      dst_stride_y: c_int,
                      dst_u: *const u8,
                      dst_stride_u: c_int,
                      dst_v: *const u8,
                      dst_stride_v: c_int,
                      dst_width: c_int,
                      dst_height: c_int,
                      filtering: FilterMode)
                      -> c_int;

    #[link_name = "I444Scale_16"]
    pub fn i444_scale_16(src_y: *const u16,
                         src_stride_y: c_int,
                         src_u: *const u16,
                         src_stride_u: c_int,
                         src_v: *const u16,
                         src_stride_v: c_int,
                         src_width: c_int,
                         src_height: c_int,
                         dst_y: *const u16,
                         dst_stride_y: c_int,
                         dst_u: *const u16,
                         dst_stride_u: c_int,
                         dst_v: *const u16,
                         dst_stride_v: c_int,
                         dst_width: c_int,
                         dst_height: c_int,
                         filtering: FilterMode)
                         -> c_int;

    #[link_name = "I444Scale_12"]
    pub fn i444_scale_12(src_y: *const u16,
                         src_stride_y: c_int,
                         src_u: *const u16,
                         src_stride_u: c_int,
                         src_v: *const u16,
                         src_stride_v: c_int,
                         src_width: c_int,
                         src_height: c_int,
                         dst_y: *const u16,
                         dst_stride_y: c_int,
                         dst_u: *const u16,
                         dst_stride_u: c_int,
                         dst_v: *const u16,
                         dst_stride_v: c_int,
                         dst_width: c_int,
                         dst_height: c_int,
                         filtering: FilterMode)
                         -> c_int;

    #[link_name = "I422Scale"]
    pub fn i422_scale(src_y: *const u8,
                      src_stride_y: c_int,
                      src_u: *const u8,
                      src_stride_u: c_int,
                      src_v: *const u8,
                      src_stride_v: c_int,
                      src_width: c_int,
                      src_height: c_int,
                      dst_y: *const u8,
                      dst_stride_y: c_int,
                      dst_u: *const u8,
                      dst_stride_u: c_int,
                      dst_v: *const u8,
                      dst_stride_v: c_int,
                      dst_width: c_int,
                      dst_height: c_int,
                      filtering: FilterMode)
                      -> c_int;

    #[link_name = "I422Scale_16"]
    pub fn i422_scale_16(src_y: *const u16,
                         src_stride_y: c_int,
                         src_u: *const u16,
                         src_stride_u: c_int,
                         src_v: *const u16,
                         src_stride_v: c_int,
                         src_width: c_int,
                         src_height: c_int,
                         dst_y: *const u16,
                         dst_stride_y: c_int,
                         dst_u: *const u16,
                         dst_stride_u: c_int,
                         dst_v: *const u16,
                         dst_stride_v: c_int,
                         dst_width: c_int,
                         dst_height: c_int,
                         filtering: FilterMode)
                         -> c_int;

    #[link_name = "I422Scale_12"]
    pub fn i422_scale_12(src_y: *const u16,
                         src_stride_y: c_int,
                         src_u: *const u16,
                         src_stride_u: c_int,
                         src_v: *const u16,
                         src_stride_v: c_int,
                         src_width: c_int,
                         src_height: c_int,
                         dst_y: *const u16,
                         dst_stride_y: c_int,
                         dst_u: *const u16,
                         dst_stride_u: c_int,
                         dst_v: *const u16,
                         dst_stride_v: c_int,
                         dst_width: c_int,
                         dst_height: c_int,
                         filtering: FilterMode)
                         -> c_int;

    #[link_name = "NV12Scale"]
    pub fn nv12_scale(src_y: *const u8,
                      src_stride_y: c_int,
                      src_uv: *const u8,
                      src_stride_uv: c_int,
                      src_width: c_int,
                      src_height: c_int,
                      dst_y: *const u8,
                      dst_stride_y: c_int,
                      dst_uv: *const u8,
                      dst_stride_uv: c_int,
                      dst_width: c_int,
                      dst_height: c_int,
                      filtering: FilterMode)
                      -> c_int;

    #[link_name = "Scale"]
    pub fn scale(src_y: *const u8,
                 src_u: *const u8,
                 src_v: *const u8,
                 src_stride_y: c_int,
                 src_stride_u: c_int,
                 src_stride_v: c_int,
                 src_width: c_int,
                 src_height: c_int,
                 dst_y: *const u8,
                 dst_u: *const u8,
                 dst_v: *const u8,
                 dst_stride_y: c_int,
                 dst_stride_u: c_int,
                 dst_stride_v: c_int,
                 dst_width: c_int,
                 dst_height: c_int,
                 interpolate: bool)
                 -> c_int;

    #[link_name = "ARGBToARGB"]
    pub fn argb_to_argb(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "YToARGB"]
    pub fn y_to_argb(src_y: *const u8,
                     src_stride_y: c_int,
                     dst_argb: *const u8,
                     dst_stride_argb: c_int,
                     width: c_int,
                     height: c_int)
                     -> c_int;

    #[link_name = "BG24ToARGB"]
    pub fn bg24_to_argb(src_rgb24: *const u8,
                        src_stride_rgb24: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AB30ToARGB"]
    pub fn ab30_to_argb(src_ar30: *const u8,
                        src_stride_ar30: c_int,
                        dst_abgr: *const u8,
                        dst_stride_abgr: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AB30ToABGR"]
    pub fn ab30_to_abgr(src_ar30: *const u8,
                        src_stride_ar30: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AB30ToAR30"]
    pub fn ab30_to_ar30(src_ar30: *const u8,
                        src_stride_ar30: c_int,
                        dst_ab30: *const u8,
                        dst_stride_ab30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AB64ToABGR"]
    pub fn ab60_to_abgr(src_ar64: *const u16,
                        src_stride_ar64: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AR64ToABGR"]
    pub fn ar64_to_abgr(src_ab64: *const u16,
                        src_stride_ab64: c_int,
                        dst_argb: *const u8,
                        dst_stride_argb: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "AB64ToAR64"]
    pub fn ab60_to_ar64(src_ar64: *const u16,
                        src_stride_ar64: c_int,
                        dst_ab64: *const u16,
                        dst_stride_ab64: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ARGBToAB30"]
    pub fn argb_to_ab30(src_abgr: *const u8,
                        src_stride_abgr: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ABGRToAB30"]
    pub fn abgr_to_ab30(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_ar30: *const u8,
                        dst_stride_ar30: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ABGRToRGB24"]
    pub fn abgr_to_rgb24(src_argb: *const u8,
                         src_stride_argb: c_int,
                         dst_raw: *const u8,
                         dst_stride_raw: c_int,
                         width: c_int,
                         height: c_int)
                         -> c_int;

    #[link_name = "ABGRToRAW"]
    pub fn abgr_to_raw(src_argb: *const u8,
                       src_stride_argb: c_int,
                       dst_rgb24: *const u8,
                       dst_stride_rgb24: c_int,
                       width: c_int,
                       height: c_int)
                       -> c_int;

    #[link_name = "ABGRToAB64"]
    pub fn abgr_to_ab60(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_ar64: *const u16,
                        dst_stride_ar64: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "ABGRToAR64"]
    pub fn abgr_to_ar64(src_argb: *const u8,
                        src_stride_argb: c_int,
                        dst_ab64: *const u16,
                        dst_stride_ab64: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H420ToH010"]
    pub fn h420_to_h010(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H420ToH012"]
    pub fn h420_to_h012(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToI420"]
    pub fn i420_to_i420(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I010ToI010"]
    pub fn i010_to_i010(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H010ToH010"]
    pub fn h010_to_h010(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H010ToH420"]
    pub fn h010_to_h420(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H210ToH420"]
    pub fn h210_to_h420(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H210ToH422"]
    pub fn h210_to_h422(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H410ToH444"]
    pub fn h410_to_h444(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H012ToH420"]
    pub fn h012_to_h420(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H212ToH422"]
    pub fn h212_to_h422(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H412ToH444"]
    pub fn h412_to_h444(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I412ToI012"]
    pub fn i412_to_i012(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H410ToH010"]
    pub fn h410_to_h010(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H412ToH012"]
    pub fn h412_to_h012(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I212ToI012"]
    pub fn i212_to_i012(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H210ToH010"]
    pub fn h210_to_h010(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "H212ToH012"]
    pub fn h212_to_h012(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I012 to I412
    #[link_name = "I012ToI412"]
    pub fn i012_to_i412(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    // Convert I212 to I412
    #[link_name = "I212ToI412"]
    pub fn i212_to_i412(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "J400ToJ420"]
    pub fn j400_to_j420(src_y: *const u8,
                        src_stride_y: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "P012ToP412"]
    pub fn p012_to_p412(src_y: *const u16,
                        src_stride_y: c_int,
                        src_uv: *const u16,
                        src_stride_uv: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_uv: *const u16,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "P016ToP416"]
    pub fn p016_to_p416(src_y: *const u16,
                        src_stride_y: c_int,
                        src_uv: *const u16,
                        src_stride_uv: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_uv: *const u16,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "P212ToP412"]
    pub fn p212_to_p412(src_y: *const u16,
                        src_stride_y: c_int,
                        src_uv: *const u16,
                        src_stride_uv: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_uv: *const u16,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "P216ToP416"]
    pub fn p216_to_p416(src_y: *const u16,
                        src_stride_y: c_int,
                        src_uv: *const u16,
                        src_stride_uv: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_uv: *const u16,
                        dst_stride_uv: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "J400ToJ400"]
    pub fn j400_to_j400(src_y: *const u8,
                        src_stride_y: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I422ToI422"]
    pub fn i422_to_i422(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I444ToI444"]
    pub fn i444_to_i444(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        dst_u: *const u8,
                        dst_stride_u: c_int,
                        dst_v: *const u8,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I210ToI210"]
    pub fn i210_to_i210(src_y: *const u16,
                        src_stride_y: c_int,
                        src_u: *const u16,
                        src_stride_u: c_int,
                        src_v: *const u16,
                        src_stride_v: c_int,
                        dst_y: *const u16,
                        dst_stride_y: c_int,
                        dst_u: *const u16,
                        dst_stride_u: c_int,
                        dst_v: *const u16,
                        dst_stride_v: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "J420ToJ400"]
    pub fn j420_to_j400(src_y: *const u8,
                        src_stride_y: c_int,
                        src_u: *const u8,
                        src_stride_u: c_int,
                        src_v: *const u8,
                        src_stride_v: c_int,
                        dst_y: *const u8,
                        dst_stride_y: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;

    #[link_name = "I420ToI420Mirror"]
    pub fn i420_to_i420_mirror(src_y: *const u8,
                               src_stride_y: c_int,
                               src_u: *const u8,
                               src_stride_u: c_int,
                               src_v: *const u8,
                               src_stride_v: c_int,
                               dst_y: *const u8,
                               dst_stride_y: c_int,
                               dst_u: *const u8,
                               dst_stride_u: c_int,
                               dst_v: *const u8,
                               dst_stride_v: c_int,
                               width: c_int,
                               height: c_int)
                               -> c_int;

    #[link_name = "I400ToI400Mirror"]
    pub fn i400_to_i400_mirror(src_y: *const u8,
                               src_stride_y: c_int,
                               dst_y: *const u8,
                               dst_stride_y: c_int,
                               width: c_int,
                               height: c_int)
                               -> c_int;

    #[link_name = "NV12ToNV12Mirror"]
    pub fn nv12_to_nv12_mirror(src_y: *const u8,
                               src_stride_y: c_int,
                               src_uv: *const u8,
                               src_stride_uv: c_int,
                               dst_y: *const u8,
                               dst_stride_y: c_int,
                               dst_uv: *const u8,
                               dst_stride_uv: c_int,
                               width: c_int,
                               height: c_int)
                               -> c_int;

    #[link_name = "ARGBToARGBMirror"]
    pub fn argb_to_argb_mirror(src_argb: *const u8,
                               src_stride_argb: c_int,
                               dst_argb: *const u8,
                               dst_stride_argb: c_int,
                               width: c_int,
                               height: c_int)
                               -> c_int;

    #[link_name = "RGB24ToRGB24Mirror"]
    pub fn rgb24_to_rgb24_mirror(src_rgb24: *const u8,
                                 src_stride_rgb24: c_int,
                                 dst_rgb24: *const u8,
                                 dst_stride_rgb24: c_int,
                                 width: c_int,
                                 height: c_int)
                                 -> c_int;

    #[link_name = "RGB24ToRAW"]
    pub fn rgb24_to_raw(src_raw: *const u8,
                        src_stride_raw: c_int,
                        dst_rgb24: *const u8,
                        dst_stride_rgb24: c_int,
                        width: c_int,
                        height: c_int)
                        -> c_int;
}
