use crate::common::kafka_aggregator::*;

#[derive(Debug)]
pub struct State(i64);

impl Aggregator for State {
    fn aggregate(&mut self, msg: &[u8]) {
        self.0 += 1;
    }
}

impl State {
    pub fn new (i: i64) -> State {
        State(i)
    }
}
