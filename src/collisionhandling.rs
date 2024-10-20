use std::f64::consts::PI;

use crate::particle::{calc_pp_accel, Particle};
use crate::vectors::Vector;

struct EventData {
    impact_speed: f64,
    b: f64,
    k: f64,
    separation_distance: f64,
    reduced_mass: f64,
}

pub fn omega_0_from_k(k: f64, m: f64) -> f64 {
    (k / m).sqrt()
}

pub fn omega_l(k: f64, m: f64, b: f64) -> f64 {
    let omega_0 = omega_0_from_k(k, m);
    let omega_0_sq = omega_0 * omega_0;
    let b_sq = b * b;
    let omega_l_sq = omega_0_sq - b_sq / (4. * m * m);
    omega_l_sq.sqrt()
}

fn fast_forward(particle: &mut Particle, current_time: f64) {
    // kick-step
    let dt = current_time - particle.t;

    particle.p += particle.v * dt;
    particle.t = current_time;
}

const COEFF_RES: f64 = 0.5;

fn beta2(v_0: f64, pen_depth: f64) -> f64 {
    (-v_0 * 2. * COEFF_RES.ln() * COEFF_RES.sqrt()) / (pen_depth * PI)
}

#[inline(always)]
fn omega_0_sq(beta_val: f64) -> f64 {
    let ln_coeff_res_sq = COEFF_RES.ln() * COEFF_RES.ln();
    (beta_val * beta_val * (ln_coeff_res_sq + PI * PI)) / (4. * ln_coeff_res_sq)
}

fn b_and_k2(v_0: f64, m: f64, pen_depth: f64) -> (f64, f64) {
    let beta_val = beta2(v_0, pen_depth);
    let omega_0_sq_val = omega_0_sq(beta_val);

    (beta_val * m, omega_0_sq_val * m)
}

fn get_next_time(
    separation_distance: f64,
    current_impact_vel: f64,
    event_time: f64,
    k: f64,
    m: f64,
    b: f64,
    relative_speed_estimate: f64,
    r1: f64,
    r2: f64,
) -> (f64, f64) {
    let desired_collision_step_count: i32 = 10;
    let pen_fraction = 0.01;
    // let omega_0 = omega_0_from_k(k, m);
    // assert!(omega_0 >= 0.);

    let omega_l = omega_l(k, m, b);

    // T = 1/f = 2\pi/\omega
    // Time of collision is T/2
    let collision_time = std::f64::consts::PI / omega_l;
    let collision_time_dt = collision_time / desired_collision_step_count as f64;
    // println!("dt_a {}", collision_time_dt);

    // TODO: abstract out gravity forces

    // how far two particles can intersect without things getting out of hand
    let max_ok_pen_estimate = f64::max(r1, r2) * pen_fraction;

    let (mut dt, distance_for_global_speed_estimate) = if separation_distance < 0. {
        (collision_time_dt, max_ok_pen_estimate)
    } else {
        // NOTE: this right here injects relative_speed_estimate into the collision time calculation
        let current_impact_speed = current_impact_vel.abs();

        // v * t = d
        let impact_time_dt = separation_distance / current_impact_speed;

        // max( dist/(2*v_normal) and 1/(\omega_0 C) )

        // should this be min?
        // No: otherwise we get a zeno's paradox
        // the collision should be processed at steps of dt
        (
            f64::max(
                impact_time_dt.abs() / 2.,
                collision_time_dt, // 1. / (omega_l * self.desired_collision_step_count as f64),
            ),
            f64::max(separation_distance, max_ok_pen_estimate),
        )
    };
    // println!("dt_b = {}", dt);

    // impact_time_estimate is to ensure that if there is some fast moving particles around,
    // dt will be small enough such that if one particle in this pair gets hit, this pair will get updated properly
    // this is if one of the fastest particles suddenly crashes into one of the pair of particles
    // processed here and transfers all its speed
    let impact_time_dt_from_speed_estimate =
        distance_for_global_speed_estimate / relative_speed_estimate;
    // println!("dist = {}, rel = {}", distance_for_global_speed_estimate, relative_speed_estimate);

    dt = f64::min(dt, impact_time_dt_from_speed_estimate);
    // println!("dt_c = {}", dt);

    if dt == 0. {
        panic!(
            "dt is 0. This should not happen and will create an infinite loop.\n separation distance: {}\n current_impact_vel: {}\n event_time: {}\n k: {}\n m: {}\n b: {}\n",
            separation_distance, current_impact_vel, event_time, k, m, b
        );
    }

    if event_time + dt == event_time {
        panic!(
            "dt is so small its a rounding error.\nevent_time + dt == event_time. This should not happen and will create an infinite loop.\n separation distance: {}\n current_impact_vel: {}\n event_time: {}\n k: {}\n m: {}\n b: {}\n",
            separation_distance, current_impact_vel, event_time, k, m, b)
    }

    let next_time = event_time + dt;

    (next_time, dt)
}

fn compute_acc(
    p1: &mut Particle,
    p2: &mut Particle,
) -> (Vector, Vector, EventData) {
    let current_relative_speed = Particle::relative_speed(p1, p2);
    let x_len = (p1.p - p2.p).mag();
    let x_hat = (p1.p - p2.p) / x_len;
    let separation_distance = x_len - p1.r - p2.r;
    let vji = p1.v - p2.v;


    let impact_speed = current_relative_speed;

    let reduced_mass = (p1.m * p2.m) / (p1.m + p2.m);

    let (b, k) =
        b_and_k2(impact_speed, reduced_mass, f64::min(p1.r, p2.r));

    let info = EventData {
        impact_speed,
        b,
        k,
        separation_distance,
        reduced_mass,
    };

    if separation_distance < 0. {
        // colliding
        // spring-force
        let f_spring = x_hat * -k * separation_distance;
        let f_damp = vji * -b;

        let f_total = f_spring + f_damp;

        (f_total / p1.m, -f_total / p2.m, info)
    } else {
        // gravity
        (calc_pp_accel(p1, p2), calc_pp_accel(p2, p1), info)
    }
}

fn process_pair_get_dv(
    p1: &mut Particle,
    p2: &mut Particle,
    current_time: f64,
    relative_speed_estimate: f64,
) -> (Vector, Vector, f64) {
    let (acc1, acc2, info) = compute_acc(p1, p2);

    #[allow(unused_variables)]
    let (next_time, dt) = get_next_time(
        info.separation_distance,
        info.impact_speed,
        current_time,
        info.k,
        info.reduced_mass,
        info.b,
        relative_speed_estimate,
        p1.r,
        p2.r,
    );

    let ret = (
        acc1 * (next_time - current_time),
        acc2 * (next_time - current_time),
        next_time
    );

    ret
}

pub fn process_collision(p1: &mut Particle, p2: &mut Particle, event_time: f64) -> f64 {
    fast_forward(p1, event_time);
    fast_forward(p2, event_time);

    let relative_speed_estimate = (p1.v - p2.v).mag();

    let (dv1, dv2, next_time) = process_pair_get_dv(
        p1,
        p2,
        event_time,
        relative_speed_estimate,
    );

    p1.apply_dv(dv1);
    p2.apply_dv(dv2);
    next_time
}