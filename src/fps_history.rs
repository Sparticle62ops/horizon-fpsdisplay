//! 1:1 of FpsHistory.java.

use std::collections::VecDeque;

pub struct FpsHistory {
    // java: private static final int LIMIT = 420;
    // java list is ArrayList + remove(0); VecDeque with pop_front is the
    // same observable behavior (FIFO cap at LIMIT).
    list: VecDeque<i32>,
}

impl Default for FpsHistory {
    fn default() -> Self {
        Self {
            list: VecDeque::with_capacity(420),
        }
    }
}

impl FpsHistory {
    pub fn add(&mut self, fps: i32) {
        self.list.push_back(fps);
        if self.list.len() > 420 {
            self.list.pop_front();
        }
    }

    // java: Collections.min(this.list); panics on an empty list the same way
    // (NoSuchElementException -> our unwrap on the empty iterator).
    pub fn get_minimum(&self) -> i32 {
        *self.list.iter().min().unwrap()
    }

    // java: Collections.max(this.list).
    pub fn get_maximum(&self) -> i32 {
        *self.list.iter().max().unwrap()
    }

    // java: int sum accumulated in an int, then integer division by size.
    pub fn get_average(&self) -> i32 {
        let mut sum: i64 = 0;
        for fps in &self.list {
            sum += *fps as i64;
        }
        (sum / self.list.len() as i64) as i32
    }
}
