/*
** src/types/wire.rs
*/

use std::cmp::{self, Ordering};
use std::collections::BTreeSet;

use crate::types::Point;

#[derive(Eq, PartialEq)]
struct WireSegment {
    p1: Point,
    p2: Point,
    path_pfix: u32,  // wire path length preceding this segment
}

impl WireSegment {
    fn new(p1: Point, p2: Point, path_pfix: u32) -> Self {
        Self { p1, p2, path_pfix }
    }

    fn overlap_intersection(&self, other: &Self) -> Point {
        // PRE-CONDITION: self.intersects(other)

        // will be the second closest point to the origin
        let mut endpoint_dists = vec![self.p1, self.p2, other.p1, other.p2]
            .into_iter()
            .map(|p| (p, p.manhattan_distance()))
            .collect::<Vec<(Point, i32)>>();

        endpoint_dists
            .sort_by(|(_, p1_dist), (_, p2_dist)| p1_dist.cmp(p2_dist));

        endpoint_dists[1].0
    }

    fn intersects(&self, other: &Self) -> bool {
        let self_x_lower = cmp::min(self.p1.x, self.p2.x);
        let self_x_upper = cmp::max(self.p1.x, self.p2.x);

        let other_x_lower = cmp::min(other.p1.x, other.p2.x);
        let other_x_upper = cmp::max(other.p1.x, other.p2.x);

        // x-range needs to overlap
        let x_isect = (self_x_lower >= other_x_lower && self_x_lower <= other_x_upper)
                   || (self_x_upper >= other_x_lower && self_x_upper <= other_x_upper)
                   || (other_x_lower >= self_x_lower && other_x_lower <= self_x_upper)
                   || (other_x_upper >= self_x_lower && other_x_upper <= self_x_upper);

        let self_y_lower = cmp::min(self.p1.y, self.p2.y);
        let self_y_upper = cmp::max(self.p1.y, self.p2.y);

        let other_y_lower = cmp::min(other.p1.y, other.p2.y);
        let other_y_upper = cmp::max(other.p1.y, other.p2.y);

        // y-range needs to overlap
        let y_isect = (self_y_lower >= other_y_lower && self_y_lower <= other_y_upper)
                   || (self_y_upper >= other_y_lower && self_y_upper <= other_y_upper)
                   || (other_y_lower >= self_y_lower && other_y_lower <= self_y_upper)
                   || (other_y_upper >= self_y_lower && other_y_upper <= self_y_upper);

        x_isect && y_isect
    }

    fn intersection(&self, other: &Self) -> Option<Point> {
        if self.intersects(other) {
            if self.p1.y == self.p2.y {
                if other.p1.y == other.p2.y {
                    // both horizontal lines
                    Some(self.overlap_intersection(other))
                } else {
                    // self is horizontal, other is vertical
                    Some(Point::new(other.p1.x, self.p1.y))
                }
            } else {
                if other.p1.y == other.p2.y {
                    // self is vertical, other is horizontal
                    Some(Point::new(self.p1.x, other.p1.y))
                } else {
                    // both vertical lines
                    Some(self.overlap_intersection(other))
                }
            }
        } else {
            None
        }
    }
}

impl Ord for WireSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.p1.cmp(&other.p1)
    }
}

impl PartialOrd for WireSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Wire {
    segments: BTreeSet<WireSegment>,
}

impl Wire {
    // based off BTreeSet::range, but that would return only lines in the set with
    // p1.x in the range [line.p1.x, line.p2.x]; we also want lines whose p1.x is
    // outside of the range but p2.x is inside the range
    fn bts_range(&self, segment: &WireSegment) -> Vec<&WireSegment> {
        let mut range = vec![];

        let seg_lower = cmp::min(segment.p1.x, segment.p2.x);
        let seg_upper = cmp::max(segment.p1.x, segment.p2.x);

        for wline in self.segments.iter() {
            // wire line overlaps the target line x-range
            if wline.p1.x >= seg_lower && wline.p1.x <= seg_upper {
                range.push(wline);
            } else if wline.p2.x >= seg_lower && wline.p2.x <= seg_upper {
                range.push(wline);
            }
        }

        range
    }

    /// Find all intersections with another wire
    pub fn intersections(&self, other: &Self) -> Vec<Point> {
        let mut isects = vec![];

        for seg_other in other.segments.iter() {
            for seg in self.bts_range(seg_other) {
                if let Some(pt) = seg_other.intersection(seg) {
                    if !pt.is_origin() {
                        isects.push(pt);
                    }
                }
            }
        }

        isects
    }

    /// Find all intersections with another wire, but return the steps taken to
    /// reach the intersection points on each wire
    pub fn intersections_path_lengths(&self, other: &Self) -> Vec<(u32, u32)> {
        let mut isects = vec![];

        for seg_other in other.segments.iter() {
            for seg in self.bts_range(seg_other) {
                if let Some(pt) = seg_other.intersection(seg) {
                    if !pt.is_origin() {
                        let w1_dist = seg_other.path_pfix + seg_other.p1.distance_to(&pt) as u32;
                        let w2_dist = seg.path_pfix + seg.p1.distance_to(&pt) as u32;
                        isects.push((w1_dist, w2_dist));
                    }
                }
            }
        }

        isects
    }
}

impl From<String> for Wire {
    fn from(s: String) -> Self {
        let mut segments = BTreeSet::new();

        let (mut x, mut y) = (0, 0);
        let mut path_len = 0;

        for wstr in s.split(",") {
            let p1 = Point::new(x, y);

            let len = &wstr[1..(wstr.len())].parse().unwrap();
            match &wstr[0..1] {
                "U" => y += len,
                "D" => y -= len,
                "L" => x -= len,
                "R" => x += len,
                _   => panic!("invalid direction: {}", &wstr[0..1]),
            }

            let p2 = Point::new(x, y);
            segments.insert(WireSegment::new(p1, p2, path_len));
            path_len += *len as u32;
        }

        Self { segments }
    }
}
