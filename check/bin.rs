#![crate_id = "interval-check"]
#![crate_type = "bin"]

extern crate interval;
extern crate quickcheck;

mod ray_check;
mod interval_check;

fn main() {
  ray_check::check();
  interval_check::check();

}
