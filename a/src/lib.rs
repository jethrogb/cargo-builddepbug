extern crate a_sys;

pub fn use_foo() {
	a_sys::foo();
}

#[cfg(feature = "bar")]
pub fn use_feature_foo() {
	a_sys::feature_foo();
}
