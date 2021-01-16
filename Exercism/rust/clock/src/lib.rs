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
    pub fn to_string(&self) -> String {
        let mut hour:i32 = self.h;
        let mut minute:i32 = self.m;
        
        if self.m.div_euclid(60) > 0 {
            hour = hour + self.m.div_euclid(60);
            minute = self.m.rem_euclid(60);
        }

        if self.h.rem_euclid(24) == 0 {
            return format!("00:{}",Clock::padding_zero(minute)); 
        }
        if self.h.rem_euclid(24) > 0{
            return format!("{}:{}",Clock::padding_zero(hour.rem_euclid(24)),Clock::padding_zero(minute));
        }else{
            format!("{}:{}", Clock::padding_zero(hour),Clock::padding_zero(minute))
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
    pub fn padding_zero(num:i32) -> String { 
        format!("{:0>2}", num)
    }
}