#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[repr(align(32))]
#[derive(Copy, Clone)]
pub struct U256(pub [u64; 4]);

impl U256 {
    pub fn default() -> U256 {
        return U256 { 0: [0, 0, 0, 0] };
    }

    pub fn from_slice(value: &[u64]) -> U256 {
        return U256 { 0: [value[0], value[1], value[2], value[3]] };
    }

    pub fn from_dec_str(value: &str) -> Result<U256, uint::FromDecStrErr> {
        match ethereum_types::U256::from_dec_str(value) {
            Ok(temp) => {
                let mask = ethereum_types::U256::from(u64::max_value());
                let data: [u64; 4] = [
                    ((temp >>   0) & mask).as_u64(),
                    ((temp >>  64) & mask).as_u64(),
                    ((temp >> 128) & mask).as_u64(),
                    ((temp >> 192) & mask).as_u64()
                ];
                Ok(U256::from_slice(&data))
            }
            Err(err) => Err(err)
        }
    }

    pub fn from_u64(value: u64) -> U256 {
        return U256 { 0: [value, 0, 0, 0] };
    }

    pub fn low_u64(&self) -> u64 {
        return self.0[0];
    }

    pub fn low_u128(&self) -> u128 {
        let lo = self.0[0];
        let hi = self.0[1];
        lo as u128 | (hi as u128 >> 64)
    }

    pub fn le_u64(&self) -> bool {
        (self.0[1] == 0) & (self.0[2] == 0) & (self.0[3] == 0)
    }
}

pub trait __m256iExt {
    unsafe fn as_u256(&self) -> U256;
}

impl __m256iExt for __m256i {
    unsafe fn as_u256(&self) -> U256 {
        return std::mem::transmute::<__m256i, U256>(*self);
    }
}

#[cfg(target_feature = "avx2")]
#[derive(Copy, Clone)]
#[repr(align(32))]
pub struct Word(pub __m256i);

#[cfg(all(not(target_feature = "avx2"), target_feature = "ssse3"))]
#[derive(Copy, Clone)]
#[repr(align(32))]
pub struct Word(pub (__m128i, __m128i));

impl Word {
    pub unsafe fn as_u256(&self) -> U256 {
        std::mem::transmute::<Word, U256>(*self)
    }

    pub unsafe fn from_slice(value: &[u64]) -> Word {
        #[cfg(target_feature = "avx2")]
        {
            return Word(_mm256_set_epi64x(value[3] as i64,
                                          value[2] as i64,
                                          value[1] as i64,
                                          value[0] as i64));
        }
        #[cfg(all(not(target_feature = "avx2"), target_feature = "ssse3"))]
        {
            return Word((_mm_set_epi64x(value[1] as i64, value[0] as i64),
                         _mm_set_epi64x(value[3] as i64, value[2] as i64)));
        }
        unimplemented!()
    }

    pub unsafe fn from_u64(value: u64) -> Word {
        #[cfg(target_feature = "avx2")]
        {
            return Word(_mm256_set_epi64x(0, 0, 0, value as i64));
        }
        #[cfg(all(not(target_feature = "avx2"), target_feature = "ssse3"))]
        {
            return Word((_mm_set_epi64x(0, value as i64), _mm_setzero_si128()));
        }
        unimplemented!()
    }
}

#[allow(unreachable_code)]
pub unsafe fn load_u256(src: *const U256, offset: isize) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let src = src.offset(offset) as *const __m256i;
        let result = _mm256_load_si256(src);
        return std::mem::transmute::<__m256i, U256>(result);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let src = src.offset(offset) as *const __m128i;
        let result = (_mm_load_si128(src), _mm_load_si128(src.offset(1)));
        return std::mem::transmute::<(__m128i, __m128i), U256>(result);
    }
    return *src;
}

#[allow(unreachable_code)]
pub unsafe fn loadu_u256(src: *const U256, offset: isize) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let src = src.offset(offset) as *const __m256i;
        let result = _mm256_loadu_si256(src);
        return std::mem::transmute::<__m256i, U256>(result);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let src = src.offset(offset) as *const __m128i;
        let result = (_mm_loadu_si128(src), _mm_loadu_si128(src.offset(1)));
        return std::mem::transmute::<(__m128i, __m128i), U256>(result);
    }
    return *src;
}

#[allow(unreachable_code)]
pub unsafe fn store_u256(dest: *mut U256, value: U256, offset: isize) {
    #[cfg(target_feature = "avx2")]
    {
        let value = std::mem::transmute::<U256, __m256i>(value);
        let dest = dest.offset(offset) as *mut __m256i;
        _mm256_store_si256(dest, value);
        return;
    }
    #[cfg(target_feature = "ssse3")]
    {
        let value = std::mem::transmute::<U256, (__m128i, __m128i)>(value);
        let dest = dest.offset(offset) as *mut __m128i;
        _mm_store_si128(dest, value.0);
        _mm_store_si128(dest.offset(1), value.1);
        return;
    }
    *dest = value;
}

#[allow(unreachable_code)]
pub unsafe fn storeu_u256(dest: *mut U256, value: U256, offset: isize) {
    #[cfg(target_feature = "avx2")]
    {
        let value = std::mem::transmute::<U256, __m256i>(value);
        let dest = dest.offset(offset) as *mut __m256i;
        _mm256_storeu_si256(dest, value);
        return;
    }
    #[cfg(target_feature = "ssse3")]
    {
        let value = std::mem::transmute::<U256, (__m128i, __m128i)>(value);
        let dest = dest.offset(offset) as *mut __m128i;
        _mm_storeu_si128(dest, value.0);
        _mm_storeu_si128(dest.offset(1), value.1);
        return;
    }
    *dest = value;
}

#[allow(unreachable_code)]
pub unsafe fn load16_u256(src: *const U256, num_bytes: i32) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let lane8_id = _mm256_set_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
        let all_ones = _mm256_set_epi64x(-1, -1, -1, -1);
        //
        let src = src as *const __m128i;
        let value = _mm256_zextsi128_si256(_mm_loadu_si128(src));
        let sfloor = _mm_set_epi32(0, 0, 0, (255 - 32) + num_bytes);
        let floor = _mm256_broadcastb_epi8(sfloor);
        let ssum = _mm256_adds_epu8(lane8_id, floor);
        let mask = _mm256_cmpeq_epi8(ssum, all_ones);
        return std::mem::transmute::<__m256i, U256>(_mm256_and_si256(value, mask));
    }
    #[cfg(target_feature = "ssse3")]
    {
        let lane8_id = _mm_set_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        let all_ones = _mm_set_epi64x(-1, -1);
        let zero = _mm_setzero_si128();
        //
        let src = src as *const __m128i;
        let value = _mm_loadu_si128(src);
        let sfloor = _mm_set_epi32(0, 0, 0, (255 - 16) + num_bytes);
        let floor = _mm_shuffle_epi8(sfloor, zero);
        let ssum = _mm_adds_epu8(lane8_id, floor);
        let mask = _mm_cmpeq_epi8(ssum, all_ones);
        return std::mem::transmute::<(__m128i, __m128i), U256>((_mm_and_si128(value, mask), zero));
    }
    unimplemented!()
}

#[allow(unreachable_code)]
pub unsafe fn load32_u256(src: *const U256, num_bytes: i32) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let lane8_id = _mm256_set_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
        let all_ones = _mm256_set_epi64x(-1, -1, -1, -1);
        //
        let src = src as *const __m256i;
        let value = _mm256_loadu_si256(src);
        let sfloor = _mm_set_epi32(0, 0, 0, (255 - 32) + num_bytes);
        let floor = _mm256_broadcastb_epi8(sfloor);
        let ssum = _mm256_adds_epu8(lane8_id, floor);
        let mask = _mm256_cmpeq_epi8(ssum, all_ones);
        return std::mem::transmute::<__m256i, U256>(_mm256_and_si256(value, mask));
    }
    #[cfg(target_feature = "ssse3")]
    {
        let lane8_id = _mm_set_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        let all_ones = _mm_set_epi64x(-1, -1);
        //
        let src = src as *const __m128i;
        let valuelo = _mm_loadu_si128(src);
        let valuehi = _mm_loadu_si128(src.offset(1));
        let sfloor = _mm_set_epi32(0, 0, 0, (255 - 32) + num_bytes);
        let floor = _mm_shuffle_epi8(sfloor, _mm_setzero_si128());
        let ssum = _mm_adds_epu8(lane8_id, floor);
        let mask = _mm_cmpeq_epi8(ssum, all_ones);
        return std::mem::transmute::<(__m128i, __m128i), U256>((valuelo, _mm_and_si128(valuehi, mask)));
    }
    unimplemented!()
}

#[allow(unreachable_code)]
pub unsafe fn bswap_u256(value: U256) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let lane8_id = _mm256_set_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
        const SWAP_LANE128: i32 = (1 << 0) + (0 << 4);
        //
        let value = std::mem::transmute::<U256, __m256i>(value);
        let bswap = _mm256_shuffle_epi8(value, lane8_id);
        let result = _mm256_permute2x128_si256(bswap, bswap, SWAP_LANE128);
        return std::mem::transmute::<__m256i, U256>(result);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let lane8_id = _mm_set_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        //
        let value = std::mem::transmute::<U256, (__m128i, __m128i)>(value);
        let resultlo = _mm_shuffle_epi8(value.1, lane8_id);
        let resulthi = _mm_shuffle_epi8(value.0, lane8_id);
        return std::mem::transmute::<(__m128i, __m128i), U256>((resultlo, resulthi));
    }
    unimplemented!()
}

#[allow(unreachable_code)]
pub unsafe fn is_zero_u256(value: U256) -> bool {
    #[cfg(target_feature = "avx2")]
    {
        let all_ones = _mm256_set_epi64x(-1, -1, -1, -1);
        //
        let value = std::mem::transmute::<U256, __m256i>(value);
        let zf = _mm256_testz_si256(all_ones, value);
        return zf != 0;
    }
    #[cfg(target_feature = "ssse3")]
    {
        let zero = _mm_setzero_si128();
        //
        let value = std::mem::transmute::<U256, (__m128i, __m128i)>(value);
        let masklo = _mm_cmpeq_epi32(value.0, zero);
        let maskhi = _mm_cmpeq_epi32(value.1, zero);
        let mask16 = _mm_movemask_epi8(_mm_and_si128(masklo, maskhi));
        return mask16 == 0xffff;
    }
    unimplemented!()
}

#[allow(unreachable_code)]
pub unsafe fn is_ltpow2_u256(value: U256, pow2: usize) -> bool {
    #[cfg(target_feature = "avx2")]
    {
        let one = _mm256_set_epi64x(0, 0, 0, 1);
        //
        let value = std::mem::transmute::<U256, __m256i>(value);
        let mask = _mm256_sub_epi64(_mm256_set_epi64x(0, 0, 0, pow2 as i64), one);
        let hipart = _mm256_andnot_si256(mask, value);
        let temp = std::mem::transmute::<__m256i, U256>(hipart);
        let result = is_zero_u256(temp);
        return result;
    }
    #[cfg(target_feature = "ssse3")]
    {
        let one = _mm_set_epi64x(0, 1);
        //
        let value = std::mem::transmute::<U256, (__m128i, __m128i)>(value);
        let mask = _mm_sub_epi64(_mm_set_epi64x(0, pow2 as i64), one);
        let hipart = _mm_andnot_si128(mask, value.0);
        let temp = std::mem::transmute::<(__m128i, __m128i), U256>((hipart, value.1));
        let result = is_zero_u256(temp);
        return result;
    }
    unimplemented!()
}

unsafe fn broadcast_avx2(value: bool) -> __m256i {
    let mask = _mm_set_epi32(0, 0, 0, if value { -1 } else { 0 });
    return _mm256_broadcastd_epi32(mask);
}

unsafe fn broadcast_sse2(value: bool) -> __m128i {
    let mask = _mm_set_epi32(0, 0, 0, if value { -1 } else { 0 });
    return _mm_shuffle_epi32(mask, 0);
}

#[inline(always)]
#[allow(unreachable_code)]
unsafe fn mm_blendv_epi8(a: __m128i, b: __m128i, mask: __m128i) -> __m128i {
    #[cfg(target_feature = "sse4.1")]
    {
        return _mm_blendv_epi8(a, b, mask);
    }
    return _mm_or_si128(_mm_and_si128(b, mask), _mm_andnot_si128(mask, a));
}

#[allow(unreachable_code)]
pub unsafe fn signextend_u256(a: U256, b: U256, value: i64) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let one = _mm256_set_epi64x(0, 0, 0, 1);
        let lane8_id = _mm256_set_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
        let all_ones = _mm256_set_epi64x(-1, -1, -1, -1);
        //
        let _a = std::mem::transmute::<U256, __m256i>(a);
        let _b = std::mem::transmute::<U256, __m256i>(b);
        let signbit = _mm_srli_epi16(_mm_set_epi64x(0, value), 7);
        let signmask8 = _mm_cmpeq_epi8(signbit, _mm256_castsi256_si128(one));
        let signmask = _mm256_broadcastb_epi8(signmask8);
        let alo = _mm256_castsi256_si128(_a);
        let sfloor = _mm_add_epi8(_mm_set_epi64x(0, 255 - 31), alo);
        let floor = _mm256_broadcastb_epi8(sfloor);
        let ssum = _mm256_adds_epu8(lane8_id, floor);
        let mask = _mm256_cmpeq_epi8(ssum, all_ones);
        let temp = _mm256_blendv_epi8(signmask, _b, mask);
        let lt32 = broadcast_avx2(is_ltpow2_u256(a, 32));
        let result = _mm256_blendv_epi8(_b, temp, lt32);
        return std::mem::transmute::<__m256i, U256>(result);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let zero = _mm_setzero_si128();
        let one = _mm_set_epi64x(0, 1);
        let lane8_id = _mm_set_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        let all_ones = _mm_set_epi64x(-1, -1);
        //
        let _a = std::mem::transmute::<U256, (__m128i, __m128i)>(a);
        let _b = std::mem::transmute::<U256, (__m128i, __m128i)>(b);
        let signbit = _mm_srli_epi16(_mm_set_epi64x(0, value), 7);
        let signmask8 = _mm_cmpeq_epi8(signbit, one);
        let signmask = _mm_shuffle_epi8(signmask8, zero);
        let sfloorlo = _mm_adds_epu8(_mm_set_epi64x(0, 255 - 15), _a.0);
        let floorlo = _mm_shuffle_epi8(sfloorlo, zero);
        let ssumlo = _mm_adds_epu8(lane8_id, floorlo);
        let masklo = _mm_cmpeq_epi8(ssumlo, all_ones);
        let templo = mm_blendv_epi8(signmask, _b.0, masklo);
        let sfloorhi = _mm_add_epi8(_mm_set_epi64x(0, 255 - 31), _a.0);
        let floorhi = _mm_shuffle_epi8(sfloorhi, zero);
        let ssumhi = _mm_adds_epu8(lane8_id, floorhi);
        let maskhi = _mm_cmpeq_epi8(ssumhi, all_ones);
        let temphi = mm_blendv_epi8(signmask, _b.1, maskhi);
        let lt32 = broadcast_sse2(is_ltpow2_u256(a, 32));
        let resultlo = mm_blendv_epi8(_b.0, templo, lt32);
        let resulthi = mm_blendv_epi8(_b.1, temphi, lt32);
        return std::mem::transmute::<(__m128i, __m128i), U256>((resultlo, resulthi));
    }
    unimplemented!()
}

#[allow(unreachable_code)]
pub unsafe fn eq_u256(a: U256, b: U256) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let all_ones = _mm256_set_epi64x(-1, -1, -1, -1);
        //
        let a = std::mem::transmute::<U256, __m256i>(a);
        let b = std::mem::transmute::<U256, __m256i>(b);
        let mask = _mm256_cmpeq_epi8(a, b);
        let cf = _mm256_testc_si256(mask, all_ones);
        let result = _mm256_set_epi64x(0, 0, 0, cf as i64);
        return std::mem::transmute::<__m256i, U256>(result);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let a = std::mem::transmute::<U256, (__m128i, __m128i)>(a);
        let b = std::mem::transmute::<U256, (__m128i, __m128i)>(b);
        let masklo = _mm_cmpeq_epi8(a.0, b.0);
        let maskhi = _mm_cmpeq_epi8(a.1, b.1);
        let mask16 = _mm_movemask_epi8(_mm_and_si128(masklo, maskhi));
        let bit = (mask16 == 0xffff) as i64;
        let result = (_mm_set_epi64x(0, bit), _mm_setzero_si128());
        return std::mem::transmute::<(__m128i, __m128i), U256>(result);
    }
    unimplemented!()
}

#[allow(unreachable_code)]
pub unsafe fn iszero_u256(a: U256) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let bit = is_zero_u256(a) as i64;
        let result = _mm256_set_epi64x(0, 0, 0, bit);
        return std::mem::transmute::<__m256i, U256>(result);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let bit = is_zero_u256(a) as i64;
        let result = (_mm_set_epi64x(0, bit), _mm_setzero_si128());
        return std::mem::transmute::<(__m128i, __m128i), U256>(result);
    }
    unimplemented!()
}

#[allow(unreachable_code)]
pub unsafe fn and_u256(a: U256, b: U256) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let a = std::mem::transmute::<U256, __m256i>(a);
        let b = std::mem::transmute::<U256, __m256i>(b);
        let result = _mm256_and_si256(a, b);
        return std::mem::transmute::<__m256i, U256>(result);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let a = std::mem::transmute::<U256, (__m128i, __m128i)>(a);
        let b = std::mem::transmute::<U256, (__m128i, __m128i)>(b);
        let result = (_mm_and_si128(a.0, b.0), _mm_and_si128(a.1, b.1));
        return std::mem::transmute::<(__m128i, __m128i), U256>(result);
    }
    unimplemented!()
}

#[allow(unreachable_code)]
pub unsafe fn or_u256(a: U256, b: U256) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let a = std::mem::transmute::<U256, __m256i>(a);
        let b = std::mem::transmute::<U256, __m256i>(b);
        let result = _mm256_or_si256(a, b);
        return std::mem::transmute::<__m256i, U256>(result);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let a = std::mem::transmute::<U256, (__m128i, __m128i)>(a);
        let b = std::mem::transmute::<U256, (__m128i, __m128i)>(b);
        let result = (_mm_or_si128(a.0, b.0), _mm_or_si128(a.1, b.1));
        return std::mem::transmute::<(__m128i, __m128i), U256>(result);
    }
    unimplemented!()
}

#[allow(unreachable_code)]
pub unsafe fn xor_u256(a: U256, b: U256) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let a = std::mem::transmute::<U256, __m256i>(a);
        let b = std::mem::transmute::<U256, __m256i>(b);
        let result = _mm256_xor_si256(a, b);
        return std::mem::transmute::<__m256i, U256>(result);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let a = std::mem::transmute::<U256, (__m128i, __m128i)>(a);
        let b = std::mem::transmute::<U256, (__m128i, __m128i)>(b);
        let result = (_mm_xor_si128(a.0, b.0), _mm_xor_si128(a.1, b.1));
        return std::mem::transmute::<(__m128i, __m128i), U256>(result);
    }
    unimplemented!()
}

#[allow(unreachable_code)]
pub unsafe fn not_u256(value: U256) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let all_ones = _mm256_set_epi64x(-1, -1, -1, -1);
        //
        let value = std::mem::transmute::<U256, __m256i>(value);
        let result = _mm256_andnot_si256(value, all_ones);
        return std::mem::transmute::<__m256i, U256>(result);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let all_ones = _mm_set_epi64x(-1, -1);
        //
        let value = std::mem::transmute::<U256, (__m128i, __m128i)>(value);
        let resultlo = _mm_andnot_si128(value.0, all_ones);
        let resulthi = _mm_andnot_si128(value.1, all_ones);
        return std::mem::transmute::<(__m128i, __m128i), U256>((resultlo, resulthi));
    }
    unimplemented!()
}

#[allow(non_snake_case)]
const fn _MM_SHUFFLE(z: i32, y: i32, x: i32, w: i32) -> i32 {
    (z << 6) | (y << 4) | (x << 2) | w
}

#[allow(unreachable_code)]
pub unsafe fn shl_u256(count: U256, value: U256) -> U256 {
    #[cfg(target_feature = "avx2")]
    {
        let one = _mm256_set_epi64x(0, 0, 0, 1);
        let sixty_four = _mm_set_epi64x(0, 64);
        let max_u8 = _mm256_sub_epi8(_mm256_setzero_si256(), one);
        let max_u64 = _mm256_sub_epi64(_mm256_setzero_si256(), one);
        //
        let count = std::mem::transmute::<U256, __m256i>(count);
        let value = std::mem::transmute::<U256, __m256i>(value);
        let hi248 = _mm256_andnot_si256(max_u8, count);
        let hiisz = broadcast_avx2(is_zero_u256(hi248.as_u256()));
        let mut temp = value;
        let mut current = _mm256_castsi256_si128(count);
        let mut i = 0;
        while i < 4 {
            let slcount = _mm_min_epu8(sixty_four, current);
            let srcount = _mm_subs_epu8(sixty_four, slcount);
            let sltemp = _mm256_sll_epi64(temp, slcount);
            let srtemp = _mm256_srl_epi64(temp, srcount);
            let carry = _mm256_permute4x64_epi64(srtemp, _MM_SHUFFLE(2, 1, 0, 3));
            temp = _mm256_or_si256(sltemp, _mm256_andnot_si256(max_u64, carry));
            current = _mm_subs_epu8(current, slcount);
            i += 1;
        }
        let result = _mm256_and_si256(temp, hiisz);
        return std::mem::transmute::<__m256i, U256>(result);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let zero = _mm_setzero_si128();
        let one = _mm_set_epi64x(0, 1);
        let sixty_four = _mm_set_epi64x(0, 64);
        let max_u8 = _mm_sub_epi8(zero, one);
        //
        let count = std::mem::transmute::<U256, (__m128i, __m128i)>(count);
        let value = std::mem::transmute::<U256, (__m128i, __m128i)>(value);
        let hi248 = (_mm_andnot_si128(max_u8, count.0), count.1);
        let hi248 = std::mem::transmute::<(__m128i, __m128i), U256>(hi248);
        let hiisz = broadcast_sse2(is_zero_u256(hi248));
        let mut temp = value;
        let mut current = count.0;
        let mut i = 0;
        while i < 4 {
            let slcount = _mm_min_epu8(sixty_four, current);
            let srcount = _mm_subs_epu8(sixty_four, slcount);
            let sltemplo = _mm_sll_epi64(temp.0, slcount);
            let sltemphi = _mm_sll_epi64(temp.1, slcount);
            let srtemplo = _mm_srl_epi64(temp.0, srcount);
            let srtemphi = _mm_srl_epi64(temp.1, srcount);
            let carrylo = _mm_bslli_si128(srtemplo, 8);
            let carryhi = _mm_unpacklo_epi64(_mm_bsrli_si128(srtemplo, 8), srtemphi);
            let templo = _mm_or_si128(sltemplo, carrylo);
            let temphi = _mm_or_si128(sltemphi, carryhi);
            temp = (templo, temphi);
            current = _mm_subs_epu8(current, slcount);
            i += 1;
        }
        let result = (_mm_and_si128(hiisz, temp.0), _mm_and_si128(hiisz, temp.1));
        return std::mem::transmute::<(__m128i, __m128i), U256>(result);
    }
    unimplemented!()
}

pub fn overflowing_add_u256(a: U256, b: U256) -> (U256, bool) {
    let t0 = (a.0[0] as u128) + (b.0[0] as u128);
    let c0 = t0 >> 64;
    let t1 = (a.0[1] as u128) + (b.0[1] as u128) + c0;
    let c1 = t1 >> 64;
    let t2 = (a.0[2] as u128) + (b.0[2] as u128) + c1;
    let c2 = t2 >> 64;
    let t3 = (a.0[3] as u128) + (b.0[3] as u128) + c2;
    let c3 = t3 >> 64;
    (U256([t0 as u64, t1 as u64, t2 as u64, t3 as u64]), c3 != 0)
}

pub fn add_u256(a: U256, b: U256) -> U256 {
    let (value, _) = overflowing_add_u256(a, b);
    value
}

pub fn mul_u64(a: u64, b: u64) -> u128 {
    /*
    #[cfg(target_feature = "bmi2")] {
        let lo: u64;
        let hi: u64;
        unsafe {
            asm!("mulxq $2, $1, $0"
                 : "=r"(hi), "=r"(lo)
                 : "r"(a), "{rdx}"(b)
                 );
        }
        return (lo as u128) | ((hi as u128) << 64);
    }
    */
    (a as u128) * (b as u128)
}

fn mul_diag(num_limbs: usize, i: usize, a: &[u64], b: u64, r: &mut [u64], c: &mut [u64]) {
    let mut carry: u64 = 0;
    for j in 0..num_limbs {
        let temp = mul_u64(a[j], b);
        if j == 0 {
            c[i] = temp as u64;
            carry = (temp >> 64) as u64;
        }
        else {
            let temp2 = temp + (carry as u128);
            if j == (num_limbs - 1) {
                r[j-1] = temp2 as u64;
                r[j-0] = (temp2 >> 64) as u64;
            }
            else {
                r[j-1] = temp2 as u64;
                carry = (temp2 >> 64) as u64;
            }
        }
    }
}

fn mul_diagc(num_limbs: usize, i: usize, a: &[u64], b: u64, r: &mut [u64], rp: &mut [u64], c: &mut [u64]) {
    let mut carry: u64 = 0;
    for j in 0..num_limbs {
        let temp = mul_u64(a[j], b) + (r[j] as u128);
        if j == 0 {
            c[i] = temp as u64;
            carry = (temp >> 64) as u64;
        }
        else {
            let temp2 = temp + (carry as u128);
            if j == (num_limbs - 1) {
                rp[j-1] = temp2 as u64;
                rp[j-0] = (temp2 >> 64) as u64;
            }
            else {
                rp[j-1] = temp2 as u64;
                carry = (temp2 >> 64) as u64;
            }
        }
    }
}

fn mul_limbs(num_limbs: usize, a: &[u64], b: &[u64], c: &mut [u64]) {
    assert!(num_limbs <= 4);
    let mut r: [u64; 8] = unsafe { std::mem::uninitialized() };
    let mut rp: [u64; 8] = unsafe { std::mem::uninitialized() };
    //
    mul_diag(num_limbs, 0, a, b[0], &mut r, c);
    for i in 1..num_limbs {
        mul_diagc(num_limbs, i, a, b[i], &mut r, &mut rp, c);
        for j in 0..num_limbs {
            r[j] = rp[j];
        }
    }
    for i in 0..num_limbs {
        c[num_limbs+i] = rp[i];
    }
}

pub fn mul_u256(a: U256, b: U256) -> U256 {
    let mut c: [u64; 8] = unsafe { std::mem::uninitialized() };
    mul_limbs(4, &a.0, &b.0, &mut c);
    U256([c[0], c[1], c[2], c[3]])
}

fn overflowing_sub_u256(a: U256, b: U256) -> (U256, bool) {
    let alo = ((a.0[1] as u128) << 64) | (a.0[0] as u128);
    let blo = ((b.0[1] as u128) << 64) | (b.0[0] as u128);
    let ahi = ((a.0[3] as u128) << 64) | (a.0[2] as u128);
    let bhi = ((b.0[3] as u128) << 64) | (b.0[2] as u128);
    let (lo, borrowlo) = alo.overflowing_sub(blo);
    let hi = ahi.wrapping_sub(bhi).wrapping_sub(borrowlo as u128);
    let borrow = (ahi < bhi) | ((ahi == bhi) & borrowlo);
    (U256([lo as u64, (lo >> 64) as u64, hi as u64, (hi >> 64) as u64]), borrow)
}

pub fn sub_u256(a: U256, b: U256) -> U256 {
    let (value, _) = overflowing_sub_u256(a, b);
    value
}

pub fn gt_u256(a: U256, b: U256) -> bool {
    let alo = ((a.0[1] as u128) << 64) | (a.0[0] as u128);
    let blo = ((b.0[1] as u128) << 64) | (b.0[0] as u128);
    let ahi = ((a.0[3] as u128) << 64) | (a.0[2] as u128);
    let bhi = ((b.0[3] as u128) << 64) | (b.0[2] as u128);
    (ahi > bhi) | ((ahi == bhi) & (alo > blo))
}

// // this is only possible with rust nightly (#15701)
// macro_rules! mm_extract_epi64 {
//     ($a:expr, 0) => {
//         #[cfg(target_feature = "sse4.1")]
//         {
//             _mm_extract_epi64($a, 0)
//         }
//         #[cfg(not(target_feature = "sse4.1"))]
//         {
//             _mm_cvtsi128_si64($a)
//         }
//     };
//     ($a:expr, 1) => {
//         #[cfg(target_feature = "sse4.1")]
//         {
//             _mm_extract_epi64($a, 1)
//         }
//         #[cfg(not(target_feature = "sse4.1"))]
//         {
//             _mm_cvtsi128_si64(_mm_srli_si128(a, 8))
//         }
//     }
// }

#[inline(always)]
#[allow(unreachable_code)]
unsafe fn mm_extract_epi64(a: __m128i, imm8: i32) -> i64 {
    #[cfg(target_feature = "sse4.1")]
    {
        if imm8 == 0 {
            return _mm_extract_epi64(a, 0);
        }
        else if imm8 == 1 {
            return _mm_extract_epi64(a, 1);
        }
        return unreachable!();
    }
    if imm8 == 0 {
        return _mm_cvtsi128_si64(a);
    }
    else if imm8 == 1 {
        return _mm_cvtsi128_si64(_mm_srli_si128(a, 8));
    }
    unreachable!()
}

#[allow(unreachable_code)]
pub unsafe fn overflowing_sub_word(value: Word, amount: u64) -> (Word, bool) {
    #[cfg(target_feature = "avx2")]
    {
        let value = std::mem::transmute::<Word, __m256i>(value);
        let value0 = _mm256_extract_epi64(value, 0) as u64;
        let value1 = _mm256_extract_epi64(value, 1) as u64;
        let value2 = _mm256_extract_epi64(value, 2) as u64;
        let value3 = _mm256_extract_epi64(value, 3) as u64;
        let (temp0, borrow0) = value0.overflowing_sub(amount);
        let (temp1, borrow1) = value1.overflowing_sub(borrow0 as u64);
        let (temp2, borrow2) = value2.overflowing_sub(borrow1 as u64);
        let (temp3, borrow3) = value3.overflowing_sub(borrow2 as u64);
        let result = _mm256_set_epi64x(temp3 as i64, temp2 as i64, temp1 as i64, temp0 as i64);
        return (std::mem::transmute::<__m256i, Word>(result), borrow3);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let value = std::mem::transmute::<Word, (__m128i, __m128i)>(value);
        let value0 = mm_extract_epi64(value.0, 0) as u64;
        let value1 = mm_extract_epi64(value.0, 1) as u64;
        let value2 = mm_extract_epi64(value.1, 0) as u64;
        let value3 = mm_extract_epi64(value.1, 1) as u64;
        let (temp0, borrow0) = value0.overflowing_sub(amount);
        let (temp1, borrow1) = value1.overflowing_sub(borrow0 as u64);
        let (temp2, borrow2) = value2.overflowing_sub(borrow1 as u64);
        let (temp3, borrow3) = value3.overflowing_sub(borrow2 as u64);
        let resultlo = _mm_set_epi64x(temp1 as i64, temp0 as i64);
        let resulthi = _mm_set_epi64x(temp3 as i64, temp2 as i64);
        let result = (resultlo, resulthi);
        return (std::mem::transmute::<(__m128i, __m128i), Word>(result), borrow3);
    }
    unimplemented!()
}

#[allow(unreachable_code)]
pub unsafe fn overflowing_sub_word_u128(value: Word, amount: u128) -> (Word, bool) {
    #[cfg(target_feature = "avx2")]
    {
        let value = std::mem::transmute::<Word, __m256i>(value);
        let value0 = _mm256_extract_epi64(value, 0) as u64;
        let value1 = _mm256_extract_epi64(value, 1) as u64;
        let value2 = _mm256_extract_epi64(value, 2) as u64;
        let value3 = _mm256_extract_epi64(value, 3) as u64;
        //
        let valuelo = (value1 as u128) << 64 | (value0 as u128);
        let valuehi = (value3 as u128) << 64 | (value2 as u128);
        let (templo, borrowlo) = valuelo.overflowing_sub(amount);
        let (temphi, borrowhi) = valuehi.overflowing_sub(borrowlo as u128);
        let temp0 = templo as u64;
        let temp1 = (templo >> 64) as u64;
        let temp2 = temphi as u64;
        let temp3 = (temphi >> 64) as u64;
        //
        let result = _mm256_set_epi64x(temp3 as i64, temp2 as i64, temp1 as i64, temp0 as i64);
        return (std::mem::transmute::<__m256i, Word>(result), borrowhi);
    }
    #[cfg(target_feature = "ssse3")]
    {
        let value = std::mem::transmute::<Word, (__m128i, __m128i)>(value);
        let value0 = mm_extract_epi64(value.0, 0) as u64;
        let value1 = mm_extract_epi64(value.0, 1) as u64;
        let value2 = mm_extract_epi64(value.1, 0) as u64;
        let value3 = mm_extract_epi64(value.1, 1) as u64;
        //
        let valuelo = (value1 as u128) << 64 | (value0 as u128);
        let valuehi = (value3 as u128) << 64 | (value2 as u128);
        let (templo, borrowlo) = valuelo.overflowing_sub(amount);
        let (temphi, borrowhi) = valuehi.overflowing_sub(borrowlo as u128);
        let temp0 = templo as u64;
        let temp1 = (templo >> 64) as u64;
        let temp2 = temphi as u64;
        let temp3 = (temphi >> 64) as u64;
        //
        let resultlo = _mm_set_epi64x(temp1 as i64, temp0 as i64);
        let resulthi = _mm_set_epi64x(temp3 as i64, temp2 as i64);
        let result = (resultlo, resulthi);
        return (std::mem::transmute::<(__m128i, __m128i), Word>(result), borrowhi);
    }
    unimplemented!()
}