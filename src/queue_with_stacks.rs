struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        MyQueue {
            s1: Vec::new(),
            s2: Vec::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.s1.is_empty() {
            self.s1.push(x);
        }
        else {
            self.s2.push(x);
        }
    }
    
    fn pop(&mut self) -> i32 {
        let x = self.s1.pop();
        if self.s1.is_empty() {
            while !self.s2.is_empty() {
                self.s1.push(self.s2.pop().unwrap());
            }
        }
        x.unwrap()
    }
    
    fn peek(&self) -> i32 {
        self.s1.last().unwrap().clone()
    }
    
    fn empty(&self) -> bool {
       self.s1.is_empty() && self.s2.is_empty() 
    }
}

