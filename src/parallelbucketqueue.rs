use std::sync::Mutex;
use rayon::prelude::*;

use crate::{sequentialbucketqueue::HasKey, ParallelPriorityQueue};

#[derive(Debug)]
pub struct ParBqueue<T:Send>{
    bucketwidth: f64,
    data: Vec<Mutex<Vec<T>>>,
    start: usize
}

impl<'a, T:HasKey + Send + Sync> ParBqueue<&'a T> {
    pub fn new(bucketnum: usize, bucketwidth: f64) -> Self {
        let mut datas:Vec<Mutex<Vec<&'a T>>> = Vec::with_capacity(bucketnum);
        (0..bucketnum).into_iter().for_each(|_i| {
            datas.push(Mutex::new(Vec::new()));
        });
        return Self {
            start: bucketnum,
            bucketwidth,
            data: datas
        }
    }

    pub fn push(&mut self, elem: &'a T) {
        let index = (elem.key()/self.bucketwidth).floor() as usize;
        self.data[index].lock().unwrap().push(elem);
        if index < self.start {
            self.start = index;
        }
    }

    pub fn pop(&mut self) -> Option<&T>{
        if self.is_empty() {
            return None
        } else {
            let y = self.data[self.start].lock().unwrap().pop();
            while self.start < self.data.len() && self.data[self.start].lock().unwrap().is_empty() {
                self.start = self.start +1;
            }
            return y
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None
        } else {
            return Some(self.data[self.start].lock().unwrap()[0]);
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.start >= self.data.len();
    }

    pub fn bulk_process<F: Fn(&'a T) -> Option<&'a T> + Sync + Send>(&mut self, f: F) {
        let bucket = self.bulk_pop();
        let mapped = bucket.map(f).flatten();
        self.bulk_push(mapped);
    }

    pub fn bulk_push<I: ParallelIterator<Item = &'a T>>(&mut self, es: I) {
        // TODO: This can be smarter, but it comes with overhead. Groupby the index and push all indices at once.
        let indices = es.map(|i| {
            let index = (i.key()/self.bucketwidth).floor() as usize;
            self.data[index].lock().unwrap().push(i);
            index
        });
        let mindex_opt = indices.min();
        if let Some(mindex) = mindex_opt {
            if self.start > mindex {
                self.start = mindex;
            }
        }
    }

    pub fn bulk_pop(&mut self) -> impl ParallelIterator<Item = &'a T> {
        self.data.push(Mutex::new(Vec::new()));
        let bucket = self.data.swap_remove(self.start).into_inner().unwrap();
        // println!("{}", bucket.len());
        self.advance_start();
        bucket.into_par_iter()
    }

    fn advance_start(&mut self) {
        while self.start < self.data.len() && self.data[self.start].lock().unwrap().is_empty() {
            self.start = self.start +1;
        }
    }
}

impl <'a, E: Ord + HasKey + Send + Sync> ParallelPriorityQueue<'a, E> for ParBqueue<&'a E> {
    fn push(&mut self, e: &'a E) {
        ParBqueue::push(self, e);
    }
    fn pop(&mut self) -> Option<&E> {
        ParBqueue::pop(self)
    }
    fn is_empty(&self) -> bool {
        ParBqueue::is_empty(self)
    }
    fn bulk_process<F: Fn(&'a E) -> Option<&'a E> + Sync + Send>(&mut self, f: F) {
        ParBqueue::bulk_process(self, f);
    }
    fn bulk_push<I: ParallelIterator<Item = &'a E>>(&mut self, es: I) {
        ParBqueue::bulk_push(self, es);
    }
    fn bulk_pop(&mut self) -> impl ParallelIterator<Item = &'a E> {
        ParBqueue::bulk_pop(self)
    }
}

#[cfg(test)]
mod tests {

    use rand::Rng;

    use super::*;
    
    #[test]
    fn it_works() {
        let max = 500;
        let div = 5;
        let total = max*div;
        let value = 500.0;

        let mut heap1: ParBqueue<&f64> = ParBqueue::new(max+1,1.0);
        assert_eq!(heap1.is_empty(), true);
        heap1.push(&value);
        assert_eq!(heap1.is_empty(), false);
        assert_eq!(heap1.peek(), Some(&value));
        assert_eq!(heap1.pop(), Some(&value));
        assert_eq!(heap1.is_empty(), true);

        
        let mut values = Vec::new();
        for i in 1..=total {
            let y = (i as f64)/div as f64;
            values.push(y);
        }
        for i in 0..total {
            heap1.push(&values[i]);
            assert_eq!(heap1.peek(), Some(&(1.0/div as f64)));
        }
        for i in 1..=total {
            let y = (i as f64)/div as f64;
            assert_eq!(heap1.peek(), Some(&y));
            assert_eq!(heap1.pop(), Some(&y));
        }
        assert_eq!(heap1.is_empty(), true);

        let mut rng = rand::thread_rng();

        let mut vector = Vec::new();

        for _i in 1..=total {
            let n:f64 = rng.gen_range(0.0..=max as f64);
            vector.push(n)
        }
        let mut sortvec = vector.clone();
        sortvec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        

        for i in 1..=total {
            let y = &vector[i-1];
            heap1.push(y);
            let min = &vector[0..i].iter().fold(f64::INFINITY, |a, &b| a.min(b));
            assert_eq!(heap1.peek().unwrap().floor(), min.floor());
        }

        for _ in 1..=total {
            let min = sortvec.remove(0);
            assert_eq!(heap1.peek().unwrap().floor(), min.floor());
            assert_eq!(heap1.pop().unwrap().floor(), min.floor());
        }
        assert_eq!(heap1.is_empty(), true);

    }
}