#![no_std]
#![crate_type="lib"]
#![crate_name="kits"]
#![feature(no_std)]
#![feature(core)]

#[macro_use]
extern crate core;
extern crate emlib;
extern crate modules;

#[cfg(feature = "stk3700")]
pub mod stk;

#[cfg(feature = "dk3750")]
pub mod dk;
