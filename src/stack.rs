
#[derive(Debug)]
pub struct Stack<T> {
    pub arr: Vec<T>,
    pub len: usize
}

#[allow(dead_code)]
impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            arr: Vec::new(),
            len: 0
        }
    }

    pub fn push(&mut self, t: T) {
        self.len += 1;
        self.arr.push(t);
    }

    pub fn pop(&mut self) -> T {
        self.len -= 1;
        self.arr.pop().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.len==0
    }

    pub fn peek(&self) -> &T {
        &self.arr[self.len-1]
    }
}
