use smeter::SignalLevel;

const TIMER_STOP: u32 = 20;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SquelchThreshold {
    Open,
    Level(u32),
}

pub struct Squelch {
    threshold: SquelchThreshold,
    timer: u32,
}

impl Squelch {
    pub fn new(threshold: SquelchThreshold) -> Squelch {
        Squelch {
            threshold: threshold,
            timer: 0,
        }
    }

    pub fn set_threshold(&mut self, threshold: SquelchThreshold) {
        self.threshold = threshold;
    }

    pub fn is_squelched(&mut self, level: SignalLevel) -> bool {
        use self::SquelchThreshold::*;

        let above = match self.threshold {
            Open => true,
            Level(thresh) => match level {
                SignalLevel::Plus(_) => true,
                SignalLevel::Level(s) => s >= thresh,
                SignalLevel::None => false,
            },
        };

        if above {
            self.timer = 0;
            false
        } else if self.timer < TIMER_STOP {
            self.timer += 1;
            false
        } else {
            true
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
