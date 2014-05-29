#[deriving(Eq, TotalEq, Ord, TotalOrd, Show, Rand, Clone)]

/// An `EndPoint` is a point on a continuous and infite set of total-ordered
/// values representing the exact value at which a `Bound` intersects the
/// set of values. An `EndPoint` may be:
///
///   * `Open`: an endpoint which includes the value in the bound.
///   * `Closed`: an endpoint which excludes the value in the bound.
///   * `None`: a non-existant endpoint. Any `Bound` with a `None` end point
///     will be effectively unbounded.
///
/// Type parameter `T` is the type of value contained in the continuous,
/// infinite, total-ordered set which the `EndPoint` operates on.
pub enum EndPoint<T> {
  Open(T),
  Closed(T),
  None
}
impl<T : TotalOrd> EndPoint<T> {

  fn getPoint(&self) -> Option<T> {
    match *self {
      Open(ref t) => Some(t),
      Closed(ref t) => Some(t),
      None => option::None
    }
  }

  fn isDefined(&self) -> bool {
    //! Returns true if this `EndPoint` is well-defined (`Open || Closed`).
    match *self {
      Open(_) => true,
      Closed(_) => true,
      None => false
    }
  }

  fn isUndefined(&self) -> bool {
    //! Returns true if this `EndPoint` is not well-defined (`None`).
    !isDefined(self)
  }

  fn map<U>(&self, op: |T| -> U) -> EndPoint<U> {
    //! Transform this EndPoint with a function over `T`.
    match *self {
      Open(ref t) => Open(op(t)),
      Closed(ref t) => Closed(op(t)),
      None => None
    }
  }


}

/// A `Bound` is a lower or upper bound over a continuous and infinite set of
/// total-ordered values. A bound can be open, closed, or unbounded. An
/// unbounded bound represents a bound either above or below all other bounds,
/// depending on whether the bound is an upper or lower bound.
///
/// `Bound` is an internal implementation mechanism for `Ray`.
///
/// Type parameter `T` is the type of value contained in the continuous,
/// infinite, total-ordered set which the bound operates on.
pub enum Bound<T> {
  Lower(EndPoint<T>),
  Upper(EndPoint<T>)
}

impl<T : TotalOrd> Bound<T> {

}


#[deriving(Eq, TotalEq, Ord, TotalOrd, Show, Rand, Clone)]
pub enum Ray<T> {
  GreaterRay(Bound<T>),   // ----->
  LesserRay(Bound<T>)     // <-----
}

impl<T : TotalOrd> Ray<T> {
  pub fn contains(&self, point: &T) -> bool {
    match *self {
      GreaterRay(Unbounded)     => true,
      GreaterRay(Open(ref b))   => b < point,
      GreaterRay(Closed(ref b)) => b <= point,
      LesserRay(Unbounded)      => true,
      LesserRay(Open(ref b))    => b > point,
      LesserRay(Closed(ref b))  => b >= point
    }
  }
}

pub mod quickcheck {
  extern crate rand;
  extern crate quickcheck;

  use rand::Rand;

  use super::Bound;
  use super::Open;
  use super::Closed;
  use super::Unbounded;
  use super::Ray;
  use super::GreaterRay;
  use super::LesserRay;


  use quickcheck::Arbitrary;
  use quickcheck::Gen;

  impl<T : Arbitrary> Arbitrary for Bound<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Bound<T> {
      match g.next_u32() % 3 {
        0 => Open(Arbitrary::arbitrary(g)),
        1 => Closed(Arbitrary::arbitrary(g)),
        2 => Unbounded,
        _ => fail!("Impossible.")
      }
    }
  }

  impl<T : Arbitrary> Arbitrary for Ray<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Ray<T> {
      let bound: Bound<T> = Arbitrary::arbitrary(g);
      if Rand::rand(g) {
        GreaterRay(bound)
      } else {
        LesserRay(bound)
      }
    }
  }
}
