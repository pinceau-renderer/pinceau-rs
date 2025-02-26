// use pinceau_rs::run;

use math::vec3::Vec3;
use math::mat4::Mat4;
use pinceau_rs::run;


mod math {
    pub mod utils;
    pub mod vec3;
    pub mod mat4;
}


fn main() {
    pollster::block_on(run());
}


