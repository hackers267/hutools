pub struct Queue<T: Clone> {
    head: usize,
    tail: usize,
    data: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new(size: usize) -> Self {
        Queue {
            head: 0,
            tail: 0,
            data: Vec::with_capacity(size),
        }
    }
    pub fn push(&mut self, ele: T) {
        if self.is_full() {
            self.data.push(ele)
        } else {
            self.data[self.tail] = ele
        }
        self.move_tail();
    }
    pub fn pop(&mut self) -> Option<T> {
        let option = self.data.get(self.head).cloned();
        self.move_head();
        option
    }

    fn move_tail(&mut self) {
        if self.tail + 1 < self.data.capacity() {
            self.tail += 1
        } else {
            self.tail = 0
        }
    }

    fn move_head(&mut self) {
        if self.head + 1 < self.data.capacity() {
            self.head += 1
        } else {
            self.head = 0
        }
    }

    fn is_full(&self) -> bool {
        self.tail == self.data.len() && self.tail < self.data.capacity()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new(5);
        queue.push(1);
        queue.push(2);
        let result = queue.pop();
        assert_eq!(result, Some(1))
    }
}
