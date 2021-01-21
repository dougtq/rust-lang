extern crate rayon;
extern crate itertools;

use rayon::prelude::*;
use itertools::Itertools;

mod bodies;
use bodies::get_values;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    x: f64,
    y: f64,
    z: f64,
    mass: f64
}

fn average (a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn average_with_mass (a: f64, b: f64, a_mass: f64, b_mass: f64) -> f64 {
    average(a * a_mass, b * b_mass) / (a_mass + b_mass)
}

fn merge_bodies (a: Body, b: Body) -> Body {
    Body {
        x: average_with_mass(a.x, b.x, a.mass, b.mass),
        y: average_with_mass(a.y, b.y, a.mass, b.mass),
        z: average_with_mass(a.z, b.z, a.mass, b.mass),
        mass: a.mass + b.mass
    }
}

fn merge_all_bodies_iter (bodies: &[Body]) -> Body {
    let barycenter : Body = bodies[0];

    bodies.iter().skip(1).fold(barycenter, |barycenter, body|{
        merge_bodies(barycenter, *body)
    })


}

fn merge_all_bodies_recursive (bodies: &[Body]) -> Body {
    println!("{}", bodies.len());

    if bodies.len() == 1 {
        return bodies[0]
    }

    let tuples : Vec<_> = bodies.iter().tuples().collect();
    let mut merged_bodies: Vec<_> = tuples.into_par_iter().map(|(a, b)| {
        merge_bodies(*a, *b)
    }).collect();

    if bodies.len() % 2 != 0 {
        merged_bodies.push(bodies[bodies.len() - 1])
    }

    return merge_all_bodies_recursive(&merged_bodies)
}

fn main() {
    let bodies = get_values();

    let barycenter = merge_all_bodies_recursive(&bodies);

    println!("Barycenter is {:?}", barycenter);
}
