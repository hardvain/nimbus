extern crate stopwatch;

use stopwatch::{Stopwatch};
use rand::prelude::*;

mod core;
mod data;

use crate::core::{Indicator, Strategy, Signal, Params, Interval};
use std::collections::HashMap;
use crate::data::{Ticker, OHLCV, TimeSeries};
use chrono::{Utc, Duration};
use std::ops::Sub;

fn main() {
    let mut tickers: HashMap<Ticker, OHLCV> = HashMap::new();
    let mut load_time = 0;
    let intervals: Vec<Interval> = vec![Interval::OneMinute, Interval::FiveMinutes, Interval::TenMinutes, Interval::FifteenMinutes, Interval::ThirtyMinutes, Interval::OneHour, Interval::FourHours, Interval::OneDay];
    let mut sw = Stopwatch::start_new();
    let tick_capacity = 1000000;
    for ticker in 0..49 {
        for interval in &intervals {
            let mut open = TimeSeries::new(tick_capacity, Utc::now());
            let mut high = TimeSeries::new(tick_capacity, Utc::now());
            let mut low = TimeSeries::new(tick_capacity, Utc::now());
            let mut close = TimeSeries::new(tick_capacity, Utc::now());
            let mut volume = TimeSeries::new(tick_capacity, Utc::now());
            for i in 0..tick_capacity {
                open.push(i as f64);
                high.push(i as f64);
                low.push(i as f64);
                close.push(i as f64);
                volume.push(i as f64);
            }
            let t = Ticker { interval: interval.clone(), instrument_id: ticker.to_string() };
            tickers.insert(t, OHLCV {
                open,
                high,
                low,
                close,
                volume,
            });
        }
        println!("{}", ticker);
    }
    load_time = sw.elapsed_ms();
    sw.reset();
    sw.start();
    let rel30min = tickers.get(&Ticker { instrument_id: 3.to_string(), interval: Interval::ThirtyMinutes }).unwrap();
    println!("{}", rel30min.close.len());
    for i in 0..1000000 {
        let index: usize = rand::thread_rng().gen_range(0..tick_capacity);
        let OHLCV { open, high, low, close, volume } = rel30min;
        let v1 = open.get_timed_range(Utc::now() - Duration::seconds(1), Utc::now());
        let v2 = high.get_timed_range(Utc::now() - Duration::seconds(1), Utc::now());
        let v3 = low.get_timed_range(Utc::now() - Duration::seconds(1), Utc::now());
        let v4 = close.get_timed_range(Utc::now() - Duration::seconds(1), Utc::now());
        let v5 = volume.get_timed_range(Utc::now() - Duration::seconds(1), Utc::now());
        // println!("{}, {}, {}, {}, {}", v1.len(), v2.len(), v3.len(), v4.len(), v5.len());
    }

    println!("Loading: {:?} ms, Accessing {:?}ms", load_time, sw.elapsed_ms());
}


struct EMAIndicator {
    period: usize,
}


impl Indicator<f64, f64> for EMAIndicator {
    fn next(&mut self, input: &f64) -> f64 {
        input * 2.0
    }
}


impl From<(Params<usize>, Params<usize>)> for ParameterizedEMACrossOverStrategy {
    fn from((p1, p2): (Params<usize>, Params<usize>)) -> Self {
        let mut result: Vec<EMACrossOverStrategy> = vec![];
        for x in (p1.from..p1.to).step_by(p1.step) {
            for y in (p2.from..p2.to).step_by(p2.step) {
                result.push(EMACrossOverStrategy::new(x, y))
            }
        }
        ParameterizedEMACrossOverStrategy {
            strategies: result
        }
    }
}

struct DynamicEMACrossOverStrategy {
    strategies: Vec<EMACrossOverStrategy>
}

struct ParameterizedEMACrossOverStrategy {
    strategies: Vec<EMACrossOverStrategy>
}

impl Strategy<f64> for ParameterizedEMACrossOverStrategy {
    fn next<'a>(&mut self, input: &'a f64) -> Option<Signal<'a>> {
        self.strategies.iter_mut().map(|s| s.next(input));
        None
    }
}


struct EMACrossOverStrategy {
    fast_ema: EMAIndicator,
    slow_ema: EMAIndicator,
}

impl EMACrossOverStrategy {
    pub fn new(fast_period: usize, slow_period: usize) -> Self {
        EMACrossOverStrategy { fast_ema: EMAIndicator { period: fast_period }, slow_ema: EMAIndicator { period: slow_period } }
    }
}

impl Strategy<f64> for EMACrossOverStrategy {
    fn next<'a>(&mut self, input: &'a f64) -> Option<Signal<'a>> {
        let slow_ema_value = self.slow_ema.next(input);
        let fast_ema_value = self.fast_ema.next(input);
        if slow_ema_value < fast_ema_value { Some(Signal::Buy { entry_price: input }) } else { Some(Signal::Sell { entry_price: input }) }
    }
}
