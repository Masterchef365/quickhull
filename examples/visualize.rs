use anyhow::Result;
use klystron::{
    runtime_2d::{event::WindowEvent, launch, App2D},
    DrawType, Engine, FramePacket, Material, Mesh, Object, Vertex, WinitBackend, UNLIT_FRAG,
    UNLIT_VERT,
};
use nalgebra::{Matrix4, Vector4};
use quickhull::{quickhull, Line, Point};
use rand::distributions::{Distribution, Uniform};

struct MyApp {
    point_material: Material,
    point_mesh: Mesh,
    line_material: Material,
    line_mesh: Mesh,
}

impl App2D for MyApp {
    const TITLE: &'static str = "Quickhull";
    type Args = ();

    fn new(engine: &mut WinitBackend, _args: Self::Args) -> Result<Self> {
        let line_material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Lines)?;
        let point_material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Points)?;

        let mut rng = rand::thread_rng();
        let count = 100;
        let dist_unif = Uniform::new(0., 1.);
        let angle_unif = Uniform::new(0., std::f32::consts::TAU);
        let mut points = Vec::new();
        for _ in 0..count {
            let dist = dist_unif.sample(&mut rng);
            let angle = angle_unif.sample(&mut rng);
            let x = angle.cos() * dist * (angle * 3.).sin();
            let y = angle.sin() * dist;
            points.push(Point::new(x, y));
        }

        let vertices = points[..points.len().min(40_000)]
            .iter()
            .map(|p| point2d_to_vertex(*p, [1.; 3]))
            .collect::<Vec<_>>();
        let indices = (0..vertices.len() as u16).collect::<Vec<_>>();
        let point_mesh = engine.add_mesh(&vertices, &indices)?;

        use std::time::Instant;
        let begin = Instant::now();
        let hull = quickhull(&points);
        let end = Instant::now();
        println!("Time: {:?}", end - begin);

        let (vertices, indices) = lines_to_mesh(&hull, [0., 1., 0.]);
        let line_mesh = engine.add_mesh(&vertices, &indices)?;

        Ok(Self {
            point_material,
            point_mesh,
            line_mesh,
            line_material,
        })
    }

    fn frame(&self) -> FramePacket {
        let flip = Matrix4::from_diagonal(&Vector4::new(1., -1., 1., 1.));
        let zoom = 0.2;
        let zoom = Matrix4::from_diagonal(&Vector4::new(zoom, zoom, 1., 1.));
        let transform = zoom * flip;

        let points = Object {
            material: self.point_material,
            mesh: self.point_mesh,
            transform,
        };

        let lines = Object {
            material: self.line_material,
            mesh: self.line_mesh,
            transform,
        };

        FramePacket {
            objects: vec![points, lines],
        }
    }

    fn event(&mut self, _event: &WindowEvent, _engine: &mut WinitBackend) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    launch::<MyApp>(())
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
