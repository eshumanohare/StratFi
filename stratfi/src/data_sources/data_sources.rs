use crate::tick::tick::Tick;

pub fn fetch_data() -> Vec<Tick> {
    let mut ticks = Vec::new();
    ticks.push(Tick{price: 10.0, timestamp: 123});
    ticks.push(Tick{price: 20.0, timestamp: 124});
    ticks.push(Tick{price: 30.0, timestamp: 125});
    ticks.push(Tick{price: 40.0, timestamp: 126});

    return ticks;
}