use std::cmp::Ordering;

pub type Point = nalgebra::Point2<f32>;
pub type Line = (Point, Point);

/// Calculate the convex hull from a set of points
pub fn quickhull(points: &[Point]) -> Vec<Line> {
    let mut lines = Vec::new();

    let init_line = match quickhull_init(points) {
        None => return lines,
        Some(l) => l,
    };

    let (init_a, init_b) = init_line;
    quickhull_recursive(points, (init_a, init_b), &mut lines);
    quickhull_recursive(points, (init_b, init_a), &mut lines);

    lines
}

/// Find initial line spanning the largest distance over the x axis
fn quickhull_init(points: &[Point]) -> Option<Line> {
    if points.is_empty() { return None }
    let mut min = std::f32::MAX;
    let mut min_point = Point::origin();
    let mut max = std::f32::MIN;
    let mut max_point = Point::origin();
    for point in points {
        if point.x >= max {
            if point.x == max && point.y < max_point.y {
                continue;
            }
            max = point.x;
            max_point = *point;
        } else if point.x < min {
            if point.x == min && point.y > min_point.y {
                continue;
            }
            min = point.x;
            min_point = *point;
        }
    }
    Some((min_point, max_point))
}

/// Recursively find the convex hull of this half of the given point set
fn quickhull_recursive(points: &[Point], line: Line, out_lines: &mut Vec<Line>) {
    let right = points
        .iter()
        .copied()
        .filter(|&pt| line_right(line, pt))
        .collect::<Vec<_>>();

    let furthest = right
        .iter()
        .max_by(|&a, &b| f32_cmp(line_dist(line, *a), line_dist(line, *b)));

    match furthest {
        None => out_lines.push(line),
        Some(furthest) => {
            quickhull_recursive(&right, (*furthest, line.1), out_lines);
            quickhull_recursive(&right, (line.0, *furthest), out_lines);
        }
    }
}

/// Compare two floats in a (semi) sane way
fn f32_cmp(a: f32, b: f32) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

/// Find the closest distance between a line and a given point
fn line_dist((a, b): Line, pt: Point) -> f32 {
    // https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
    let length = (a - b).magnitude();
    let cross = b.x * a.y - b.y * a.x;
    let numerator = ((b.y - a.y) * pt.x - (b.x - a.x) * pt.y + cross).abs();
    numerator / length
}

/// Returns `true` when the given point is to the right of a given line
fn line_right((a, b): Line, point: Point) -> bool {
    let l = b - a;
    let p = point - a;
    l.x * p.y - l.y * p.x < 0.
}

#[cfg(test)]
#[test]
fn test_line_right() {
    let line = (Point::new(1., -1.), Point::new(-1., 1.));
    assert!(line_right(line, Point::new(20., 10.)));
    assert!(!line_right(line, Point::new(-20., -10.)));
    assert!(line_right(line, Point::new(20., -10.)));
    assert!(!line_right(line, Point::new(-20., 10.)));
}
