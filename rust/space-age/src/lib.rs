#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn orbital_period_relative_to_earth() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 60.0 / 60.0 / 24.0 / 365.25 / Self::orbital_period_relative_to_earth()
    }
}

macro_rules! Planet {
    ( $name:ident, $orbital_period_relative_to_earth:literal ) => {
        pub struct $name;
        impl Planet for $name {
            fn orbital_period_relative_to_earth() -> f64 {
                $orbital_period_relative_to_earth
            }
        }
    };
}

Planet!(Mercury, 0.2408467);
Planet!(Venus, 0.61519726);
Planet!(Earth, 1.0);
Planet!(Mars, 1.8808158);
Planet!(Jupiter, 11.862615);
Planet!(Saturn, 29.447498);
Planet!(Uranus, 84.016846);
Planet!(Neptune, 164.79132);
