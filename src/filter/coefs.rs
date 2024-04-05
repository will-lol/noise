#[derive(Copy, Clone)]
pub struct Coefficients {
    pub a0: f32,
    pub a1: f32,
    pub a2: f32,
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
}

pub trait Bandpass {
    fn new_bandpass(sampling_frequency: f32, center_frequency: f32, q_factor: f32) -> Self;
}

impl Coefficients {
    // pub fn new_bandpass(sampling_frequency: f32, center_frequency: f32, q_factor: f32) -> Self {
    //     let w = 2.0 * std::f32::consts::PI * (center_frequency / sampling_frequency);
    //     let alpha = f32::sin(w) / 2.0 * q_factor;
    //     return Self {
    //         a0: 1.0 + alpha,
    //         a1: -2.0 * f32::cos(w),
    //         a2: 1.0 - alpha,
    //         b0: q_factor * alpha,
    //         b1: 0.0,
    //         b2: q_factor * alpha * -1.0,
    //     };
    // }

    pub fn new_peaking_eq(
        sampling_frequency: f32,
        center_frequency: f32,
        db: f32,
        q_factor: f32,
    ) -> Self {
        let w = 2.0 * std::f32::consts::PI * (center_frequency / sampling_frequency);
        let alpha = f32::sin(w) / (2.0 * q_factor);
        let a = f32::powf(10.0, db / 40.0);
        return Self {
            a0: 1.0 + (alpha / a),
            a1: -2.0 * f32::cos(w),
            a2: 1.0 - (alpha / a),
            b0: 1.0 + (alpha * a),
            b1: -2.0 * f32::cos(w),
            b2: 1.0 - (alpha * a),
        };
    }
}
