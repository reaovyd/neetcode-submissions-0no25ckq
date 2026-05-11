impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = prices[0];
        let mut res = 0;
        for price in prices {
            res = res.max(price - min_price);
            min_price = min_price.min(price);
        }
        res
    }
}
