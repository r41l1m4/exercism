#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {seconds: s}
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        crate::years_corrected!(d.seconds, 0.2408467)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        crate::years_corrected!(d.seconds, 0.61519726)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        crate::years_corrected!(d.seconds, 1.0)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        crate::years_corrected!(d.seconds, 1.8808158)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        crate::years_corrected!(d.seconds, 11.862615)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        crate::years_corrected!(d.seconds, 29.447498)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        crate::years_corrected!(d.seconds, 84.016846)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        crate::years_corrected!(d.seconds, 164.79132)
    }
}

#[macro_export]
macro_rules! years_corrected {
    ($seconds:literal, $orbital_constant:literal) => {
        (((((($seconds as f64 / 60f64) / 60f64) / 24f64) / 365.25) / $orbital_constant as f64) * 100.0).round() / 100.0
    };
    ($seconds:expr, $orbital_constant:literal) => {
        (((((($seconds as f64 / 60f64) / 60f64) / 24f64) / 365.25) / $orbital_constant as f64) * 100.0).round() / 100.0
    }
}

fn main() {
    println!("{}", Earth::years_during(&Duration::from(2_134_835_688)));

    println!("{}", years_corrected!(2_134_835_688, 0.2408467));

}
