extern crate kiss3d;

mod client;

use std::collections::{HashMap, HashSet};

use client::GameClient;
use kiss3d::camera::ArcBall;
use kiss3d::event::{Action, Key};
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use log::{info, LevelFilter};
use naiagame_shared::{ExampleActor, PointActorColor};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

fn translation(x: f32, y: f32, z: f32) -> Translation3<f32> {
    Translation3::new(x, y, z)
}

struct ActorView {
    pub node: SceneNode,
}
impl ActorView {
    pub fn new(window: &mut Window) -> Self {
        let mut node = window.add_sphere(1.0);
        node.set_local_translation(translation(0.0, 0.0, 0.0));
        node.set_color(1.0, 0.5, 0.1);
        Self { node }
    }
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.node
            .set_local_translation(translation(x * 0.01, y * 0.01, z * 0.01));
    }
    pub fn set_color(&mut self, color: &PointActorColor) {
        let (r, g, b) = match color {
            PointActorColor::Red => (1.0, 0.2, 0.2),
            PointActorColor::Blue => (0.2, 1.0, 0.2),
            PointActorColor::Yellow => (1.0, 0.9, 0.2),
        };
        self.node.set_color(r, g, b);
    }
    pub fn remove(mut self, window: &mut Window) {
        window.remove_node(&mut self.node);
    }
}

fn main() {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    // Visuals

    let mut window = Window::new("Kiss3d: wasm example");
    window.set_light(Light::StickToCamera);

    //window.add_cube(1.0, 1.0, 1.0);

    let mut camera = ArcBall::new(Point3::new(0.0, 30.0, -50.0), Point3::origin());

    let mut actor_views = HashMap::new();

    // Networking
    let mut client = GameClient::new();

    // Go

    while window.render_with_camera(&mut camera) {
        client.update(
            window.get_key(Key::W) == Action::Press,
            window.get_key(Key::S) == Action::Press,
            window.get_key(Key::A) == Action::Press,
            window.get_key(Key::D) == Action::Press,
        );
        if client.has_connection() {
            // draw actors
            let actor_keys = client.client.actor_keys().unwrap();
            let mut unused_actors = actor_views.keys().copied().collect::<HashSet<_>>();
            for actor_key in actor_keys {
                if let Some(actor) = client.client.get_actor(&actor_key) {
                    unused_actors.remove(&actor_key);
                    match actor {
                        ExampleActor::PointActor(point_actor) => {
                            let view = actor_views.entry(actor_key).or_insert_with(|| {
                                info!("Creating view for actor {}", actor_key);
                                ActorView::new(&mut window)
                            });
                            let x = f32::from(*(point_actor.as_ref().borrow().x.get()));
                            let y = f32::from(*(point_actor.as_ref().borrow().y.get()));
                            //debug!("Setting actor {} pos to {},{}", actor_key, x, y);
                            view.set_position(x, y, 0.0);
                            view.set_color(point_actor.as_ref().borrow().color.get());
                            /*
                            let color = match point_actor.as_ref().borrow().color.get() {
                                PointActorColor::Red => RED,
                                PointActorColor::Blue => BLUE,
                                PointActorColor::Yellow => YELLOW,
                            };
                            draw_rectangle(
                                f32::from(*(point_actor.as_ref().borrow().x.get())),
                                f32::from(*(point_actor.as_ref().borrow().y.get())),
                                square_size,
                                square_size,
                                color,
                            );*/
                        }
                    }
                }
            }
            for key in unused_actors {
                info!("Actor disappeared: {}", key);
                actor_views.remove(&key).unwrap().remove(&mut window);
            }

            // draw pawns
            for pawn_key in client.client.pawn_keys().unwrap() {
                if let Some(actor) = client.client.get_pawn(&pawn_key) {
                    match actor {
                        ExampleActor::PointActor(_point_actor) => {
                            /*
                            draw_rectangle(
                                f32::from(*(point_actor.as_ref().borrow().x.get())),
                                f32::from(*(point_actor.as_ref().borrow().y.get())),
                                square_size,
                                square_size,
                                WHITE,
                            );*/
                        }
                    }
                }
            }
        }
    }
}
