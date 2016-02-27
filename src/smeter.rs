const SCALE_START: f32 = -93.0;
const SCALE_END: f32 = -141.0;
const SCALE_STEP: u32 = 6;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SignalLevel {
    Plus(f32),
    Level(u32),
    None,
}

impl SignalLevel {
    pub fn from_dbm(dbm: f32) -> SignalLevel {
        if dbm < SCALE_END {
            return SignalLevel::None;
        }

        if dbm > SCALE_START {
            return SignalLevel::Plus(dbm - SCALE_START);
        }

        let level = (dbm - SCALE_END) as u32 / SCALE_STEP + 1;

        SignalLevel::Level(level)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smeter() {
        assert_eq!(SignalLevel::from_dbm(-142.0), SignalLevel::None);
        assert_eq!(SignalLevel::from_dbm(-141.01), SignalLevel::None);
        assert_eq!(SignalLevel::from_dbm(-141.0), SignalLevel::Level(1));
        assert_eq!(SignalLevel::from_dbm(-135.0), SignalLevel::Level(2));
        assert_eq!(SignalLevel::from_dbm(-129.0), SignalLevel::Level(3));
        assert_eq!(SignalLevel::from_dbm(-123.0), SignalLevel::Level(4));
        assert_eq!(SignalLevel::from_dbm(-117.0), SignalLevel::Level(5));
        assert_eq!(SignalLevel::from_dbm(-111.0), SignalLevel::Level(6));
        assert_eq!(SignalLevel::from_dbm(-105.0), SignalLevel::Level(7));
        assert_eq!(SignalLevel::from_dbm(-99.0), SignalLevel::Level(8));
        assert_eq!(SignalLevel::from_dbm(-93.0), SignalLevel::Level(9));
        assert_eq!(SignalLevel::from_dbm(-92.0), SignalLevel::Plus(1.0));
        assert_eq!(SignalLevel::from_dbm(-91.0), SignalLevel::Plus(2.0));
        assert_eq!(SignalLevel::from_dbm(-83.0), SignalLevel::Plus(10.0));
        assert_eq!(SignalLevel::from_dbm(-73.0), SignalLevel::Plus(20.0));
    }
}
