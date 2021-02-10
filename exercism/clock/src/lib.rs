use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        (self.hours == other.hours) && (self.minutes == other.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*
        f.debug_struct("Clock")
            .field("hours", &self.hours)
            .eld("minutes", &self.minutes)
            .finish()
               */

        let mut hstr = String::new();
        let mut m_str = String::new();
        hstr.push_str(&self.hours.to_string());
        m_str.push_str(&self.minutes.to_string());
        if m_str.len() == 1 {
            m_str.insert(0, '0');
        }
        write!(f, "{}:{}", hstr, m_str) //"8:00"
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hstr = String::new();
        let mut sstr = String::new();
        if self.hours < 10 {
            hstr.push('0');
        }
        hstr.push_str(&self.hours.to_string());
        if self.minutes < 10 {
            sstr.push('0');
        }
        sstr.push_str(&self.minutes.to_string());
        write!(f, "{}:{}", hstr, sstr) //"08:00"
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h: i32 = 0;
        let mut m: i32 = minutes;

        if minutes >= 60 || minutes <= -60 {
            h += (minutes / 60) as i32;
        }

        m %= 60;

        if h < 0 {
            h = ((h + hours) % 24) + 24;
        } else {
            h = (h + hours) % 24;
        }

        if m < 0 {
            m += 60;
            h -= 1;
        }

        if h < 0 {
            h += 24;
        }

        h %= 24;

        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut h: i32 = self.hours;
        let mut m: i32 = self.minutes + minutes;

        if m >= 60 || m <= -60 {
            h += (m / 60) as i32;
        }

        m %= 60;

        if h < 0 {
            h = (h % 24) + 24;
        } else {
            h %= 24;
        }

        if m < 0 {
            m += 60;
            h -= 1;
        }

        if h < 0 {
            h += 24;
        }

        h %= 24;

        Clock {
            hours: h,
            minutes: m,
        }
    }
}
