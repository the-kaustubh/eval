
#[derive(Debug)]
pub struct Queue<T> {
    pub arr: Vec<T>,
    pub len: usize
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            arr: Vec::new(),
            len: 0
        }
    }

    pub fn enqueue(&mut self, t: T) {
        self.len += 1;
        self.arr.push(t);
    }

    pub fn dequeue(&mut self) -> T {
        self.len -= 1;
        self.arr.pop().unwrap()
    }

    pub fn length(&self) -> usize {
        self.len as usize
    }
}
