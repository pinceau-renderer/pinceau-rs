// use pinceau_rs::run;

use math::vec3::Vec3;
use math::mat4::Mat4;

mod math {
    pub mod utils;
    pub mod vec3;
    pub mod mat4;
}

fn main() {
    let vec = Vec3::<f32>::new(1., 0., 0.);
    println!("{}", vec);
    let mat = Mat4::<f32>::identity();
    println!("{}", mat);
    let projection = Mat4::perspective(45., 1.5, 0.1, 1000.);
    println!("{}", projection);
}
