#![feature(btree_range)]
#![feature(collections_bound)]
extern crate num;

use std::collections::{Bound, BTreeMap};

use num::{Num, NumCast};

/// Encode a piecewise-linear function, y(x)
/// X is the type of the domain variable (x),
/// Y is the type of the range variable (y).
pub struct PwLine<X: Ord, Y> {
    // a common operation will be to evaluate the PWL at N adjacent points.
    // a tree structure will allow for rapid iteration of adjacent pieces,
    // as well as logarithmic time to locate the first segment.
    points: BTreeMap<X, Y>,
}

impl<X: Ord + Num + Clone + NumCast, Y: Num + Clone + From<X> + NumCast> PwLine<X, Y> {
    pub fn new() -> Self {
        PwLine{
            points: BTreeMap::new(),
        }
    }
    /// evaluate the function at one point, returning None if the query point
    /// is outside the domain (defined inclusively) of the function.
    pub fn get(&self, at: X) -> Option<Y> {
        // locate the point on either side of `at`.
        // TODO: can we do this with just one call to range?
        let left_range = self.points.range(Bound::Unbounded, Bound::Included(&at));
        let mut right_range = self.points.range(Bound::Included(&at), Bound::Unbounded);
        match left_range.rev().next() {
            None => None,
            Some((left_x, left_y)) => {
                match right_range.next() {
                    None => None,
                    Some((right_x, right_y)) => {
                        // We've defined a line from (left_x, left_y) to (right_x, right_y): now
                        // evaluate it at `x=at`. Note: need to consider the case where left_x ==
                        // right_x.
                        if &at == left_x {
                            // serves to avoid the division-by-zero case
                            Some(left_y.clone())
                        } else {
                            Some(left_y.clone() + (right_y.clone()-left_y.clone())/num::cast(right_x.clone()-left_x.clone()).unwrap())
                        }
                    }
                }
            }
        }
    }
}
