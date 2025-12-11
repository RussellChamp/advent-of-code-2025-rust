advent_of_code::solution!(9);
use itertools::Itertools;

fn parse_point(s: &str) -> Option<(u64, u64)> {
    s.trim()
    .split(',')
    .map(|v| v.parse::<u64>().ok())
    .collect_tuple()
    .and_then(|(a, b)| Some((a?, b?)))
}

// fn does_vertex_intersect(a1: (u64, u64), a2: (u64, u64), b1: (u64, u64), b2: (u64, u64)) -> bool {

// }

pub fn part_one(input: &str) -> Option<u64> {
    let points = input.lines().filter(|l| !l.is_empty()).filter_map(parse_point).collect_vec();
    let size = points.len();
    // println!("{points:?}");
    let mut best_size = 0;

    for a_idx in 0..size {
        let a = points[a_idx];
        for b_idx in a_idx+1..size {
            let b = points[b_idx];
            let size = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            // println!("size of {a:?} and {b:?} is {size}");
            if size > best_size { best_size = size }
        }
    }

    Some(best_size)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = input.lines().filter(|l| !l.is_empty()).filter_map(parse_point).collect_vec();
    let size = points.len();
    // println!("{points:?}");
    let mut best_size = 0;

    for a_idx in 0..size {
        let a = points[a_idx];
        for b_idx in a_idx+1..size {
            let b = points[b_idx];
            let is_valid = false;

            // determine whether the polygon represented by (a.0, a.1), (a.0, b.1), (b.0, b.1), (b.0, a.1)
            // fits completely within the polygon represented by `points`
            // assumptions: we assume that the two polygons either overlap or intersect (and are not completely separate) because they are composed of the same points. therefore we do not need to check that case
            // * compare each vertex in the outer polygon with each vertex in the inner polygon
            //  * if they intersect, the polygons intersect
            //   * if they only intersect on the endpoint...
            //  * if they overlap
            //   * if the inner vertex is contained within the outer vertex it is ok
            //   * otherwise maybe it isn't ok?

            // generate slope from points
            // given point p1 = (x1, y1) and p2 = (x2, y2) the equation representing the slope of the form y = a*x + b would have values
            // a1 = (y1 - y2) / (x1 - x2)
            // b1 = y1 - x1 * (y1 - y2) / (x1 - x2)
            // for point p3 and p4 (representing the second vertex) we use the same equation
            // a2 = (y3 - y4) / (x3 - x4)
            // b2 = y3 - x3 * (y3 - y4) / (x3 - x4)
            // to find whether the vertices intersect, we attempt to find a point (x,y) point that exists on both slopes
            // that is, where y1 = a1*x1 == y2 = a2*x2
            // simplifying we find that point would be
            // x_intersect = (b2 - b1) / (a1 - a2)
            // y_intersect = b1 + a1 * (b2 - b1) / (a1 - a2)
            // although we need to check whether x exists within lower and upper bounds of the range defined by x1,x2,x3,x4



            if !is_valid { continue }

            let size = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            // println!("size of {a:?} and {b:?} is {size}");
            if size > best_size { best_size = size }
        }
    }

    Some(best_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
