# Clock solution 

## Another person solution

```
use chrono::{Duration, NaiveTime};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    time: NaiveTime,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Local Vsariables
        let mut result_time = NaiveTime::from_hms(0, 0, 0);
        let hour_duration = Duration::hours(hours as i64);
        let minutes_duration = Duration::minutes(minutes as i64);

        // Calculate time
        result_time += hour_duration;
        result_time += minutes_duration;

        // Return
        Clock {
            time: result_time,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Add minutes to time object
        Clock {
            time: self.time + Duration::minutes(minutes as i64),
        }
    }
}

// Display Trait implementation for Clock
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time.format("%H:%M"))
    }
}
```

```
use std::fmt;

#[derive(Debug, Default)]
pub struct Clock {
	hours: i32,
	minutes: i32,
}

impl fmt::Display for Clock {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:02}:{:02}", modulo(self.hours, 24), self.minutes % 60)
	}
}

impl PartialEq for Clock {
	fn eq(&self, other: &Self) -> bool {
		self.to_string() == other.to_string()
	}
}

impl Clock {
	fn default() -> Self {
		Self {
			hours: 0,
			minutes: 0,
		}
	}

	// Clocks may keep negative hours, but will always keep positive minutes.
	pub fn new(hours: i32, minutes: i32) -> Self {
		let mut clock = Clock::default();
		clock.hours = hours + (minutes - modulo(minutes, 60)) / 60;
		clock.minutes = modulo(minutes, 60);
		clock
	}

	pub fn add_minutes(mut self, minutes: i32) -> Self {
		self.minutes += minutes;
		self.hours += (self.minutes - modulo(self.minutes, 60)) / 60;
		self.minutes = modulo(self.minutes, 60);
		self
	}
}

// I'm used to Lua, where the % operator behaves somewhat differently,
// especially regarding negative numbers.

// This function mimics the required behaviour, but may not be complete.
fn modulo(a: i32, b: i32) -> i32 {
	let r = a % b;
	if r < 0 {
		return r + b;
	}
	r
}
```

---

## My:


```
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
```