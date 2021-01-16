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
        format!("{}:{}", Clock::hour_formater(self.h), Clock::hour_formater(self.m))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }

    pub fn hour_formater(num: i32) -> String {
        if num >= 24 {
            let module_operator_result = num % 24;
            format!("{:0>2}", module_operator_result)
        }else{
            format!("{:0>2}", num)
        }
    }
}