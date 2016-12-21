extern crate pbrt;

use pbrt::{Ray, Point3f, Vector3f};

fn main() {
    let origin = Point3f {
        x: -5.5,
        y: 2.75,
        z: 0.0,
    };
    let direction = Vector3f {
        x: 1.0,
        y: -8.75,
        z: 2.25,
    };
    let ray = Ray {
        o: origin,
        d: direction,
    };

    println!("{:?}", ray);
}
