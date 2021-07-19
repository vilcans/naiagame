extern crate kiss3d;

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3, UnitQuaternion, Vector3};
use kiss3d::window::Window;

fn translation(x: f32, y: f32, z: f32) -> Translation3<f32> {
    Translation3::new(x, y, z)
}

fn main() {
    let mut window = Window::new("Kiss3d: wasm example");
    window.set_light(Light::StickToCamera);

    let mut cube = window.add_cube(1.0, 1.0, 1.0);
    cube.set_color(1.0, 0.0, 0.0);

    let mut sphere = window.add_sphere(0.2);
    sphere.append_translation(&translation(0.0, 0.0, 1.0));

    let mut camera = ArcBall::new(Point3::new(0.0, 3.0, -10.0), Point3::origin());

    while window.render_with_camera(&mut camera) {
        let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
        cube.prepend_to_local_rotation(&rot);
    }
}
