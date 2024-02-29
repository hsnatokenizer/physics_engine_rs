use crate::environment::objects::Ball;
use environment::world::Space;
pub mod environment;

fn main() {
    let world_space: Space = Space::default();
    let ball: Ball = Ball::new();
    // let objects: [Object; 1] = [ball];
    // world.add_objects(objects);
    //
    // world.run_for(5);

    println!("{:?}", world_space);
    println!("{:?}", ball.coordinates());
}
