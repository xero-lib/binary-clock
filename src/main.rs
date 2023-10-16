use std::ops::{ShlAssign, AddAssign, ShrAssign};

fn main() {
    let mut clock = Clock::new();
    clock += true;
    clock <<= 2;
    println!("{}", clock.read());

    let mut clock_2 = Clock::new();
    clock_2 += true;
    clock_2 <<= 2;
    println!("{}", clock_2.read());

    clock_2 += clock;

    println!("{}", clock_2.read());
}

struct Clock {
    hours: [bool; 4],
    minutes: [bool; 6]
}

#[allow(unused)]
impl Clock {
    pub fn new() -> Self {
        Self {
            hours: [false; 4],
            minutes: [false; 6],
        }
    }

    pub fn from(hours: [bool; 4], minutes: [bool; 6]) -> Self {
        Self {
            hours,
            minutes,
        }
    }

    pub fn read(&self) -> String {
        let mut hours = 0;
        for i in 0..4 {
            if self.hours[3 - i] {
                hours += 2_u32.pow(i as u32);
            }
        }

        if hours > 12 {
            hours = 12;
        }
        
        let mut minutes = 0;
        for i in 0..6 {
            if self.minutes[5 - i] {
                minutes += 2_u32.pow(i as u32);
            }
        }

        if minutes > 59 {
            minutes = 59;
        }

        format!("{hours:0>2}:{minutes:0>2}")
    }
}

impl Clone for Clock {
    fn clone(&self) -> Self {
        Self {
            hours: self.hours,
            minutes: self.minutes
        }
    }
}

impl ShlAssign<usize> for Clock {
    fn shl_assign(&mut self, rhs: usize) {
        for _ in 0..rhs {
            let hours_bool = self.minutes[0];
            
            self.minutes.rotate_left(1); // could potentially rotate by rhs
            self.minutes[5] = false;

            self.hours.rotate_left(1);
            self.hours[3] = hours_bool;
        }
    }
}

impl ShrAssign<usize> for Clock {
    fn shr_assign(&mut self, rhs: usize) {
        for _ in 0..rhs {
            let minutes_bool = self.hours[3];

            self.hours.rotate_right(1); // could potentially rotate by rhs
            self.hours[0] = false;

            self.minutes.rotate_right(1);
            self.minutes[0] = minutes_bool;

        }
    }
}

impl AddAssign<bool> for Clock {
    fn add_assign(&mut self, rhs: bool) {
        if !rhs { return }
        *self.minutes.last_mut().unwrap() = true;
    }
}

impl AddAssign<Self> for Clock {
    fn add_assign(&mut self, rhs: Self) {
        let mut minute_carry = false;
        for i in 0..6 {
            let current_carry = minute_carry;
            minute_carry = current_carry as u8 + self.minutes[5 - i] as u8 + rhs.minutes[5 - i] as u8 >= 2;
            self.minutes[5 - i] = self.minutes[5 - i] as u8 + rhs.minutes[5 - i] as u8 + current_carry as u8 + 2 % 2 == 1;
        }

        let mut hour_carry = minute_carry;
        for i in 0..4 {
            let current_carry = hour_carry;
            hour_carry = current_carry as u8 + self.hours[3 - i] as u8 + rhs.hours[3 - i] as u8 >= 2;
            self.hours[3 - i] = self.hours[3 - i] as u8 + rhs.hours[3 - i] as u8 + current_carry as u8 + 2 % 2 == 1;
        }
    }
}
