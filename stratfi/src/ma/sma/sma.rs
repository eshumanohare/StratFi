// Simple Moving Average keeps track of the average price in a specified period with equal weight to all the prices.
// Formula:
// sma = (P1 + P2 + P3 + ... + Pn)/n
// When Pn+1 is added to the window then P1 is dropped
// Use case: Since sma assigns equal weight to all prices it is not much affected by the volatality of the current price
// and hence it is suitable for long term strategies

use crate::tick::tick::Tick;
use std::collections::VecDeque;

pub struct SMA {
    pub period: usize, // window size of the sma
    pub ticks: VecDeque<Tick>, // stores period number of ticks
    pub sum: f64 // sum of the prices of an asset in a window
}

impl SMA {
    pub fn new(_period: usize) -> Self {
        SMA {
            period: _period,
            ticks: VecDeque::with_capacity(_period),
            sum: 0f64
        }
    }
    // push a tick to sma indicator and get the sma
    pub fn push(&mut self, _tick: Tick) -> f64 {
        self.sum += _tick.price;
        self.ticks.push_back(_tick);
        // if the new tick belongs to the new epoch then remove the oldest tick
        if self.ticks.len() > self.period {
            if let Some(front_tick) = self.ticks.pop_front() {
                self.sum -= front_tick.price;
            }
        }
        
        self.sum / self.ticks.len() as f64
    }
}


