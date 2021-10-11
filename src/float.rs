use core::ops::*;
pub use num_traits::{float::FloatCore, Float, Num, NumCast, One, AsPrimitive, ToPrimitive, Zero};

extern "C" {
    // #[link_name = "llvm.nvvm.add.rn.ftz.f"]
    // fn add_rn_ftz(lhs: f32, rhs: f32) -> f32;
    // #[link_name = "llvm.nvvm.sub.rn.ftz.f"]
    // fn sub_rn_ftz(lhs: f32, rhs: f32) -> f32;
    // #[link_name = "llvm.nvvm.mul.rn.ftz.f"]
    // fn mul_rn_ftz(lhs: f32, rhs: f32) -> f32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.fma.rn.ftz.f"]
    pub fn fma_rn_ftz(a: f32, b: f32, c: f32) -> f32;

    #[ffi_const]
    #[link_name = "llvm.nvvm.rcp.approx.ftz.f"]
    fn recip_approx(v: f32) -> f32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.rcp.approx.ftz.d"]
    fn recip_approx_f64(v: f64) -> f64;

    #[ffi_const]
    #[link_name = "llvm.nvvm.sqrt.approx.ftz.f"]
    fn sqrt_approx(v: f32) -> f32;

    #[ffi_const]
    #[link_name = "llvm.nvvm.rsqrt.approx.ftz.f"]
    pub fn rsqrt_approx(v: f32) -> f32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.rsqrt.approx.d"]
    pub fn rsqrt_approx_f64(v: f64) -> f64;

    #[ffi_const]
    #[link_name = "llvm.nvvm.div.approx.ftz.f"]
    pub fn div_approx(l: f32, r: f32) -> f32;

    #[ffi_const]
    #[link_name = "llvm.nvvm.sin.approx.ftz.f"]
    fn sin_approx(v: f32) -> f32;
    #[ffi_const]
    #[link_name = "llvm.nvvm.cos.approx.ftz.f"]
    fn cos_approx(v: f32) -> f32;

    #[ffi_const]
    #[link_name = "llvm.nvvm.ex2.approx.ftz.f"]
    fn ex2_approx(v: f32) -> f32;
    // #[link_name = "llvm.nvvm.ex2.approx.d"]
    // fn ex2_approx_f64(v: f64) -> f64;
    #[ffi_const]
    #[link_name = "llvm.nvvm.lg2.approx.ftz.f"]
    fn lg2_approx(v: f32) -> f32;
    // #[link_name = "llvm.nvvm.lg2.approx.ftz.d"]
    // fn lg2_approx_f64(v: f64) -> f64;
}

pub trait FastNum: 'static + Sized + Copy + PartialOrd + PartialEq + FloatCore + Float {
    fn fma(self, b: Self, c: Self) -> Self;
    fn fast_add(self, rhs: Self) -> Self {
        unsafe { core::intrinsics::fadd_fast(self, rhs) }
    }
    fn fast_sub(self, rhs: Self) -> Self {
        unsafe { core::intrinsics::fsub_fast(self, rhs) }
    }
    fn fast_mul(self, rhs: Self) -> Self {
        unsafe { core::intrinsics::fmul_fast(self, rhs) }
    }
    fn fast_div(self, rhs: Self) -> Self {
        // unsafe { core::intrinsics::fdiv_fast(self, rhs) }
        self.fast_mul(rhs.fast_recip())
    }
    fn fast_rem(self, rhs: Self) -> Self {
        unsafe { core::intrinsics::frem_fast(self, rhs) }
    }
    fn fast_recip(self) -> Self;
    fn fast_sqrt(self) -> Self;
    fn fast_sin(self) -> Self;
    fn fast_cos(self) -> Self;
    fn fast_rsqrt(self) -> Self;
    fn fast_log2(self) -> Self;
    fn fast_exp2(self) -> Self;

    fn fast_log10(self) -> Self;
    fn fast_ln(self) -> Self;
    fn fast_exp(self) -> Self;

    fn fast_abs(self) -> Self;
    fn fast_copysign(self, other: Self) -> Self;
    fn fast_trunc(self) -> Self;
    fn fast_fract(self) -> Self {
        self - self.fast_trunc()
    }
    fn fast_ceil(self) -> Self;
    fn fast_floor(self) -> Self;
}

impl FastNum for f32 {
    fn fma(self, b: Self, c: Self) -> Self {
        // unsafe { fma_rn_ftz(self, b, c) }
        unsafe { core::intrinsics::fmaf32(self, b, c) }
    }

    fn fast_div(self, rhs: Self) -> Self {
        // unsafe { div_approx(self, rhs) }
        unsafe { core::intrinsics::fdiv_fast(self, rhs) }
        // self / rhs
    }

    fn fast_recip(self) -> Self {
        unsafe { recip_approx(self) }
    }

    fn fast_sqrt(self) -> Self {
        unsafe { sqrt_approx(self) }
        // unsafe { core::intrinsics::sqrtf32(self) }
    }

    fn fast_sin(self) -> Self {
        unsafe { sin_approx(self) }
    }

    fn fast_cos(self) -> Self {
        unsafe { cos_approx(self) }
    }

    fn fast_rsqrt(self) -> Self {
        unsafe { rsqrt_approx(self) }
    }

    fn fast_log2(self) -> Self {
        unsafe { lg2_approx(self) }
    }

    fn fast_exp2(self) -> Self {
        unsafe { ex2_approx(self) }
    }

    fn fast_log10(self) -> Self {
        const RECIP_LOG2_10: f32 = 1f32 / core::f32::consts::LOG2_10;
        self.fast_log2() * RECIP_LOG2_10
    }

    fn fast_ln(self) -> Self {
        const RECIP_LOG2_E: f32 = 1f32 / core::f32::consts::LOG2_E;
        self.fast_log2() * RECIP_LOG2_E
    }

    fn fast_exp(self) -> Self {
        (self * core::f32::consts::LOG2_E).fast_exp2()
    }

    fn fast_abs(self) -> Self {
        unsafe { core::intrinsics::fabsf32(self) }
    }

    fn fast_copysign(self, other: Self) -> Self {
        unsafe { core::intrinsics::copysignf32(self, other) }
    }

    fn fast_trunc(self) -> Self {
        unsafe { core::intrinsics::truncf32(self) }
    }

    fn fast_ceil(self) -> Self {
        unsafe { core::intrinsics::ceilf32(self) }
    }

    fn fast_floor(self) -> Self {
        unsafe { core::intrinsics::floorf32(self) }
    }
}

impl FastNum for f64 {
    fn fma(self, b: Self, c: Self) -> Self {
        unsafe { core::intrinsics::fmaf64(self, b, c) }
    }

    fn fast_recip(self) -> Self {
        unsafe { recip_approx_f64(self) }
    }

    fn fast_sqrt(self) -> Self {
        unsafe { core::intrinsics::sqrtf64(self) }
    }

    fn fast_sin(self) -> Self {
        libm::sin(self)
    }

    fn fast_cos(self) -> Self {
        libm::cos(self)
    }

    fn fast_rsqrt(self) -> Self {
        unsafe { rsqrt_approx_f64(self) }
    }

    fn fast_log2(self) -> Self {
        libm::log2(self)
        // unsafe { lg2_approx_f64(self) }
    }

    fn fast_exp2(self) -> Self {
        libm::exp2(self)
        // unsafe { ex2_approx_f64(self) }
    }

    fn fast_log10(self) -> Self {
        libm::log10(self)
        // const RECIP_LOG2_10: f64 = 1f64 / core::f64::consts::LOG2_10;
        // self.fast_log2() * RECIP_LOG2_10
    }

    fn fast_ln(self) -> Self {
        libm::log(self)
        // const RECIP_LOG2_E: f64 = 1f64 / core::f64::consts::LOG2_E;
        // self.fast_log2() * RECIP_LOG2_E
    }

    fn fast_exp(self) -> Self {
        libm::exp(self)
        // (self * core::f64::consts::LOG2_E).fast_exp2()
    }

    fn fast_abs(self) -> Self {
        unsafe { core::intrinsics::fabsf64(self) }
    }

    fn fast_copysign(self, other: Self) -> Self {
        unsafe { core::intrinsics::copysignf64(self, other) }
    }

    fn fast_trunc(self) -> Self {
        unsafe { core::intrinsics::truncf64(self) }
    }

    fn fast_ceil(self) -> Self {
        unsafe { core::intrinsics::ceilf64(self) }
    }

    fn fast_floor(self) -> Self {
        unsafe { core::intrinsics::floorf64(self) }
    }
}

#[repr(transparent)]
#[derive(
    Debug,
    Display,
    Binary,
    Octal,
    LowerHex,
    UpperHex,
    LowerExp,
    UpperExp,
    Clone,
    Copy,
    From,
    Deref,
    DerefMut,
    Neg,
)]
pub struct FastFloat<F>(pub F);

impl<F: FastNum> PartialOrd for FastFloat<F> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if FloatCore::is_nan(*self) || FloatCore::is_nan(*other) {
            unsafe { core::hint::unreachable_unchecked() }
        } else if self.0 < other.0 {
            Some(core::cmp::Ordering::Less)
        } else if self.0 == other.0 {
            Some(core::cmp::Ordering::Equal)
        } else {
            Some(core::cmp::Ordering::Greater)
        }
    }
}

impl<F: FastNum> PartialEq for FastFloat<F> {
    fn eq(&self, other: &Self) -> bool {
        if FloatCore::is_nan(*self) || FloatCore::is_nan(*other) {
            unsafe { core::hint::unreachable_unchecked() }
        }
        self.0 == other.0
    }
}

impl<F: FastNum> Add for FastFloat<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.fast_add(*rhs))
    }
}

impl<F: FastNum> AddAssign for FastFloat<F> {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self).add(rhs);
    }
}

impl<F: FastNum> Sub for FastFloat<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.fast_sub(*rhs))
    }
}

impl<F: FastNum> SubAssign for FastFloat<F> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = (*self).sub(rhs);
    }
}

impl<F: FastNum> Mul for FastFloat<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.fast_mul(*rhs))
    }
}

impl<F: FastNum> MulAssign for FastFloat<F> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = (*self).mul(rhs);
    }
}

impl<F: FastNum> Div for FastFloat<F> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.fast_div(*rhs))
    }
}

impl<F: FastNum> DivAssign for FastFloat<F> {
    fn div_assign(&mut self, rhs: Self) {
        *self = (*self).div(rhs);
    }
}

impl<F: FastNum> Rem for FastFloat<F> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.fast_rem(*rhs))
    }
}

impl<F: FastNum> RemAssign for FastFloat<F> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = (*self).rem(rhs);
    }
}

impl<F: FastNum> FastFloat<F> {
    pub fn rsqrt(self) -> Self {
        Self(self.fast_rsqrt())
    }

    pub fn copysign(self, other: Self) -> Self {
        Self(self.fast_copysign(*other))
    }
}

impl<F: FastNum> Zero for FastFloat<F> {
    fn zero() -> Self {
        Self(Zero::zero())
    }

    fn is_zero(&self) -> bool {
        Zero::is_zero(&self.0)
    }
}

impl<F: FastNum> One for FastFloat<F> {
    fn one() -> Self {
        Self(One::one())
    }
}

impl<F: FastNum> Num for FastFloat<F> {
    type FromStrRadixErr = <F as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Num::from_str_radix(str, radix).map(FastFloat)
    }
}

impl<F: FastNum> ToPrimitive for FastFloat<F> {
    fn to_isize(&self) -> Option<isize> {
        ToPrimitive::to_isize(&self.0)
    }

    fn to_i8(&self) -> Option<i8> {
        ToPrimitive::to_i8(&self.0)
    }

    fn to_i16(&self) -> Option<i16> {
        ToPrimitive::to_i16(&self.0)
    }

    fn to_i32(&self) -> Option<i32> {
        ToPrimitive::to_i32(&self.0)
    }

    fn to_i64(&self) -> Option<i64> {
        ToPrimitive::to_i64(&self.0)
    }

    #[cfg(feature = "i128")]
    fn to_i128(&self) -> Option<i128> {
        ToPrimitive::to_i128(&self.0)
    }

    fn to_usize(&self) -> Option<usize> {
        ToPrimitive::to_usize(&self.0)
    }

    fn to_u8(&self) -> Option<u8> {
        ToPrimitive::to_u8(&self.0)
    }

    fn to_u16(&self) -> Option<u16> {
        ToPrimitive::to_u16(&self.0)
    }

    fn to_u32(&self) -> Option<u32> {
        ToPrimitive::to_u32(&self.0)
    }

    fn to_u64(&self) -> Option<u64> {
        ToPrimitive::to_u64(&self.0)
    }

    #[cfg(feature = "i128")]
    fn to_u128(&self) -> Option<u128> {
        ToPrimitive::to_u128(&self.0)
    }

    fn to_f32(&self) -> Option<f32> {
        ToPrimitive::to_f32(&self.0)
    }

    fn to_f64(&self) -> Option<f64> {
        ToPrimitive::to_f64(&self.0)
    }
}

impl<F: FastNum> NumCast for FastFloat<F> {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        NumCast::from(n).map(Self)
    }
}

impl<T: 'static + Copy, F: FastNum + AsPrimitive<T>> AsPrimitive<T> for FastFloat<F> {
    fn as_(self) -> T {
        self.0.as_()
    }
}

macro_rules! impl_as_primitive {
    ($($T: ty),+) => {
        $(impl<F: FastNum> AsPrimitive<FastFloat<F>> for $T where $T: AsPrimitive<F> {
            fn as_(self) -> FastFloat<F> {
                FastFloat(self.as_())
            }
        })+
    };
}

impl_as_primitive! { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64 }

impl<F: FastNum> FloatCore for FastFloat<F> {
    fn infinity() -> Self {
        Self(FloatCore::infinity())
    }

    fn neg_infinity() -> Self {
        Self(FloatCore::neg_infinity())
    }

    fn nan() -> Self {
        Self(FloatCore::nan())
    }

    fn neg_zero() -> Self {
        Self(FloatCore::neg_zero())
    }

    fn min_value() -> Self {
        Self(FloatCore::min_value())
    }

    fn min_positive_value() -> Self {
        Self(FloatCore::min_positive_value())
    }

    fn epsilon() -> Self {
        Self(FloatCore::epsilon())
    }

    fn max_value() -> Self {
        Self(FloatCore::max_value())
    }

    fn classify(self) -> core::num::FpCategory {
        FloatCore::classify(*self)
    }

    fn to_degrees(self) -> Self {
        Self(FloatCore::to_degrees(*self))
    }

    fn to_radians(self) -> Self {
        Self(FloatCore::to_radians(*self))
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        FloatCore::integer_decode(*self)
    }

    fn abs(self) -> Self {
        Self(self.fast_abs())
    }

    fn floor(self) -> Self {
        Self(self.fast_floor())
    }

    fn ceil(self) -> Self {
        Self(self.fast_ceil())
    }

    fn trunc(self) -> Self {
        Self(self.fast_trunc())
    }

    fn fract(self) -> Self {
        Self(self.fast_fract())
    }

    fn is_nan(self) -> bool {
        false
    }

    fn is_sign_positive(self) -> bool {
        self >= Self::zero()
    }

    fn is_sign_negative(self) -> bool {
        self < Self::zero()
    }

    fn recip(self) -> Self {
        Self(self.fast_recip())
    }
}

impl<F: FastNum> Float for FastFloat<F> {
    fn nan() -> Self {
        FloatCore::nan()
    }

    fn infinity() -> Self {
        FloatCore::infinity()
    }

    fn neg_infinity() -> Self {
        FloatCore::neg_infinity()
    }

    fn neg_zero() -> Self {
        FloatCore::neg_zero()
    }

    fn min_value() -> Self {
        FloatCore::min_value()
    }

    fn min_positive_value() -> Self {
        FloatCore::min_positive_value()
    }

    fn max_value() -> Self {
        FloatCore::max_value()
    }

    fn is_nan(self) -> bool {
        FloatCore::is_nan(self)
    }

    fn is_infinite(self) -> bool {
        FloatCore::is_infinite(self)
    }

    fn is_finite(self) -> bool {
        FloatCore::is_finite(self)
    }

    fn is_normal(self) -> bool {
        FloatCore::is_normal(self)
    }

    fn classify(self) -> core::num::FpCategory {
        FloatCore::classify(self)
    }

    fn floor(self) -> Self {
        FloatCore::floor(self)
    }

    fn ceil(self) -> Self {
        FloatCore::ceil(self)
    }

    fn round(self) -> Self {
        FloatCore::round(self)
    }

    fn trunc(self) -> Self {
        Self(self.fast_trunc())
    }

    fn fract(self) -> Self {
        Self(self.fast_fract())
    }

    fn abs(self) -> Self {
        Self(self.fast_abs())
    }

    fn signum(self) -> Self {
        FloatCore::signum(self)
    }

    fn is_sign_positive(self) -> bool {
        FloatCore::is_sign_positive(self)
    }

    fn is_sign_negative(self) -> bool {
        FloatCore::is_sign_negative(self)
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Self(self.fma(*a, *b))
    }

    fn recip(self) -> Self {
        Self(self.fast_recip())
    }

    fn powi(self, n: i32) -> Self {
        FloatCore::powi(self, n)
    }

    fn powf(self, n: Self) -> Self {
        Self(Float::powf(self.0, n.0))
    }

    fn sqrt(self) -> Self {
        Self(self.fast_sqrt())
    }

    fn exp(self) -> Self {
        Self(self.fast_exp())
    }

    fn exp2(self) -> Self {
        Self(self.fast_exp2())
    }

    fn ln(self) -> Self {
        Self(self.fast_ln())
    }

    fn log(self, base: Self) -> Self {
        <Self as Float>::log2(self) / <Self as Float>::log2(base)
    }

    fn log2(self) -> Self {
        Self(self.fast_log2())
    }

    fn log10(self) -> Self {
        Self(self.fast_log10())
    }

    fn to_degrees(self) -> Self {
        FloatCore::to_degrees(self)
    }

    fn to_radians(self) -> Self {
        FloatCore::to_radians(self)
    }

    fn max(self, other: Self) -> Self {
        FloatCore::max(self, other)
    }

    fn min(self, other: Self) -> Self {
        FloatCore::min(self, other)
    }

    fn abs_sub(self, other: Self) -> Self {
        if self <= other {
            Self::zero()
        } else {
            self - other
        }
    }

    fn cbrt(self) -> Self {
        Self(Float::cbrt(self.0))
    }

    fn hypot(self, other: Self) -> Self {
        Self(Float::hypot(self.0, other.0))
    }

    fn sin(self) -> Self {
        Self(self.fast_sin())
    }

    fn cos(self) -> Self {
        Self(self.fast_cos())
    }

    fn tan(self) -> Self {
        // <Self as Float>::sin(self) / <Self as Float>::cos(self)
        Self(Float::tan(self.0))
    }

    fn asin(self) -> Self {
        Self(Float::asin(self.0))
    }

    fn acos(self) -> Self {
        Self(Float::acos(self.0))
    }

    fn atan(self) -> Self {
        Self(Float::atan(self.0))
    }

    fn atan2(self, other: Self) -> Self {
        Self(Float::atan2(self.0, other.0))
    }

    fn sin_cos(self) -> (Self, Self) {
        (<Self as Float>::sin(self), <Self as Float>::cos(self))
    }

    fn exp_m1(self) -> Self {
        Self(Float::exp_m1(self.0))
    }

    fn ln_1p(self) -> Self {
        Self(Float::ln_1p(self.0))
    }

    fn sinh(self) -> Self {
        Self(Float::sinh(self.0))
    }

    fn cosh(self) -> Self {
        Self(Float::cosh(self.0))
    }

    fn tanh(self) -> Self {
        Self(Float::tanh(self.0))
    }

    fn asinh(self) -> Self {
        Self(Float::asinh(self.0))
    }

    fn acosh(self) -> Self {
        Self(Float::acosh(self.0))
    }

    fn atanh(self) -> Self {
        Self(Float::atanh(self.0))
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        FloatCore::integer_decode(self)
    }
}

use float_eq::*;

impl<F: FloatEqUlpsTol> FloatEqUlpsTol for FastFloat<F>
where
    UlpsTol<F>: Sized,
{
    type UlpsTol = FastFloat<UlpsTol<F>>;
}

impl<F: FloatEqDebugUlpsDiff> FloatEqDebugUlpsDiff for FastFloat<F> {
    type DebugUlpsDiff = FastFloat<DebugUlpsDiff<F>>;
}

impl<F> FloatEq for FastFloat<F>
where
    F: FloatEq + FloatEqUlpsTol,
    F::Tol: Sized,
    UlpsTol<F>: Sized,
    UlpsTol<F::Tol>: Sized,
{
    type Tol = FastFloat<F::Tol>;

    fn eq_abs(&self, other: &Self, max_diff: &Self::Tol) -> bool {
        self.0.eq_abs(&other.0, &max_diff.0)
    }

    fn eq_rmax(&self, other: &Self, max_diff: &Self::Tol) -> bool {
        self.0.eq_rmax(&other.0, &max_diff.0)
    }

    fn eq_rmin(&self, other: &Self, max_diff: &Self::Tol) -> bool {
        self.0.eq_rmin(&other.0, &max_diff.0)
    }

    fn eq_r1st(&self, other: &Self, max_diff: &Self::Tol) -> bool {
        self.0.eq_r1st(&other.0, &max_diff.0)
    }

    fn eq_r2nd(&self, other: &Self, max_diff: &Self::Tol) -> bool {
        self.0.eq_r2nd(&other.0, &max_diff.0)
    }

    fn eq_ulps(&self, other: &Self, max_diff: &UlpsTol<Self::Tol>) -> bool {
        self.0.eq_ulps(&other.0, &max_diff.0)
    }
}

impl<F> AssertFloatEq for FastFloat<F>
where
    F: FloatEqUlpsTol + AssertFloatEq + core::fmt::Debug,
    F::Tol: Sized,
    F::DebugTol: Sized,
    UlpsTol<F>: Sized,
    UlpsTol<F::Tol>: Sized,
    UlpsTol<F::DebugTol>: Sized,
{
    type DebugAbsDiff = FastFloat<F::DebugAbsDiff>;

    type DebugTol = FastFloat<F::DebugTol>;

    fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
        FastFloat(self.0.debug_abs_diff(&other.0))
    }

    fn debug_ulps_diff(&self, other: &Self) -> DebugUlpsDiff<Self::DebugAbsDiff> {
        FastFloat(self.0.debug_ulps_diff(&other.0))
    }

    fn debug_abs_tol(&self, other: &Self, max_diff: &Self::Tol) -> Self::DebugTol {
        FastFloat(self.0.debug_abs_tol(&other.0, &max_diff.0))
    }

    fn debug_rmax_tol(&self, other: &Self, max_diff: &Self::Tol) -> Self::DebugTol {
        FastFloat(self.0.debug_rmax_tol(&other.0, &max_diff.0))
    }

    fn debug_rmin_tol(&self, other: &Self, max_diff: &Self::Tol) -> Self::DebugTol {
        FastFloat(self.0.debug_rmin_tol(&other.0, &max_diff.0))
    }

    fn debug_r1st_tol(&self, other: &Self, max_diff: &Self::Tol) -> Self::DebugTol {
        FastFloat(self.0.debug_r1st_tol(&other.0, &max_diff.0))
    }

    fn debug_r2nd_tol(&self, other: &Self, max_diff: &Self::Tol) -> Self::DebugTol {
        FastFloat(self.0.debug_r2nd_tol(&other.0, &max_diff.0))
    }

    fn debug_ulps_tol(&self, other: &Self, max_diff: &UlpsTol<Self::Tol>) -> UlpsTol<Self::DebugTol>
    where
        UlpsTol<Self::DebugTol>: Sized,
    {
        FastFloat(self.0.debug_ulps_tol(&other.0, &max_diff.0))
    }
}
