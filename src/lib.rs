#![feature(hash_drain_filter)]
#![feature(vec_retain_mut)]

#[macro_use]
extern crate scan_fmt;

use aoc_runner_derive::aoc_lib;

pub mod aoc2021;
aoc_lib! { year = 2021 }
