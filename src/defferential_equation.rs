use crate::struct_quaternion::Quaternion;

fn derivative(angular_velocity: &Quaternion, posture_quaternion: &Quaternion) -> Quaternion {
    let derivative = angular_velocity.mul(posture_quaternion).scale(0.5);
    return derivative;
}

pub fn rungekutta(angulars: &Vec<Quaternion>, qs: &mut Vec<Quaternion>, n: usize, dt: f32) {
    let current_angular = &angulars[n];
    let next_angular = &angulars[n + 1];
    let average_angular = (current_angular.sum(next_angular)).scale(0.5);
    let k1: &Quaternion = &derivative(&current_angular, &qs[n]);
    let quaternion_for_k2: &Quaternion = &qs[n].sum(&k1.scale(dt / 2.0));
    let k2: &Quaternion = &derivative(&average_angular, &quaternion_for_k2);
    let quaternion_for_k3: &Quaternion = &qs[n].sum(&k2.scale(dt / 2.0));
    let k3: &Quaternion = &derivative(&average_angular, &quaternion_for_k3);
    let quaternion_for_k4: &Quaternion = &qs[n].sum(&k3.scale(dt));
    let k4: &Quaternion = &derivative(&next_angular, &quaternion_for_k4);
    let weighted_avetrage_k: &Quaternion = &k1.sum(&k2.scale(2.0)).sum(&k3.scale(2.0)).sum(&k4).scale(1.0/6.0);
    let next_quaternion: &Quaternion = &qs[n].sum(&weighted_avetrage_k.scale(dt));
    let next_quaternion_normalized = next_quaternion.normalize();
    qs.push(next_quaternion_normalized);
}

