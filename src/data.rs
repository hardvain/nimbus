use crate::core::Interval;
use chrono::{DateTime, Utc};

#[derive(PartialEq, Eq, Hash)]
pub struct Ticker {
    pub interval: Interval,
    pub instrument_id: String,
}

pub struct TimeSeries {
    capacity: usize,
    data: Vec<f64>,
    start_index: i64,
    last_index:i64,
}

impl TimeSeries {
    pub fn new(capacity: usize, from: DateTime<Utc>) -> Self {
        TimeSeries { capacity, data: Vec::with_capacity(capacity), start_index: from.timestamp(), last_index:0 }
    }
    pub fn push(&mut self, value: f64) {
        self.data.push(value);
        self.last_index = self.last_index + 1;
    }
    pub fn get_by_index(&self, index: usize) -> &f64 {
        &self.data[index]
    }

    pub fn get_by_elapsed(&self, index: usize) -> &f64 {
        &self.data[self.last_index - index]
    }

    pub fn get_timed(&self, at: DateTime<Utc>) -> &f64 {
        let index: usize = (at.timestamp() - self.start_index) as usize;
        assert!(index > 0 && index < self.capacity);
        &self.data[index as usize]
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn get_timed_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> &[f64] {
        let from_index = (from.timestamp() - self.start_index) as usize;
        let to_index = (to.timestamp() - self.start_index) as usize;
        let capacity = self.capacity;
        assert!(from_index > 0 && from_index < capacity && to_index > 0 && to_index < capacity && to_index >= from_index);
        &self.data[from_index..to_index]
    }
}

pub struct OHLCV {
    pub open: TimeSeries,
    pub high: TimeSeries,
    pub low: TimeSeries,
    pub close: TimeSeries,
    pub volume: TimeSeries,
}

impl OHLCV {
    pub fn get_at(&self, index: usize) -> (&f64, &f64, &f64, &f64, &f64) {
        (self.open.get(index), self.high.get(index), self.low.get(index), self.close.get(index), self.volume.get(index))
    }

    pub fn get_timed_at(&self, at: DateTime<Utc>) -> (&f64, &f64, &f64, &f64, &f64) {
        (self.open.get_timed(at), self.high.get_timed(at), self.low.get_timed(at), self.close.get_timed(at), self.volume.get_timed(at))
    }

    pub fn get_timed_at_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> (&[f64], &[f64], &[f64], &[f64], &[f64]) {
        let open = self.open.get_timed_range(from, to);
        let high = self.high.get_timed_range(from, to);
        let low = self.low.get_timed_range(from, to);
        let close = self.close.get_timed_range(from, to);
        let volume = self.volume.get_timed_range(from, to);
        (high, high, low, close, volume)
    }
}