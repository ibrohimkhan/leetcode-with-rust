// https://leetcode.com/problems/design-a-stack-with-increment-operation/

struct CustomStack {
    data: Vec<i32>,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self { data: Vec::with_capacity(max_size as usize), }
    }
    
    fn push(&mut self, x: i32) {
        if self.data.capacity() <= self.data.len() { return; }
        self.data.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.data.pop().unwrap_or(-1)
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        for item in self.data.iter_mut().take(k as usize) {
            *item += val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CustomStack;

    #[test]
    fn test_stack_item_incrementation() {
        let mut stack = CustomStack::new(3);
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.data, vec![1, 2]);

        stack.pop();
        assert_eq!(stack.data, vec![1]);

        stack.push(2);
        stack.push(3);
        stack.push(4);
        assert_eq!(stack.data, vec![1, 2, 3]);

        stack.increment(5, 100);
        assert_eq!(stack.data, vec![101, 102, 103]);

        stack.increment(2, 100);
        assert_eq!(stack.data, vec![201, 202, 103]);

        assert_eq!(103, stack.pop());
        assert_eq!(202, stack.pop());
        assert_eq!(201, stack.pop());
        assert_eq!(-1, stack.pop());
    }
}
