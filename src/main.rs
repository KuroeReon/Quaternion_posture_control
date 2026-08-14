mod struct_quaternion;
mod defferential_equation;
use struct_quaternion::Quaternion;
use defferential_equation::rungekutta;
use std::f32::consts::PI;

fn main() {
    let dt = 0.01;
    let mut qs: Vec<Quaternion> = Vec::new();
    let mut angulars: Vec<Quaternion> = Vec::new();
    angulars.resize_with(101, || Quaternion::new(0.0,  0.0, 0.1 * PI, 0.0));

    qs.push(Quaternion::new(1.0, 0.0, 0.0, 0.0));

    for n in 0..(angulars.len() - 1) {
        rungekutta(&angulars, &mut qs, n, dt);
    }
    for (i, q) in qs.iter().enumerate() {
        println!("new Quaternion({}f, {}f, {}f, {}f),", q.w, q.x, q.y, q.z);
    }
    /*
    let q1_theta: f32 = PI / 4.0;
    let q1 = &mut Quaternion::new(q1_theta.cos(), 0.0, q1_theta.sin(), 0.0);
    let q2 = &mut Quaternion::new(q1_theta.cos(), 0.0, 0.0, q1_theta.sin());
    q1.scale(2.0);  
    q1.print();  
    q1.normalize();
    q1.print();
    let q3 = q2.mul(q1);
    q3.print();
    let q4 = q1.sum(q2);
    q4.print();
    */
}
