#![no_std]
#![warn(
	unused,
	future_incompatible,
	clippy::cargo,
	clippy::pedantic,
	clippy::nursery,
	clippy::shadow_unrelated,
	clippy::decimal_literal_representation,
	clippy::unseparated_literal_suffix,
	clippy::empty_structs_with_brackets,
	clippy::cast_precision_loss,
	clippy::excessive_precision,
	clippy::lossy_float_literal
)]
#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code, clippy::exit)]
#![allow(clippy::cargo_common_metadata)]

use core::{
	num::NonZeroI128,
	ops::{Div, Neg},
};

// `abs` method only exists in `std`
#[must_use]
#[inline]
fn fabs(x: f64) -> f64 {
	if x.is_sign_negative() {
		-x
	} else {
		x
	}
}

#[must_use]
pub fn abs_ratio_f(x: f64, y: f64) -> f64 {
	let [x, y] = [fabs(x), fabs(y)];
	x.max(y) / x.min(y)
}

#[must_use]
pub fn abs_ratio_nz(x: NonZeroI128, y: NonZeroI128) -> u128 {
	let [x, y] = [u128::from(x.unsigned_abs()), u128::from(y.unsigned_abs())];
	if x < y {
		y / x
	} else {
		x / y
	}
}

#[must_use]
/// # Panics
/// when either arg is 0
pub const fn abs_ratio_i(x: i128, y: i128) -> u128 {
	let [x, y] = [x.unsigned_abs(), y.unsigned_abs()];
	if x < y {
		y / x
	} else {
		x / y
	}
}

#[must_use]
/// Returns `None` when either arg is 0
pub const fn checked_abs_ratio_i(x: i128, y: i128) -> Option<u128> {
	let [x, y] = [x.unsigned_abs(), y.unsigned_abs()];
	if x < y {
		y.checked_div(x)
	} else {
		x.checked_div(y)
	}
}

fn abs<'a, T>(x: &'a T) -> &'a T
where
	&'a T: Ord + Neg<Output = &'a T>,
{
	x.max(-x)
}

pub fn abs_ratio<'a, T>(x: &'a T, y: &'a T) -> T
where
	&'a T: Ord + Neg<Output = &'a T> + Div<Output = T>,
{
	let (x, y) = (abs(x), abs(y));
	x.max(y) / x.min(y)
}


#[cfg(test)]
#[test]
fn fabs_test() {
	use core::cmp::Ordering::Equal;

	assert!(fabs(-0.0).total_cmp(&0.0) == Equal);
	assert!(fabs(-f64::NAN).total_cmp(&f64::NAN) == Equal);
	assert!(fabs(f64::NEG_INFINITY).total_cmp(&f64::INFINITY) == Equal);
}

#[cfg(test)]
mod tests {
	#![allow(clippy::float_cmp)]
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(abs_ratio_f(2.0, 2.0), 1.0);
	}
}
