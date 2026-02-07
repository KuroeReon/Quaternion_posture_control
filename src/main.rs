mod struct_quaternion;
use struct_quaternion::Quaternion;
use std::f32::consts::PI;

fn main() {
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
}
