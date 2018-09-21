use num_traits;
use num_traits::Signed;
use approx;
use d128;


/// Set up for num traits required by real
impl num_traits::Bounded for d128 {
    fn max_value() -> d128 {
        d128!(Infinity).previous()
    }
    fn min_value() -> d128 {
        d128!(-Infinity).next()
    }
}


impl num_traits::Signed for d128 {
    fn abs(&self) -> d128 {
        d128::abs(*self)
    }

    fn abs_sub(&self, other: &d128) -> d128 {
        if &self <= &other {d128::zero()} else {self - other}
    }

    fn signum(&self) -> d128 {
        match &self {
            n if n.is_positive() => d128!(1),
            n if n.is_negative() => d128!(-1),
            _                    => d128::zero(),
        }
    }

    fn is_positive(&self) -> bool {
        (*self).is_positive()
    }

    fn is_negative(&self) -> bool {
        (*self).is_negative()
    }
}

impl num_traits::FromPrimitive for d128 {
    fn from_i64(n: i64) -> Option<d128> {
        Some(d128::from(n))
    }
    fn from_u64(n: u64) -> Option<d128> {
        Some(d128::from(n))
    }
}


#[derive(Debug)]
pub enum DecimalErrorKind {
    Empty,
    Invalid,
}

#[derive(Debug)]
pub struct ParseDecimalError {
    pub kind: DecimalErrorKind,
}

// TODO: actually implement this
impl num_traits::Num for d128 {
    type FromStrRadixErr = ParseDecimalError;

    fn from_str_radix(src: &str, radix: u32) -> Result<d128, Self::FromStrRadixErr> {
        use self::DecimalErrorKind::*;
        use self::ParseDecimalError as PDE;

        Ok(d128!(0.0))
    }
}

impl num_traits::One for d128 {
    fn one() -> d128 {
        d128!(1.0)
    }
}

impl num_traits::Zero for d128 {
    fn zero() -> d128 {
        d128::zero()
    }

    fn is_zero(&self) -> bool {
        *self == d128::zero()
    }
}

impl approx::RelativeEq for d128 {
    fn default_max_relative() -> d128 {
      d128::zero().next()
    }

    fn relative_eq(&self, other: &d128, epsilon: d128, max_relative: d128) -> bool {
        if self == other {
          return true;
        }

        if (*self).is_infinite() || (*other).is_infinite() {
          return false;
        }

        let abs_diff = (self - other).abs();

        if abs_diff <= epsilon {
          return true;
        }

        let abs_self = (*self).abs();
        let abs_other = (*self).abs();

        let largest = match abs_other > abs_self {
            true  => abs_other,
            false => abs_self
        };

        abs_diff <= largest * max_relative
    }
}

impl approx::AbsDiffEq for d128 {
    type Epsilon = d128; 

    fn default_epsilon() -> d128 {
        d128::zero().next()
    }

    fn abs_diff_eq(&self, other: &d128, epsilon: d128) -> bool {
      (self - other).abs() <= epsilon
    }
}

impl approx::UlpsEq for d128 {
    fn default_max_ulps() -> u32 {
        4
    }

    fn ulps_eq(&self, other: &d128, epsilon: d128, max_ulps: u32) -> bool {
        if  (self - other).abs() <= epsilon {
            return true;
        }

        if self.signum() != other.signum() {
            return false;
        }

        let u32_self:u32 = d128::into(*self);
        let u32_other:u32 = d128::into(*other);
        u32_self - u32_other <= max_ulps
    }
}
