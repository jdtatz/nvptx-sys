use core::ops::*;

pub trait Float:
'static
+ Copy
+ Clone
+ PartialEq
+ PartialOrd
+ Neg<Output=Self>
+ Add<Output=Self>
+ Sub<Output=Self>
+ Mul<Output=Self>
+ Div<Output=Self>
+ Rem<Output=Self>
+ AddAssign
+ SubAssign
+ MulAssign
+ DivAssign
+ RemAssign
{
    const ZERO: Self;
    const ONE: Self;
    fn abs(self) -> Self;
    fn acos(self) -> Self;
    fn acosh(self) -> Self;
    fn asin(self) -> Self;
    fn asinh(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn atanh(self) -> Self;
    fn cbrt(self) -> Self;
    fn ceil(self) -> Self;
    fn copysign(self, sign: Self) -> Self;
    fn cos(self) -> Self;
    fn cosh(self) -> Self;
    fn div_euclid(self, rhs: Self) -> Self {
        let q = (self / rhs).trunc();
        if self % rhs < Self::ZERO {
            return if rhs > Self::ZERO { q - Self::ONE } else { q + Self::ONE };
        }
        q
    }
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn exp_m1(self) -> Self;
    fn floor(self) -> Self;
    fn fract(self) -> Self {
        self - self.trunc()
    }
    fn hypot(self, other: Self) -> Self;
    fn ln(self) -> Self;
    fn ln_1p(self) -> Self;
    fn log(self, base: Self) -> Self {
        self.log2() / base.log2()
    }
    fn log10(self) -> Self;
    fn log2(self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn powf(self, n: Self) -> Self;
    /// WARNING: Float::powi(0, 0) = 1, while this is mathematically undefined,
    /// its the convention most programming languages & IEE 754-2008 use
    fn powi(self, mut n: i32) -> Self {
        if n == 0 {
            return Self::ONE;
        }
        let mut x = self;
        if n < 0 {
            n = -n;
            x = Self::ONE / x;
        }
        let mut y = Self::ONE;
        while n > 1 {
            if n & 1 == 1 {
                y *= x;
            }
            x *= x;
            n >>= 1;
        }
        x * y
    }
    fn rem_euclid(self, rhs: Self) -> Self {
        let r = self % rhs;
        if r < Self::ZERO { r + rhs.abs() } else { r }
    }
    fn round(self) -> Self;
    fn signum(self) -> Self;
    fn sin(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn sinh(self) -> Self;
    fn sqrt(self) -> Self;
    fn tan(self) -> Self;
    fn tanh(self) -> Self;
    fn trunc(self) -> Self;
}

impl Float for f32 {
    const ZERO: Self = 0f32;
    const ONE: Self = 1f32;

    fn abs(self) -> Self {
        unsafe { core::intrinsics::fabsf32(self) }
    }

    fn acos(self) -> Self {
        libm::acosf(self)
    }

    fn acosh(self) -> Self {
        libm::acoshf(self)
    }

    fn asin(self) -> Self {
        libm::asinf(self)
    }

    fn asinh(self) -> Self {
        libm::asinhf(self)
    }

    fn atan(self) -> Self {
        libm::atanf(self)
    }

    fn atan2(self, other: Self) -> Self {
        libm::atan2f(self, other)
    }

    fn atanh(self) -> Self {
        libm::atanhf(self)
    }

    fn cbrt(self) -> Self {
        libm::cbrtf(self)
    }

    fn ceil(self) -> Self {
        unsafe { core::intrinsics::ceilf32(self) }
    }

    fn copysign(self, sign: Self) -> Self {
        unsafe { core::intrinsics::copysignf32(self, sign) }
    }

    fn cos(self) -> Self {
        libm::cosf(self)
    }

    fn cosh(self) -> Self {
        libm::coshf(self)
    }

    fn exp(self) -> Self {
        libm::expf(self)
    }

    fn exp2(self) -> Self {
        libm::exp2f(self)
    }

    fn exp_m1(self) -> Self {
        libm::expm1f(self)
    }

    fn floor(self) -> Self {
        unsafe { core::intrinsics::floorf32(self) }
    }

    fn hypot(self, other: Self) -> Self {
        libm::hypotf(self, other)
    }

    fn ln(self) -> Self {
        libm::logf(self)
    }

    fn ln_1p(self) -> Self {
        libm::log1pf(self)
    }

    fn log10(self) -> Self {
        libm::log10f(self)
    }

    fn log2(self) -> Self {
        libm::log2f(self)
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        unsafe { core::intrinsics::fmaf32(self, a, b) }
    }

    fn powf(self, n: Self) -> Self {
        libm::powf(self, n)
    }

    fn round(self) -> Self {
        libm::roundf(self)
    }

    fn signum(self) -> Self {
        if self.is_nan() { Self::NAN } else { 1.0_f32.copysign(self) }
    }

    fn sin(self) -> Self {
        libm::sinf(self)
    }

    fn sin_cos(self) -> (Self, Self) {
        libm::sincosf(self)
    }

    fn sinh(self) -> Self {
        libm::sinhf(self)
    }

    fn sqrt(self) -> Self {
        unsafe { core::intrinsics::sqrtf32(self) }
    }

    fn tan(self) -> Self {
        libm::tanf(self)
    }

    fn tanh(self) -> Self {
        libm::tanhf(self)
    }

    fn trunc(self) -> Self {
        unsafe { core::intrinsics::truncf32(self) }
    }
}
