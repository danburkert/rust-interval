extern crate interval;
extern crate quickcheck;

use quickcheck::quickcheck;
use quickcheck::Arbitrary;
use interval::ray::Ray;

pub fn check() {
  quickcheck(ray_contains);
}

fn ray_contains(ray: Ray<int>, element: int) -> bool {
  ray.contains(&element)
}
