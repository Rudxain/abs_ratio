//#![no_std]
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

#[allow(clippy::suboptimal_flops)]
#[must_use]
#[inline]
fn abs(x: f64) -> f64 {
	if x.is_sign_negative() {
		-x
	} else {
		x
	}
}

#[must_use]
#[inline]
pub fn abs_ratio(x: f64, y: f64) -> f64 {
	let [x, y] = [abs(x), abs(y)];
	x.max(y) / x.min(y)
}

#[must_use]
#[inline]
pub const fn abs_ratio_int(x: i128, y: i128) -> u128 {
	let [x, y] = [x.unsigned_abs(), y.unsigned_abs()];
	if x < y {
		y / x
	} else {
		x / y
	}
}

#[must_use]
#[inline]
pub const fn checked_abs_ratio_int(x: i128, y: i128) -> Option<u128> {
	let [x, y] = [x.unsigned_abs(), y.unsigned_abs()];
	if x < y {
		y.checked_div(x)
	} else {
		x.checked_div(y)
	}
}

#[cfg(test)]
#[test]
fn abs_test() {
	use core::cmp::Ordering::Equal;

	assert!(abs(-0.0).total_cmp(&0.0) == Equal);
	assert!(abs(-f64::NAN).total_cmp(&f64::NAN) == Equal);
	assert!(abs(f64::NEG_INFINITY).total_cmp(&f64::INFINITY) == Equal);
}

#[cfg(test)]
mod tests {
	#![allow(clippy::float_cmp)]
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(abs_ratio(2.0, 2.0), 1.0);
	}
}
