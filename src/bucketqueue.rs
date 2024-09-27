use std::collections::VecDeque;
use rand::Rng;




pub struct Bqueue<T:Copy + PartialOrd>{
    bucketwidth: f64,
    len: usize,
    data: Vec<VecDeque<T>>,
    start: usize
}

impl<T:Copy + PartialOrd> Bqueue<T> {
    pub fn new(bucketnum: usize, bucketwidth: f64) -> Self {
        return Self {
            len: 0,
            start: 0,
            bucketwidth: bucketwidth,
            data: vec![VecDeque::new();bucketnum]
        }
    }

    pub fn push(&mut self, elem: T,key: f64) {
        let index = (key/self.bucketwidth).floor() as usize;
        self.data[index].push_back(elem);
        self.len = self.len +1;
        if index < self.start || self.len() == 1{
            self.start = index;
        }
    }

    pub fn pop(&mut self) -> Option<T>{
        if self.is_empty() {
            return None
        } else {
            let y = self.data[self.start].pop_front();
            self.len = self.len -1;
            if !self.is_empty() {
                while self.data[self.start].is_empty() {
                    self.start = self.start +1;
                }
            }
            return y
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None
        } else {
            return self.data[self.start].front()
        }
        
    }

    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn it_works() {
        let max = 500;
        let div = 5;
        let total = max*div;

        let mut heap1: Bqueue<f64> = Bqueue::new(max+1,1.0);
        assert_eq!(heap1.is_empty(), true);
        heap1.push(500.0,500.0);
        assert_eq!(heap1.is_empty(), false);
        assert_eq!(heap1.len(), 1);
        assert_eq!(heap1.peek(), Some(&500.0));
        assert_eq!(heap1.pop(), Some(500.0));
        assert_eq!(heap1.is_empty(), true);

        
    
        for i in 1..=total {
            let y = (i as f64)/div as f64;
            heap1.push(y, y);
            assert_eq!(heap1.len(), i);
            assert_eq!(heap1.peek(), Some(&(1.0/div as f64)));
        }
        assert_eq!(heap1.len(), total);
        for i in 1..=total {
            let y = (i as f64)/div as f64;
            assert_eq!(heap1.peek(), Some(&y));
            assert_eq!(heap1.pop(), Some(y));
            assert_eq!(heap1.len(), total-i);
            
        }
        assert_eq!(heap1.is_empty(), true);

        let mut rng = rand::thread_rng();

        let mut vector = Vec::new();

        for i in 1..=total {
            let n:f64 = rng.gen_range(0.0..=max as f64);
            vector.push(n)
        }
        let mut sortvec = vector.clone();
        sortvec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        

        for i in 1..=total {
            let y = vector[i-1];
            heap1.push(y, y);
            assert_eq!(heap1.len(), i);
            let min = &vector[0..i].iter().fold(f64::INFINITY, |a, &b| a.min(b));
            assert_eq!(heap1.peek().unwrap().floor(), min.floor());
        }
        assert_eq!(heap1.len(), total);

        for i in 1..=total {
            let min = sortvec.remove(0);
            assert_eq!(heap1.peek().unwrap().floor(), min.floor());
            assert_eq!(heap1.pop().unwrap().floor(), min.floor());
            assert_eq!(heap1.len(), total-i);
        }
        assert_eq!(heap1.is_empty(), true);

    }
}