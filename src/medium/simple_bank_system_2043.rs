// https://leetcode.com/problems/simple-bank-system/

#![allow(dead_code)]
struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }
    
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if self.balance.len() < account1 as usize || self.balance.len() < account2 as usize { return false; }
        self.withdraw(account1, money) && self.deposit(account2, money)
    }
    
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if self.balance.len() < account as usize { return false; }
        self.balance[account as usize - 1] += money;
        true
    }
    
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if self.balance.len() < account as usize || self.balance[account as usize - 1] < money { return false; }
        self.balance[account as usize - 1] -= money;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Bank;

    #[test]
    fn test() {
        let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
        assert!(bank.withdraw(3, 10));
        assert!(bank.transfer(5, 1, 20));
        assert!(bank.deposit(5, 20));
        assert_eq!(bank.transfer(3, 4, 15), false);
        assert_eq!(bank.withdraw(10, 50), false);
    }
}
