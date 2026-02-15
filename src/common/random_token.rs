use rand::distr::{Alphanumeric, SampleString};

pub fn generate_secure_random_string(length: usize) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), length)
}
