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
        format!("{}:{}", Clock::add_zero(self.h), Clock::add_zero(self.m))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }

    pub fn add_zero(num: i32) -> String {
        if num.to_string().capacity() == 1 {
            format!("0{}", num.to_string())
        } else {
            num.to_string()
        }
    }
}
