use anyhow::Result;
use klystron::{
    runtime_2d::{event::WindowEvent, launch, App2D},
    DrawType, Engine, FramePacket, Material, Object, Vertex, WinitBackend, UNLIT_FRAG, Mesh,
    UNLIT_VERT,
};
use rand::distributions::{Uniform, Distribution};
use nalgebra::Matrix4;

struct MyApp {
    point_material: Material,
    point_mesh: Mesh,
    //line_material: Material,
    //line_mesh: Mesh,
}

impl App2D for MyApp {
    const TITLE: &'static str = "2D example app";
    type Args = ();

    fn new(engine: &mut WinitBackend, _args: Self::Args) -> Result<Self> {
        //let line_material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Lines)?;
        let point_material = engine.add_material(UNLIT_VERT, UNLIT_FRAG, DrawType::Points)?;

        let mut rng = rand::thread_rng();
        let count = 400;
        let mut vertices = Vec::with_capacity(count);
        let mut indices = Vec::with_capacity(count);
        let dist_unif = Uniform::new(0., 0.2);
        let angle_unif = Uniform::new(0., std::f32::consts::TAU);
        for _ in 0..count {
            let dist = dist_unif.sample(&mut rng);
            let angle = angle_unif.sample(&mut rng);
            let x = angle.cos() * dist;
            let y = angle.sin() * dist;
            vertices.push(Vertex {
                pos: [x, y, 0.],
                color: [1.; 3],
            });
            indices.push(indices.len() as _);
        }
        let point_mesh = engine.add_mesh(&vertices, &indices)?;

        Ok(Self {
            point_material,
            point_mesh,
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

        FramePacket {
            objects: vec![points],
        }
    }
}

fn main() -> Result<()> {
    launch::<MyApp>(())
}
