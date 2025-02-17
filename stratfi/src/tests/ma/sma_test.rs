#[cfg(test)]
mod tests {
    use crate::ma::sma::sma::SMA;
    use crate::tick::tick::Tick;
    use crate::data_sources::data_sources::fetch_data;

    #[test]
    fn test_sma() {
        let mut sma = SMA::new(3);
        let ticks = fetch_data();
        // use the fetched data to verify the sma
        let res1 = sma.push(Tick{price: 10.0, timestamp: 123});
        assert_eq!(res1, 10.0);
        let res2 = sma.push(Tick{price: 20.0, timestamp: 124});
        assert_eq!(res2, 15.0);
        let res3 = sma.push(Tick{price: 30.0, timestamp: 125});
        assert_eq!(res3, 20.0);
        let res4 = sma.push(Tick{price: 40.0, timestamp: 126});
        assert_eq!(res4, 30.0);
    }
}