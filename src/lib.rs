pub fn moving_average(prices: &[f64], window: usize) -> Vec<f64> {
    if prices.len() < window {
        return Vec::new();
    }
    prices
        .windows(window)
        .map(|w| w.iter().sum::<f64>() / window as f64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_average() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let ma = moving_average(&data, 3);
        assert_eq!(ma, vec![2.0, 3.0, 4.0]);
    }
}
