const HOURS: i32 = 24;
const MINUTES:i32 = 60;
pub struct Clock {
    h: i32,
    m: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            h: hours,
            m: minutes,
        }
    }

    pub fn to_string(mut self) -> String {
        self.h  = self.h + self.m.div_euclid(MINUTES);
        self.m  = self.m.rem_euclid(MINUTES);
        self.h = self.h.rem_euclid(HOURS);
        format!("{:0>2}:{:0>2}", self.h,self.m)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.h,self.m +  minutes)
    }
}