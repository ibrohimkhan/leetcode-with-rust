// https://leetcode.com/problems/apply-discount-every-n-orders/
#![allow(dead_code)]

struct Cashier {
    items: [i64; 201],
    discount: i64,
    discount_period: i32,
    counter: i32,
}

impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut array = [0i64; 201];
        for (product_id, price_id) in products.into_iter().zip(prices) {
            array[product_id as usize] = price_id as i64;
        }

        Self { items: array, discount: discount as _, discount_period: n, counter: 0 }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut total = 0i64;
        for (product_id, quantity) in product.into_iter().zip(amount) {
            total += quantity as i64 * self.items[product_id as usize];
        }

        self.counter += 1;
        if self.counter % self.discount_period == 0 {
            (total * (100 - self.discount)) as f64 / 100.
        } else {
            total as _
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cashier;

    #[test]
    fn test() {
        let mut cashier = Cashier::new(
            3,
            50,
            vec![1, 2, 3, 4, 5, 6, 7],
            vec![100, 200, 300, 400, 300, 200, 100],
        );

        let bill = cashier.get_bill(vec![1, 2], vec![1, 2]);
        assert_eq!(bill, 500.);

        let bill = cashier.get_bill(vec![3, 7], vec![10, 10]);
        assert_eq!(bill, 4000.);

        let bill = cashier.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(bill, 800.);

        let bill = cashier.get_bill(vec![4], vec![10]);
        assert_eq!(bill, 4000.);

        let bill = cashier.get_bill(vec![7, 3], vec![10, 10]);
        assert_eq!(bill, 4000.);

        let bill = cashier.get_bill(vec![7, 5, 3, 1, 6, 4, 2], vec![10, 10, 10, 9, 9, 9, 7]);
        assert_eq!(bill, 7350.);

        let bill = cashier.get_bill(vec![2, 3, 5], vec![5, 3, 2]);
        assert_eq!(bill, 2500.);
    }
}
