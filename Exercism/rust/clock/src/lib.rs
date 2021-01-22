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
    pub fn to_string(&mut self) -> String {
        if self.m < 0 {
            let abs_value:i32 = self.m.abs();
            self.m = 60 - abs_value.rem_euclid(60);
            if abs_value.div_euclid(60) == 0{
                self.h = self.h -1;
            }else{
                self.h = self.h - (abs_value.div_euclid(60));
            }
        }

        if self.m.div_euclid(60) > 0 {
            self.h = self.h + self.m.div_euclid(60);
            self.m = self.m.rem_euclid(60);
        }

        if self.h.rem_euclid(24) >= 0 {
            format!("{}:{}",Clock::padding_zero(self.h.rem_euclid(24)),Clock::padding_zero(self.m))
        }else{
            format!("{}:{}", Clock::padding_zero(self.h),Clock::padding_zero(self.m))
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
    pub fn padding_zero(num:i32) -> String { 
        format!("{:0>2}", num)
    }
}