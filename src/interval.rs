extern crate core;

use core::cmp::TotalOrd;

#[deriving(Eq, Show)]
pub enum Bound<T> {
  Open(T),
  Closed(T),
  Unbounded
}

#[deriving(Eq, Show)]
pub struct Interval<T>(Bound<T>, Bound<T>);

impl<T : TotalOrd> Ord for Interval<T> {

  fn lt(&self, other: &Interval<T>) -> bool {
    let &Interval(ref a, _) = self;
    let &Interval(ref b, _) = other;
    match (a, b) {
      (&Unbounded, &Unbounded) => false,
      (&Unbounded, _) => true,
      (_, &Unbounded) => false,
      (&Open(ref l),   &Open(ref r))   => l < r,
      (&Open(ref l),   &Closed(ref r)) => l < r,
      (&Closed(ref l), &Open(ref r))   => l <= r,
      (&Closed(ref l), &Closed(ref r)) => l < r
    }
  }

  //fn gt(&self, other: &Interval<T>) -> bool {
    //let &Interval(ref _, a) = self;
    //let &Interval(ref _, b) = other;
    //match (a, b) {
      //(&Unbounded, _) => true,
      //(_, &Unbounded) => false,
      //(&Open(ref l),   &Open(ref r))   => l < r,
      //(&Open(ref l),   &Closed(ref r)) => l < r,
      //(&Closed(ref l), &Open(ref r))   => l <= r,
      //(&Closed(ref l), &Closed(ref r)) => l < r
    //}
  //}

}

#[cfg(test)]
mod tests {

  use super::Unbounded;
  use super::Open;
  use super::Closed;
  use super::Interval;

    // unbounded           <----------------------->
    //
    // open_unb_a              (------------------->
    // open_unb_b                  (--------------->
    //
    // closed_unb_a            [------------------->
    // closed_unb_b                [--------------->
    //
    // unb_open_a          <---------------)
    // unb_open_b          <-------------------)
    //
    // unb_closed_a        <---------------]
    // unb_closed_b        <-------------------]
    //
    // open_open_a             (---------------)
    // open_open_b                 (-------)
    // open_open_c             (-------)
    // open_open_d                     (-------)
    //
    // closed_closed_a         [---------------]
    // closed_closed_b             [-------]
    // closed_closed_c         [-------]
    // closed_closed_d                 [-------]
    //
    // open_closed_a           (---------------]
    // open_closed_b               (-------]
    // open_closed_c           (-------]
    // open_closed_d                   (-------]
    //
    // closed_open_a           [---------------)
    // closed_open_b               [-------)
    // closed_open_c           [-------)
    // closed_open_d                   [-------)
    //
    //                         0   1   2   3   4

    static unbounded: Interval<int> = Interval(Unbounded, Unbounded);

    static open_unb_a: Interval<int> = Interval(Open(0), Unbounded);
    static open_unb_b: Interval<int> = Interval(Open(1), Unbounded);

    static closed_unb_a: Interval<int> = Interval(Closed(0), Unbounded);
    static closed_unb_b: Interval<int> = Interval(Closed(1), Unbounded);

    static unb_open_a: Interval<int> = Interval(Unbounded, Open(3));
    static unb_open_b: Interval<int> = Interval(Unbounded, Open(4));

    static unb_closed_a: Interval<int> = Interval(Unbounded, Closed(3));
    static unb_closed_b: Interval<int> = Interval(Unbounded, Closed(4));

    static open_open_a: Interval<int> = Interval(Open(0), Open(4));
    static open_open_b: Interval<int> = Interval(Open(1), Open(3));
    static open_open_c: Interval<int> = Interval(Open(0), Open(2));
    static open_open_d: Interval<int> = Interval(Open(2), Open(4));

    static closed_closed_a: Interval<int> = Interval(Closed(0), Closed(4));
    static closed_closed_b: Interval<int> = Interval(Closed(1), Closed(3));
    static closed_closed_c: Interval<int> = Interval(Closed(0), Closed(2));
    static closed_closed_d: Interval<int> = Interval(Closed(2), Closed(4));

    static open_closed_a: Interval<int> = Interval(Open(0), Closed(4));
    static open_closed_b: Interval<int> = Interval(Open(1), Closed(3));
    static open_closed_c: Interval<int> = Interval(Open(0), Closed(2));
    static open_closed_d: Interval<int> = Interval(Open(2), Closed(4));

    static closed_open_a: Interval<int> = Interval(Closed(0), Open(4));
    static closed_open_b: Interval<int> = Interval(Closed(1), Open(3));
    static closed_open_c: Interval<int> = Interval(Closed(0), Open(2));
    static closed_open_d: Interval<int> = Interval(Closed(2), Open(4));

    static all_intervals: [Interval<int>, ..25] =
      [
        unbounded,
        open_unb_a, open_unb_b,
        closed_unb_a, closed_unb_b,
        unb_open_a, unb_open_b,
        unb_closed_a, unb_closed_b,
        open_open_a, open_open_b, open_open_c, open_open_d,
        closed_closed_a, closed_closed_b, closed_closed_c, closed_closed_d,
        closed_open_a, closed_open_b, closed_open_c, closed_open_d,
        open_closed_a, open_closed_b, open_closed_c, open_closed_d
      ];

    // The intervals whose left bound falls on 0
    static left_0_intervals: [Interval<int>, .. 10] =
      [
        open_unb_a,
        closed_unb_a,
        open_open_a, open_open_c,
        closed_closed_a, closed_closed_c,
        open_closed_a, open_closed_c,
        closed_open_a, closed_open_c
      ];

    // The intervals whose left bound falls on 1
    static left_1_intervals: [Interval<int>, .. 6] =
      [
        open_unb_b,
        closed_unb_b,
        open_open_b,
        closed_closed_b,
        open_closed_b,
        closed_open_b
      ];

    // The intervals whose left bound falls on 2
    static left_2_intervals: [Interval<int>, .. 4] =
      [
        open_open_d,
        closed_closed_d,
        open_closed_d,
        closed_open_d
      ];

  #[test]
  fn identity_lt() {
    for interval in all_intervals.iter() {
      assert!(!(interval < interval));
    }
  }

  #[test]
  fn identity_lte() {
    for interval in all_intervals.iter() {
      assert!(interval <= interval);
    }
  }

  #[test]
  fn unbounded_lt() {
    assert!(unbounded < open_unb_a);
    assert!(unbounded < closed_unb_a);
  }

  #[test]
  fn unbounded_lte() {
    for interval in all_intervals.iter() {
      assert!(unbounded <= *interval);
    }
  }

  #[test]
  fn open_lt() {
    // The intervals with a left open 0 bound
    let left_open_0_intervals: Vec<Interval<int>> =
      vec1(open_unb_a, open_open_a, open_open_c, open_closed_a, open_closed_c);

    for open_interval in left_open_0_intervals.iter() {
      for interval in left_1_intervals.iter() {
        assert(open_interval < interval);
      }
      for interval in left_2_intervals.iter() {
        assert(open_interval < interval);
      }
      for interval in left_3_intervals.iter() {
        assert(open_interval < interval);
      }
    }
  }

  #[test]
  fn open_lt() {
    assert!(open_unb_a < open_unb_b);
    assert!(open_unb_a < closed_unb_b);
    assert!(open_unb_a < open_open_b);
    assert!(open_unb_a < closed_closed_b);
    assert!(open_unb_a < open_closed_b);
    assert!(open_unb_a < closed_open_b);
  }

  #[test]
  fn open_lte() {
    assert!(open_unb_a <= open_unb_b);
    assert!(open_unb_a <= closed_unb_b);
  }

  #[test]
  fn closed_lt() {
    assert!(closed_unb_a < closed_unb_b);
    assert!(closed_unb_a < open_unb_a);
    assert!(closed_unb_a < open_unb_b);
  }

}
