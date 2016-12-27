
use std::collections::{Bound, BTreeMap};
use std::ops::AddAssign;

use num::{Num, NumCast, One};
use num;

/// Encode a piecewise-linear function, y(x)
/// X is the type of the domain variable (x),
/// Y is the type of the range variable (y).
#[derive(Clone)]
pub struct PwLine<X: Ord, Y> {
    // a common operation will be to evaluate the PWL at N adjacent points.
    // a tree structure will allow for rapid iteration of adjacent pieces,
    // as well as logarithmic time to locate the first segment.
    points: BTreeMap<X, Y>,
}

pub struct PwLineIter<'a, X: 'a + Ord + Num + Clone + NumCast + AddAssign + One, Y: 'a + Num + Clone + NumCast> {
    pw: &'a PwLine<X, Y>,
    /// The X value of the next point to query.
    x: X,
}

impl<X: Ord, Y> PwLine<X, Y> {
    pub fn new() -> Self {
        PwLine{
            points: BTreeMap::new(),
        }
    }
    pub fn add_pt(&mut self, x: X, y: Y) {
        self.points.insert(x, y);
    }
}
impl<X: Ord + Default, Y> PwLine<X, Y> {
    pub fn from_const(y: Y) -> Self {
        let mut me = PwLine::new();
        me.add_pt(X::default(), y);
        me
    }
}

impl<X, Y> PwLine<X, Y>
    where X: Ord + Num + Clone + NumCast,
          Y: Num + Clone + NumCast + Default {
    /// Evaluates the function at one point. If the query point is OOB,
    /// return the nearest in-bounds point. If there is no such point, return the default
    /// output value.
    pub fn get(&self, at: X) -> Y {
        let mut left_range = self.points.range(Bound::Unbounded, Bound::Included(&at));
        let mut right_range = self.points.range(Bound::Included(&at), Bound::Unbounded);
        let left_point = left_range.next_back();
        let right_point = right_range.next();

        match (left_point, right_point) {
            // The line has NO points
            (None, None) => Y::default(),
            // Queried a point to the right of the function
            (Some((_left_x, left_y)), None) => left_y.clone(),
            // Queried a point to the left of the function
            (None, Some((_right_x, right_y))) => right_y.clone(),
            // Point is on a segment
            (Some((left_x, left_y)), Some((right_x, right_y))) => {
                if left_x == right_x {
                    // This is possible because the ranges are inclusive.
                    left_y.clone()
                } else {
                    let dx : Y = num::cast(right_x.clone()-left_x.clone()).unwrap();
                    let dy = right_y.clone()-left_y.clone();
                    let x_off : Y = num::cast(at.clone()-left_x.clone()).unwrap();
                    left_y.clone() + x_off*dy/dx
                }
            }
        }
    }
}




impl<X, Y> PwLine<X, Y>
    where X: Ord + Num + Clone + NumCast + AddAssign + One,
          Y: Num + Clone + NumCast + Default {
    /// Evaluate the function at `at`, `at+1`, ..., and place results into `into`, unwrapped.
    pub fn get_consec(&self, at: X) -> PwLineIter<X, Y> {
        // TODO: this can be implemented in O(n + log k), where k is the number of segments and n
        // is the number of points to be queried.
        PwLineIter {
            pw: &self,
            x: at,
        }
    }
}

impl<'a, X, Y> Iterator for PwLineIter<'a, X, Y>
    where X: Ord + Num + Clone + NumCast + AddAssign + One,
          Y: Num + Clone + NumCast + Default {
    type Item=Y;
    fn next(&mut self) -> Option<Y> {
        let res = self.pw.get(self.x.clone());
        self.x += One::one();
        Some(res)
    }
}
