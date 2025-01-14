// Greedy algorithm for coin change
pub fn greedy_coin_change(amount: i64) -> Vec<u32> {
    let coins: [u32; 4] = [25, 10, 5, 1]; // Coin denominations
    let mut amount = amount.abs() as u32; // Take absolute value for negative inputs
    let mut result: Vec<u32> = vec![];

    for &coin in &coins {
        let count = amount / coin;
        result.push(count);
        amount %= coin;
    }

    // Ensure the result has the correct number of elements
    while result.len() < 4 {
        result.push(0);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_cases() {
        // $1.32 (132 cents)
        assert_eq!(greedy_coin_change(132), vec![5, 0, 1, 2]);

        // $0.00 (0 cents)
        assert_eq!(greedy_coin_change(0), vec![0, 0, 0, 0]);

        // $2.41 (241 cents)
        assert_eq!(greedy_coin_change(241), vec![9, 1, 1, 1]);
    }

    #[test]
    fn test_edge_cases() {
        // Large value: $10.99 (1099 cents)
        assert_eq!(greedy_coin_change(1099), vec![43, 2, 0, 4]);

        // Negative value: -$0.75 (negative 75 cents)
        assert_eq!(greedy_coin_change(-75), vec![3, 0, 0, 0]);

        // Small value: $0.04 (4 cents)
        assert_eq!(greedy_coin_change(4), vec![0, 0, 0, 4]);
    }
}
