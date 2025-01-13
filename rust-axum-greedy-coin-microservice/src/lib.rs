/* Greedy Coin Change Logic*/

pub fn greedy_coin_change(amount: u32) -> Vec<u32> {
    let mut coins = vec![1, 5, 10, 25];
    coins.sort();
    coins.reverse();

    let mut change = vec![];
    let mut remaining = amount;

    for coin in coins {
        while remaining >= coin {
            remaining -= coin;
            change.push(coin);
        }
    }

    change
}

//test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greedy_coin_change() {
        assert_eq!(greedy_coin_change(0), vec![]);
        assert_eq!(greedy_coin_change(1), vec![1]);
        assert_eq!(greedy_coin_change(5), vec![5]);
        assert_eq!(greedy_coin_change(10), vec![10]);
        assert_eq!(greedy_coin_change(25), vec![25]);
        assert_eq!(greedy_coin_change(26), vec![25, 1]);
        assert_eq!(greedy_coin_change(27), vec![25, 1, 1]);
        assert_eq!(greedy_coin_change(28), vec![25, 1, 1, 1]);
        assert_eq!(greedy_coin_change(29), vec![25, 1, 1, 1, 1]);
        assert_eq!(greedy_coin_change(30), vec![25, 25]);
        assert_eq!(greedy_coin_change(31), vec![25, 25, 1]);
    }
}
