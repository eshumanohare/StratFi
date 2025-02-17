use std::slice::Iter;
// A tick refers to some asset's price at a specific timestamp
#[derive(Debug)]
pub struct Tick {
    pub price: f64,
    pub timestamp: usize
}

impl Tick {
    pub fn null() -> Tick {
        Tick {
            price: 0f64,
            timestamp: 0usize
        }
    }
}

// TickField keeps track of ticks for a specific asset
pub struct TickField {
    pub symbol: String,
    pub ticks: Vec<Tick>
}

impl TickField {
    // creates a new tick field for a specific asset
    pub fn new(_symbol: &str) -> TickField {
        TickField {
            symbol: _symbol.to_string(),
            ticks: Vec::new()
        }
    }
    // push new tick
    pub fn push(&mut self, _tick: Tick) {
        self.ticks.push(_tick);
    }
    // iterate over the current ticks of an asset
    pub fn iter(&mut self) -> Iter<Tick> {
        return self.ticks.iter();
    }
}
