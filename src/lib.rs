#![no_std]

use core::{
	num::NonZeroI128,
	ops::{Div, Neg},
};

// https://github.com/rust-lang/rust/issues/50145
#[must_use]
#[inline]
const fn fabs(x: f64) -> f64 {
	if x.is_sign_negative() { -x } else { x }
}

#[must_use]
pub const fn abs_ratio_f(x: f64, y: f64) -> f64 {
	let [x, y] = [fabs(x), fabs(y)];
	x.max(y) / x.min(y)
}

#[must_use]
pub fn abs_ratio_nz(x: NonZeroI128, y: NonZeroI128) -> u128 {
	let [x, y] = [
		u128::from(x.unsigned_abs()),
		u128::from(y.unsigned_abs()),
	];
	if x < y { y / x } else { x / y }
}

#[must_use]
/// # Panics
/// when either arg is 0
pub const fn abs_ratio_i(x: i128, y: i128) -> u128 {
	let [x, y] = [x.unsigned_abs(), y.unsigned_abs()];
	if x < y { y / x } else { x / y }
}

#[must_use]
/// Returns `None` when either arg is 0
pub const fn checked_abs_ratio_i(
	x: i128,
	y: i128,
) -> Option<u128> {
	let [x, y] = [x.unsigned_abs(), y.unsigned_abs()];
	if x < y {
		y.checked_div(x)
	} else {
		x.checked_div(y)
	}
}

fn abs<T: Clone + Ord + Neg<Output = T>>(x: T) -> T {
	x.clone().max(-x)
}

pub fn abs_ratio<
	T: Clone + Ord + Neg<Output = T> + Div<Output = T>,
>(
	x: T,
	y: T,
) -> T {
	let [x, y] = [abs(x), abs(y)];
	x.clone().max(y.clone()) / x.min(y)
}

#[cfg(test)]
mod tests {
	#![allow(clippy::float_cmp)]
	use super::*;

	#[test]
	fn fabs_works() {
		use core::cmp::Ordering::Equal;

		assert_eq!(fabs(-0.0).total_cmp(&0.0), Equal);
		assert_eq!(fabs(0.0).total_cmp(&0.0), Equal);
		assert_eq!(fabs(-f64::NAN).total_cmp(&f64::NAN), Equal);
		assert_eq!(fabs(f64::NAN).total_cmp(&f64::NAN), Equal);

		assert_eq!(fabs(f64::NEG_INFINITY), f64::INFINITY);
		assert_eq!(fabs(f64::INFINITY), f64::INFINITY);
		assert_eq!(fabs(-1.0), 1.0);
		assert_eq!(fabs(2.8), 2.8);
	}

	#[test]
	fn f_works() {
		assert_eq!(abs_ratio_f(1.0, 2.0), 2.0);
		assert_eq!(abs_ratio_f(2.0, 1.0), 2.0);
		assert_eq!(abs_ratio_f(2.0, 2.0), 1.0);

		assert_eq!(abs_ratio_f(1.0, 0.0), f64::INFINITY);
		assert_eq!(abs_ratio_f(0.0, 1.0), f64::INFINITY);
		assert_eq!(abs_ratio_f(-0.0, 1.0), f64::INFINITY);
	}

	#[test]
	fn i_works() {
		assert_eq!(abs_ratio_i(2, 2), 1);
		assert_eq!(checked_abs_ratio_i(2, 2), Some(1));
		assert_eq!(checked_abs_ratio_i(1, 0), None);
		assert_eq!(checked_abs_ratio_i(0, 1), None);
	}
}
