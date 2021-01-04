use chrono::{DateTime, Utc};
use std::ops::Add;

pub trait Indicator<Input, Output> {
    fn next(&mut self, input: &Input) -> Output;
}

pub trait Strategy<Input> {
    fn next<'a>(&mut self, input: &'a Input) -> Option<Signal<'a>>;
}

pub struct MultiTimeFrameStrategy {}

pub struct MultiTickerStrategy {}

pub trait Scanner<Input> {
    fn filter(&mut self, input: Input) -> bool;
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Interval {
    OneMinute,
    FiveMinutes,
    TenMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    FourHours,
    OneDay,
}

pub struct TimeFrame {
    interval: Interval,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

pub enum Signal<'a> {
    Buy { entry_price: &'a f64 },
    Sell { entry_price: &'a f64 },
}

pub struct Params<I: Add> {
    name: String,
    pub from: I,
    pub to: I,
    pub step: I,
}

pub struct StrategySummary {
    pnl: f64,
    open_pnl: f64,
}

pub struct BackTest<Input> {
    strategy: Box<dyn Strategy<Input>>,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    summary: StrategySummary,
}

impl<Input> BackTest<Input> {
    pub fn new(strategy: Box<dyn Strategy<Input>>, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        BackTest {
            strategy,
            summary: StrategySummary { pnl: 0.0, open_pnl: 0.0 },
            from,
            to,
        }
    }

    pub fn run(&mut self, data: &[Input]) -> &StrategySummary {
        data.iter().map(|input| self.strategy.next(input));
        &self.summary
    }
}

pub struct LiveRun<Input> {
    strategy: Box<dyn Strategy<Input>>
}

impl<Input> LiveRun<Input> {
    pub fn new(strategy: Box<dyn Strategy<Input>>) -> Self {
        LiveRun {
            strategy,
        }
    }

    pub fn start(&mut self, data: &[Input]) {
        data.iter().map(|input| self.strategy.next(input));
    }
}