use std;

pub struct Bqueue<T:Copy + PartialOrd>{
    bucketwidth: f64,
    len: usize,
    data: Vec<Vec<T>>,
    start: usize
}

impl<T:Copy + PartialOrd> Bqueue<T> {
    fn new(bucketnum: usize, bucketwidth: f64) -> Self {
        return Self {
            len: 0,
            start: 0,
            bucketwidth: bucketwidth,
            data: vec![Vec::new();bucketnum]
        }
    }

    fn enqueue(&mut self, elem: T,key: f64) {
        let index = (key/self.bucketwidth).floor() as usize;
        self.data[index].push(elem);
        self.len = self.len+1;
    }

    fn dequeue(&mut self) -> T{
        if (self.data[self.start].is_empty()) {
            self.start = self.start +1;
        }
        
        self.len = self.len-1;
        return self.data[self.start].remove(0)
    }

    fn peek(&self) -> T {
        return self.data[self.start][0];
    }

    fn is_empty(&self) -> bool {
        return self.data[self.start].is_empty() && self.start == self.data.len()-1;
    }

    fn size(&self) -> usize {
        return self.len;
    }

}