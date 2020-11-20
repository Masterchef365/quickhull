use anyhow::Result;
use klystron::{
    runtime_2d::{event::WindowEvent, launch, App2D},
    DrawType, Engine, FramePacket, Material, Mesh, Object, Vertex, WinitBackend, UNLIT_FRAG,
    UNLIT_VERT,
};
use nalgebra::{Matrix4, Point2, Vector4};
use rand::distributions::{Distribution, Uniform};
use std::cmp::Ordering;

type Point = Point2<f32>;
type Line = (Point, Point);


fn quickhull_init(points: &[Point]) -> Option<Line> {
    fn pt_compare (a: &Point, b: &Point) -> Ordering {
        f32_cmp(a.x, b.x).then(f32_cmp(a.y, b.y))
    }
    let min = *points.iter().min_by(|&a, &b| pt_compare(a, b))?;
    let max = *points.iter().max_by(|&a, &b| pt_compare(a, b))?;
    Some((min, max))
}

fn quickhull(points: &[Point], line: &Line) -> Vec<Line> {
    if points.is_empty() { return vec![] }
    vec![]
}

fn f32_cmp(a: f32, b: f32) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

// https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
fn line_dist((a, b): &Line, pt: Point) -> f32 {
    let length = (a - b).magnitude();
    let cross = b.x * a.y - b.y * a.x;
    let numerator = ((b.y - a.y) * pt.x - (b.x - a.x) * pt.y + cross).abs();
    numerator / length
}

fn line_right((a, b): Line, point: Point) -> bool {
    let l = b - a;
    let p = point - a;
    l.x*p.y - l.y*p.x < 0.
}

fn triangle_prot(a: Point, b: Point, c: Point, pt: Point) -> bool {
    line_right((c, a), pt) &&
    line_right((a, b), pt) &&
    line_right((b, c), pt)
}

fn triangle_member(a: Point, b: Point, c: Point, pt: Point) -> bool {
    triangle_prot(a, b, c, pt) || triangle_prot(c, b, a, pt)
}

#[test]
fn test_line_right() {
    let line = (Point::new(1., -1.), Point::new(-1., 1.));
    assert!(line_right(line, Point::new(20., 10.)));
    assert!(!line_right(line, Point::new(-20., -10.)));
    assert!(line_right(line, Point::new(20., -10.)));
    assert!(!line_right(line, Point::new(-20., 10.)));
}

#[test]
fn test_triangle_member() {
    let a = Point::new(0., 1.);
    let b = Point::new(1., -1.);
    let c = Point::new(-1., -1.);
    let test_all_order = |pt| {
        triangle_member(a, b, c, pt) &&
        triangle_member(b, c, a, pt) &&
        triangle_member(c, a, b, pt) &&

        triangle_member(a, c, b, pt) &&
        triangle_member(b, a, c, pt) &&
        triangle_member(c, b, a, pt)
    };
    assert!(test_all_order(Point2::origin()));
    assert!(test_all_order(Point2::new(0.1, 0.1)));
    assert!(!test_all_order(Point2::new(5., 5.)));
    assert!(!test_all_order(Point2::new(-5., 5.)));
    assert!(!test_all_order(Point2::new(-5., -5.)));
}

// ######################################################################

struct MyApp {
    point_material: Material,
    point_mesh: Mesh,
    line_material: Material,
    line_mesh: Mesh,
}

fn point2d_to_vertex(point: Point, color: [f32; 3]) -> Vertex {
    Vertex {
        pos: [point.x, point.y, 0.],
        color,
    }
}

fn lines_to_mesh(lines: &[Line], color: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::with_capacity(lines.len() * 2);
    let mut indices = Vec::with_capacity(lines.len() * 2);
    for (a, b) in lines {
        vertices.push(point2d_to_vertex(*a, color));
        indices.push(indices.len() as _);
        vertices.push(point2d_to_vertex(*b, color));
        indices.push(indices.len() as _);
    }
    (vertices, indices)
}

impl App2D for MyApp {
    const TITLE: &'static str = "2D example app";
    type Args = ();

    fn new(engine: &mut WinitBackend, _args: Self::Args) -> Result<Self> {
        let line_material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Lines)?;
        let point_material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Points)?;

        let mut rng = rand::thread_rng();
        let count = 400;
        let dist_unif = Uniform::new(0., 1.);
        let angle_unif = Uniform::new(0., std::f32::consts::TAU);
        let mut points = Vec::new();
        for _ in 0..count {
            let dist = dist_unif.sample(&mut rng);
            let angle = angle_unif.sample(&mut rng);
            let x = angle.cos() * dist;
            let y = angle.sin() * dist;
            points.push(Point::new(x, y));
        }

        let line = (Point::new(1., -1.), Point::new(-1., 1.));
        let vertices = points
            .iter()
            //.map(|p| point2d_to_vertex(*p, [1.; 3]))
            .map(|p| point2d_to_vertex(*p, [line_dist(&line, *p); 3]))
            .collect::<Vec<_>>();
        let indices = (0..vertices.len() as u16).collect::<Vec<_>>();
        let point_mesh = engine.add_mesh(&vertices, &indices)?;

        let init = quickhull_init(&points).expect("Empty set");
        let mut hull = quickhull(&points, &init);
        //hull.push(init);
        hull.push(line);
        let (vertices, indices) = lines_to_mesh(&hull, [0., 1., 0.]);
        let line_mesh = engine.add_mesh(&vertices, &indices)?;

        Ok(Self {
            point_material,
            point_mesh,
            line_mesh,
            line_material,
        })
    }

    fn event(&mut self, _event: &WindowEvent, _engine: &mut WinitBackend) -> Result<()> {
        Ok(())
    }

    fn frame(&self) -> FramePacket {
        let flip = Matrix4::from_diagonal(&Vector4::new(1., -1., 1., 1.));
        let zoom = 0.2;
        let zoom = Matrix4::from_diagonal(&Vector4::new(zoom, zoom, 1., 1.));
        let transform = zoom * flip;

        let points = Object {
            material: self.point_material,
            mesh: self.point_mesh,
            transform
        };

        let lines = Object {
            material: self.line_material,
            mesh: self.line_mesh,
            transform
        };

        FramePacket {
            objects: vec![points, lines],
        }
    }
}

fn main() -> Result<()> {
    launch::<MyApp>(())
}
