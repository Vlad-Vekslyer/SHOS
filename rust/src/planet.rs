use web_sys;

use crate::utils::set_panic_hook;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// https://www.mathsisfun.com/algebra/trig-solving-sss-triangles.html
fn calculate_angle(initial_x: f32, initial_y: f32) -> f32 {
    let side_a = initial_x.abs();
    let side_c = initial_y.abs();
    let side_b = calculate_pythagorean(side_a, side_c);

    log!(
        "angle {}(radians)",
        ((side_b.powi(2) + side_c.powi(2) - side_a.powi(2)) / (2.0 * side_b * side_c)).acos()
    );
    ((side_b.powi(2) + side_c.powi(2) - side_a.powi(2)) / (2.0 * side_b * side_c)).acos()
}

fn calculate_perihelion(initial_x: f32, initial_y: f32) -> f32 {
    let is_at_angle = initial_y != 0.0;
    match is_at_angle {
        false => initial_x.abs(),
        true => calculate_pythagorean(initial_y.abs(), initial_x.abs()),
    }
}

// https://sciencing.com/calculate-period-orbit-5840979.html
fn calculate_eccentricity(initial_x: f32, initial_y: f32, semi_major_axis: f32) -> f32 {
    let perihelion = calculate_perihelion(initial_x, initial_y);
    let aphelion = (semi_major_axis * 2.0) - perihelion;
    log!("aphelion {}", aphelion);
    log!("perihelion {}", perihelion);

    log!(
        "eccentricity {}",
        (aphelion - perihelion) / (aphelion + perihelion)
    );
    (aphelion - perihelion) / (aphelion + perihelion)
}

// https://en.wikipedia.org/wiki/Semi-major_and_semi-minor_axes
fn calculate_semi_minor_axis(semi_major_axis: f32, eccentricity: f32) -> f32 {
    log!(
        "semi_minor_axis {}",
        semi_major_axis * (1.0 - eccentricity.powi(2)).sqrt()
    );
    semi_major_axis * (1.0 - eccentricity.powi(2)).sqrt()
}

fn calculate_pythagorean(a: f32, b: f32) -> f32 {
    (a.powi(2) + b.powi(2)).sqrt()
}

pub struct Planet {
    pub radius: f32,
    eccentricity: f32,
    semi_major_axis: f32,
    semi_minor_axis: f32,
    angle: f32,
    //  keep track of planet coordinates as if the orbit followed the standard ellipse equation.
    //  these coordinates then get transformed to match the actual ellipse equation of the orbit
    // https://www.maa.org/external_archive/joma/Volume8/Kalman/General.html
    standard_coords: [f32; 2],
}

impl Planet {
    pub fn new(radius: f32, initial_x: f32, initial_y: f32, semi_major_axis: f32) -> Planet {
        set_panic_hook();
        Planet::validate(initial_x, initial_y, semi_major_axis);

        let eccentricity = calculate_eccentricity(initial_x, initial_y, semi_major_axis);
        let semi_minor_axis = calculate_semi_minor_axis(semi_major_axis, eccentricity);
        let angle = calculate_angle(initial_x, initial_y);

        Planet {
            radius,
            semi_major_axis,
            semi_minor_axis,
            eccentricity,
            angle,
            standard_coords: [semi_major_axis, 0.0],
        }
    }

    fn validate(initial_x: f32, initial_y: f32, semi_major_axis: f32) {
        let perihelion = calculate_perihelion(initial_x, initial_y);
        if perihelion >= semi_major_axis {
            panic!("Perihelion cannot be larger than the semi major axis");
        }
    }

    pub fn tick(&mut self) -> [f32; 2] {
        let next_standard_x = self.standard_coords[0] + 0.01;
        let next_standard_y = (self.semi_minor_axis
            * (self.semi_major_axis.powi(2) - next_standard_x.powi(2)))
            / self.semi_major_axis;

        self.standard_coords = [next_standard_x, next_standard_y];
        self.transform_standard_coords()
    }

    // rotate and translate
    fn transform_standard_coords(&self) -> [f32; 2] {}
}
