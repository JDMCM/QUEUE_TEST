use csv::Error;



pub struct Bqueue<T:Copy + PartialOrd>{
    bucketwidth: f64,
    len: usize,
    data: Vec<Vec<T>>,
    start: usize
}

impl<T:Copy + PartialOrd> Bqueue<T> {
    pub fn new(bucketnum: usize, bucketwidth: f64) -> Self {
        return Self {
            len: 0,
            start: 0,
            bucketwidth: bucketwidth,
            data: vec![Vec::new();bucketnum]
        }
    }

    pub fn push(&mut self, elem: T,key: f64) {
        let index = (key/self.bucketwidth).floor() as usize;
        self.data[index].push(elem);
        self.len = self.len+1;
        if index < self.start {
            self.start = index;
        }
    }

    pub fn pop(&mut self) -> Option<T>{
        while self.data[self.start].is_empty() {
            self.start = self.start +1;
        }
        
        self.len = self.len-1;
        if self.is_empty() {
            return None
        } else {
            return Some(self.data[self.start].remove(0))
        }
    }

    pub fn peek(&self) -> Option<T> {
        if self.is_empty() {
            return None
        } else {
            return Some(self.data[self.start][0])
        }
        
    }

    pub fn is_empty(&self) -> bool {
        return self.data[self.start].is_empty() && self.start == self.data.len()-1;
    }

    pub fn size(&self) -> usize {
        return self.len;
    }

}