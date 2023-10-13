use std::ops::{ShlAssign, AddAssign};

fn main() {
    let clock = Clock::new(
        [true; 4],
        [true; 6]
    );

    println!("{}", clock.read());

    let mut clock_2 = Clock::new(
        [false; 4],
        [false; 6],
    );

    *clock_2.minutes.last_mut().unwrap() = true;    
    println!("{}", clock_2.read());
    clock_2 <<= 1;
    println!("{}", clock_2.read());
    clock_2 <<= 1;
    println!("{}", clock_2.read());
}

struct Clock {
    hours: [bool; 4],
    minutes: [bool; 6]
}

impl Clock {
    pub fn new(hours: [bool; 4], minutes: [bool; 6]) -> Self {
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

        format!("{hours:0>2}:{minutes:0>2}",)
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
            self.minutes.rotate_left(1);
            *self.minutes.last_mut().unwrap() = false;
            self.hours.rotate_left(1);
            *self.hours.last_mut().unwrap() = hours_bool;
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
        todo!()   
    }
}

fn read_watch(number: usize) {
    // input
    assert_eq!(number, 1);
    
}
