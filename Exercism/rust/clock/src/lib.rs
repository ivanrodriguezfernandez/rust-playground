use std::fmt;

const RHS_HOURS: i32 = 24;
const RHS_MINUTES: i32 = 60;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    h: i32,
    m: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let mut h = hours + minutes.div_euclid(RHS_MINUTES);

        let m = minutes.rem_euclid(RHS_MINUTES);

        h = match h.checked_rem_euclid(RHS_HOURS) {
            Some(result_hour) => result_hour,
            _ => 0
        };
        
        Clock { 
            h: h, 
            m: m
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let add_minutes: i32 = self.m + minutes;
        Self::new(self.h, add_minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.h, self.m)
    }
}