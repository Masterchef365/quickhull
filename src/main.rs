use anyhow::Result;
use klystron::{
    runtime_2d::{event::WindowEvent, launch, App2D},
    DrawType, Engine, FramePacket, Material, Mesh, Object, Vertex, WinitBackend, UNLIT_FRAG,
    UNLIT_VERT,
};
use nalgebra::{Matrix4, Point2};
use rand::distributions::{Distribution, Uniform};

struct MyApp {
    point_material: Material,
    point_mesh: Mesh,
    line_material: Material,
    line_mesh: Mesh,
}

fn point2d_to_vertex(point: Point2<f32>, color: [f32; 3]) -> Vertex {
    Vertex {
        pos: [point.x, point.y, 0.],
        color,
    }
}

type Line = (Point2<f32>, Point2<f32>);
fn quickhull(points: &[Point2<f32>]) -> Vec<Line> {
    vec![(points[0], points[1])]
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
        let dist_unif = Uniform::new(0., 0.2);
        let angle_unif = Uniform::new(0., std::f32::consts::TAU);
        let mut points = Vec::new();
        for _ in 0..count {
            let dist = dist_unif.sample(&mut rng);
            let angle = angle_unif.sample(&mut rng);
            let x = angle.cos() * dist;
            let y = angle.sin() * dist;
            points.push(Point2::new(x, y));
        }

        let vertices = points
            .iter()
            .map(|p| point2d_to_vertex(*p, [1.; 3]))
            .collect::<Vec<_>>();
        let indices = (0..vertices.len() as u16).collect::<Vec<_>>();
        let point_mesh = engine.add_mesh(&vertices, &indices)?;

        let hull = quickhull(&points);
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
        let points = Object {
            material: self.point_material,
            mesh: self.point_mesh,
            transform: Matrix4::identity(),
        };

        let lines = Object {
            material: self.line_material,
            mesh: self.line_mesh,
            transform: Matrix4::identity(),
        };

        FramePacket {
            objects: vec![points, lines],
        }
    }
}

fn main() -> Result<()> {
    launch::<MyApp>(())
}
