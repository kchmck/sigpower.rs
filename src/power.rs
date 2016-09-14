use num::complex::Complex32;

/// Calculate the power (dBm) into the resistance (ohms) of the given samples.
pub fn power_dbm(samples: &[Complex32], resistance: f32) -> f32 {
    // Units of Watt-ohms
    let avg = samples.iter().fold(0.0, |s, x| {
        s + x.norm_sqr()
    }) / samples.len() as f32;

    // Power in Watts.
    let power = avg / resistance;

    // Convert Watts to dBm.
    30.0 + 10.0 * power.log10()
}

#[cfg(test)]
mod test {
    use super::*;
    use num::complex::Complex32;

    #[test]
    fn test_dbm() {
        assert!((power_dbm(&[Complex32::new(2.0, 0.0)], 50.0) - 19.03089987) < 0.0001);
        assert!((power_dbm(&[Complex32::new(0.5, 0.0)], 50.0) - 6.989700043) < 0.0001);
        assert!((power_dbm(&[Complex32::new(10.0, 0.0)], 50.0) - 33.0102996) < 0.0001);
        assert!((power_dbm(&[Complex32::new(0.1, 0.0)], 50.0) - -6.989700043) < 0.0001);
    }
}

