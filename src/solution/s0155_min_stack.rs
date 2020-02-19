/**
 * [155] Min Stack
 *
 *
 * Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
 *
 *
 * push(x) -- Push element x onto stack.
 *
 *
 * pop() -- Removes the element on top of the stack.
 *
 *
 * top() -- Get the top element.
 *
 *
 * getMin() -- Retrieve the minimum element in the stack.
 *
 *
 *
 *
 * Example:<br />
 *
 * MinStack minStack = new MinStack();
 * minStack.push(-2);
 * minStack.push(0);
 * minStack.push(-3);
 * minStack.getMin();   --> Returns -3.
 * minStack.pop();
 * minStack.top();      --> Returns 0.
 * minStack.getMin();   --> Returns -2.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/min-stack/
// discuss: https://leetcode.com/problems/min-stack/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
这题居然是 easy... 我怀疑人生了, getMin() 怎么能做到常数时间? Heap 也是 LogN 啊

看了最高票解之后...........天哪, 我可太菜了

核心思想是保证每次 pop 时都能以常数时间更新最小值, 这就需要在空间上以某种方式记录下来

那一种做法就是存储每个元素和最小值之间的差值, 这样 pop 的时候就能不断还原出原始值

另一种更直观的做法就是每次入栈 min 时, 都把前一个 min (当前第二小的数字) 放在它前面, 作为记录
*/
struct MinStack {
    vec: Vec<i32>,
    min: i32,
}

impl MinStack {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        MinStack {
            vec: Vec::new(),
            min: i32::max_value(),
        }
    }

    pub fn push(&mut self, x: i32) {
        if x <= self.min {
            self.vec.push(self.min);
            self.min = x;
        }
        self.vec.push(x);
    }

    pub fn pop(&mut self) {
        if self.vec.pop().unwrap() == self.min {
            self.min = self.vec.pop().unwrap();
        }
    }

    pub fn top(&self) -> i32 {
        *self.vec.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_155() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3); // --> Returns -3.
        min_stack.pop();
        assert_eq!(min_stack.top(), 0); // --> Returns 0.
        assert_eq!(min_stack.get_min(), -2); // --> Returns -2.[]
    }
}
