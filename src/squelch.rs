use smeter::SignalLevel;
use ewma::{MovingAverage, MovingAverageWeight};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SquelchThreshold {
    Open,
    Level(u32),
}

struct SquelchWeight;

impl MovingAverageWeight for SquelchWeight {
    fn weight() -> f32 { 0.15 }
}

pub struct Squelch {
    threshold: SquelchThreshold,
    avg: MovingAverage<SquelchWeight>,
}

impl Squelch {
    pub fn new(threshold: SquelchThreshold) -> Squelch {
        Squelch {
            threshold: threshold,
            avg: MovingAverage::new(-117.0),
        }
    }

    pub fn set_threshold(&mut self, threshold: SquelchThreshold) {
        self.threshold = threshold;
    }

    pub fn is_squelched(&mut self, power: f32) -> bool {
        use self::SquelchThreshold::*;

        if power > self.avg.get() {
            self.avg.set(power);
        }

        match self.threshold {
            Open => false,
            Level(thresh) => match SignalLevel::from_dbm(self.avg.add(power)) {
                SignalLevel::Plus(_) => false,
                SignalLevel::Level(s) => s < thresh,
                SignalLevel::None => true,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_squelch() {
        let mut s = Squelch::new(SquelchThreshold::Level(3));

        assert!(!s.is_squelched(-129.0));
        assert!(!s.is_squelched(-129.0));
        assert!(!s.is_squelched(-135.0));
        assert!(!s.is_squelched(-135.0));
        assert!(!s.is_squelched(-135.0));
        assert!(!s.is_squelched(-135.0));
        assert!(s.is_squelched(-141.0));
        assert!(s.is_squelched(-137.0));
        assert!(s.is_squelched(-135.0));

        assert!(!s.is_squelched(-129.0));
        assert!(s.is_squelched(-141.0));

        assert!(!s.is_squelched(-123.0));
        assert!(!s.is_squelched(-141.0));
        assert!(!s.is_squelched(-141.0));
        assert!(s.is_squelched(-141.0));

        assert!(!s.is_squelched(-93.0));
        assert!(!s.is_squelched(-141.0));
        assert!(!s.is_squelched(-141.0));
        assert!(!s.is_squelched(-141.0));
        assert!(!s.is_squelched(-141.0));
        assert!(!s.is_squelched(-141.0));
        assert!(!s.is_squelched(-141.0));
        assert!(!s.is_squelched(-141.0));
        assert!(!s.is_squelched(-141.0));
        assert!(s.is_squelched(-141.0));
    }
}
