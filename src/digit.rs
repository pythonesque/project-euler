use std::cmp::{max, min};
use std::iter::iterate;
use std::fmt;
use std::fmt::Show;
use std::mem;
use std::num::{One, Zero};
use std::u8;

#[deriving(Clone, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Digit<B>(u8);

impl<B: Base> fmt::Show for Digit<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Base::fmt(self, f)
    }
}

impl<B: Base> Digit<B> {
    /// precondition: digit < Base::base::<Digit<B>>()
    pub fn make(digit: u8) -> Digit<B> {
        assert!(digit < Base::base(None::<B>));
        Digit(digit)
    }

    pub fn value(&self) -> u8 {
        let Digit(value) = *self;
        value
    }
}

impl<B: Base> Not<Digit<B>> for Digit<B> {
    fn not(&self) -> Digit<B> {
        Digit(Base::base(None::<B>) - self.value())
    }
}

impl<B: Base> Add<Digit<B>, (Digit<B>, Digit<B>)> for Digit<B> {
    fn add(&self, other: &Digit<B>) -> (Digit<B>, Digit<B>) {
        let sum = self.value() as u16 + other.value() as u16;
        (
            Digit((sum / Base::base(None::<B>) as u16) as u8),
            Digit((sum % Base::base(None::<B>) as u16) as u8)
        )
    }
}

impl<B: Base> Mul<Digit<B>, (Digit<B>, Digit<B>)> for Digit<B> {
    fn mul(&self, other: &Digit<B>) -> (Digit<B>, Digit<B>) {
        let prod = self.value() as u16 * other.value() as u16;
        (
            Digit((prod / Base::base(None::<B>) as u16) as u8),
            Digit((prod % Base::base(None::<B>) as u16) as u8)
        )
    }
}

pub trait Base {
    /// to implement this trait, you must have base() > 1.  It takes an Option to work around
    /// Rust's current inability to handle such function signatures properly.  FIXME: remove this
    /// when Rust supports it.
    fn base(Option<Self>) -> u8;

    fn fmt(digit: &Digit<Self>, f: &mut fmt::Formatter) -> fmt::Result;
}

#[deriving(Clone)]
pub struct Base10;

impl Base for Base10 {
    fn base(_: Option<Base10>) -> u8 { 10 }

    fn fmt(digit: &Digit<Base10>, f: &mut fmt::Formatter) -> fmt::Result {
        digit.value().fmt(f)
    }
}

#[deriving(Clone, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Digits<B>(Vec<Digit<B>>);

impl<B: Base> Digits<B> {
    pub fn make(digits: Vec<Digit<B>>) -> Digits<B> {
        Digits(digits)
    }

    pub fn get_ref(&self) -> &Vec<Digit<B>> {
        let Digits(ref value) = *self;
        value
    }

    pub fn get_mut_ref(&mut self) -> &mut Vec<Digit<B>> {
        let Digits(ref mut value) = *self;
        value
    }

    pub fn add_in_place(&mut self, other: &Digits<B>) {
        let (digits, other) = (self.get_mut_ref(), other.get_ref() );
        let mut move_iter = other.iter();
        let mut carry = Digit(0);
        let zero = Digit(0);
        {
            let mut mut_iter = digits.mut_iter();
            let mut iter = mut_iter.by_ref()
                .zip(move_iter.by_ref().chain(iterate( |_: &Digit<B>| &zero , &zero )));
            // First, sum all aligned digits between self and other.
            for (digit, other) in iter {
                let (first_carry, remainder) = carry + *digit;
                let (second_carry, remainder) = remainder + *other;
                *digit = remainder;
                // Third carry is always zero
                let (_, remainder) = first_carry + second_carry;
                carry = remainder;
            }
        }
        // Extend self with any remaining elements from other.
        digits.extend(move_iter.map( |&digit| {
            let (carry_, remainder) = carry + digit;
            carry = carry_;
            remainder
        }));
        // If the carry flag is nonzero, push it onto the end of the vector.
        if carry.value() != 0 {
            digits.push(carry);
        }
    }
}

impl<B: Base> FromPrimitive for Digits<B> {
    fn from_i64(n: i64) -> Option<Digits<B>> {
        FromPrimitive::from_u64(n as u64)
    }

    fn from_u64(mut n: u64) -> Option<Digits<B>> {
        let mut vec = Vec::new();
        let base = Base::base(None::<B>) as u64;
        while n != 0 {
            vec.push(Digit::make((n % base) as u8));
            n /= base;
        }
        Some(Digits(vec))
    }
}

impl<B: Base> fmt::Show for Digits<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         for digit in self.get_ref().iter().rev() {
             try!(digit.fmt(f))
         }
         Ok(())
    }
}

impl<B: Base + Clone> One for Digits<B> {
    fn one() -> Digits<B> { Digits(vec!(Digit(1))) }
}

impl<B: Base + Clone> Zero for Digits<B> {
    fn zero() -> Digits<B> { Digits(Vec::new()) }
    fn is_zero(&self) -> bool {
        self.get_ref().iter().all( |&num| num.value() == 0 )
    }
}

impl<B: Base + Clone> Add<Digits<B>, Digits<B>> for Digits<B> {
    /// Inefficient, but convenient.
    fn add(&self, other: &Digits<B>) -> Digits<B> {
        let mut carry = Digit(0);
        let (short, long) = (self.get_ref(), other.get_ref());
        let (short, long) = if short.len() < long.len() { (short, long) } else { (long, short) };
        let mut digits = long.clone();
        let zero = Digit(0);
        {
            let mut iter = digits.mut_iter()
                .zip(short.iter().chain(iterate( |_: &Digit<B>| &zero , &zero)));
            // We add digits backwards, but it shouldn't matter as long as we're consistent about it.
            for (digit, other) in iter {
                let (first_carry, remainder) = *digit + carry;
                let (second_carry, remainder) = remainder + *other;
                *digit = remainder;
                let (_, remainder) = first_carry + second_carry;
                carry = remainder;
            }
        }
        if carry.value() != 0 {
            digits.push(carry)
        }
        Digits(digits)
    }
}

impl<B: Base + Clone> Mul<Digits<B>, Digits<B>> for Digits<B> {
    /// Similarly inefficient / convenient.
    fn mul(&self, other: &Digits<B>) -> Digits<B> {
        let (short, long) = (self.get_ref(), other.get_ref());
        let (slen, llen) = (short.len(), long.len());
        let (short, long, slen, llen) = if short.len() < long.len() {
            (short, long, slen, llen)
        } else {
            (long, short, llen, slen)
        };
        // Justification for using (uint, u8) for the carry: behind the scenes, a Vec is backed by
        // an array, and all array indexing is of size uint.  So the maximal representable number
        // is b^uint::MAX - 1, and the maximum number of digits is uint::MAX or less.  Now
        // consider the scenario where we are multiplying the largest representable number by
        // itself.  From the formula for calculating division:
        //    r_i = c_i + Sum{j + k = i}(a_j * b_k)
        // where a_i and b_i are the ith bits from the right of the first and second operands, r_i
        // is the ith bit from the right of the total sum for the result at column i, and c_i is
        // the carry from the previous column (i starts at 0, and c_0 = 0).
        // Now, let P(i) <=> r_i <= b * (b - 1) * (i + 1) and c_{i + 1} <= (b - 1) * (i + 1)
        // for 2 <= b < 256
        // BC:
        //  r_0 = c_0 + Sum{j + k = 0}(a_j * b_k) = 0 + a_0 * b_0 <= (b - 1) * (b - 1)
        //* =>  r_i <= b * (b - 1) * (i + 1)
        //  =>  r_0 <= b^2 - 2b + 1 => r_0 <= b(b - 2) + 1 => floor(r_0 / b) <= floor(b - 2 + 1/b)
        //  =>  c_1 <= b - 2 + 0 = b - 2 <= b - 1 = (b - 1) * (0 + 1)
        //* => c_{i+1} <= (b - 1) * (i + 1)
        // IS: Suppose P(i).
        //  r_{i + 1} = c_{i + 1} + Sum{j + k = i + 1}(a_j * b_k)
        //  => r_{i + 1} <= (b - 1)(i + 1) + (i + 2)(b - 1)^2 = (b - 1)(b(i + 2) - 1)
        //* => r_{i + 1} <= b(b - 1)(i + 2)
        //  c_{i + 2} = floor(r_{i + 1} / b) <= floor((b - 1)(i + 2))
        //* => c_{i + 2} <= (b - 1) (i + 2)
        //  Thus, P(i) => P(i + 1).
        //  We conclude that P(i) for all b >= 2, i >= 0.
        //  Then we need at most lg(255 * (255 - 1) * (uint::MAX)) < lg(256 * 256 * (uint::MAX))
        //  = 16 * uint::BITS bits for the full sum, and < lg(255 * (uint::MAX)) = 8 * uint::BITS
        //  bits for the carry.
        let mut digits = Vec::with_capacity(slen + llen);
        let (mut carry1, mut carry0, mut sum) = (0u, Digit(0), Digit(0));
        let zero = Digit(0);
        {
            for i in range(0u, max(1, llen) + slen - 1) {
                let (j_initial, j_final) = (max(llen, i + 1) - llen, min(i + 1, llen));
                let (carry1_, carry0_, sum_) = range(j_initial, j_final)
                    .fold((carry1, carry0, sum), |(mut carry1, carry0, sum), j| {
                        let k = i - j;
                        let digit = long[j];
                        let other = if k < slen { &short[k] } else { &zero };
                        let (first_carry, product) = digit * *other;
                        let (second_carry, sum) = sum + product;
                        let (third_carry, carry0) = carry0 + first_carry;
                        let (fourth_carry, carry0) = carry0 + second_carry;
                        carry1 += third_carry.value() as uint + fourth_carry.value() as uint;
                        (carry1, carry0, sum)
                    });
                digits.push(sum_);
                sum = carry0_;
                carry0 = Digit((carry1_ & u8::MAX as uint) as u8);
                carry1 = carry1_ >> u8::BITS as uint;
            }
        }
        if sum.value() != 0 {
            digits.push(sum);
        }
        Digits(digits)
    }
}

pub fn double_digits(digits: &mut Digits<Base10>) {
    let mut sum = *digits + *digits;
    mem::swap(digits, &mut sum)
}
