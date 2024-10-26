use std::collections::VecDeque;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::sync::{Mutex,MutexGuard};


use crate::ParallelPriorityQueue;

pub trait HasKey {
    fn key(&self) -> OrderedFloat<f64>;
}

#[derive(Debug)]
pub struct ParaBqueue<T:Copy + PartialOrd + Sized + Send>{
    bucketwidth: f64,
    len: usize,
    data: Vec<Mutex<Vec<T>>>,
    start: usize
}

impl<'a, T:Copy + PartialOrd + HasKey + Send + Sync> ParaBqueue<&'a T> {
    pub fn new(bucketnum: usize, bucketwidth: f64) -> Self {
        let mut datas:Vec<Mutex<Vec<&'a T>>> = Vec::with_capacity(bucketnum);
        (0..bucketnum).into_iter().for_each(|_i| {
            datas.push(Mutex::new(Vec::new()));
        });
        return Self {
            len: 0,
            start: bucketnum,
            bucketwidth,
            data: datas
        }
    }

    pub fn push(&mut self, elem: &'a T) {
        let index = (elem.key()/self.bucketwidth).floor() as usize;
        self.data[index].lock().unwrap().push(elem);
        self.len += 1;
        if index < self.start {
            self.start = index;
        }
    }

    pub fn pop(&mut self) -> Option<&T>{
        if self.is_empty() {
            return None
        } else {
            let y = self.data[self.start].lock().unwrap().pop();
            self.len -= 1;
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

    // If we don't want this method, this version can get rid of the len field completely.
    pub fn len(&self) -> usize {
        return self.len;
    }

    fn bulk_push<I: ParallelIterator<Item = &'a T>>(&mut self, elems: I) {
        let mut mindex = self.data.len();
        elems.for_each(|i| {
            let index = (i.key()/self.bucketwidth).floor() as usize;
            if index < mindex {
                mindex = index;
            }
            self.data[index].lock().unwrap().push(i);
        });
        if self.start > mindex {
            self.start = mindex;
        }

    }

    fn bulk_pop(&mut self) -> impl ParallelIterator<Item = &'a T> {
        self.data.push(Mutex::new(Vec::new()));
        let bucket = self.data.swap_remove(self.start).into_inner().unwrap();
        self.len = self.len - bucket.len();
        while self.start < self.data.len() && self.data[self.start].lock().unwrap().is_empty() {
              self.start = self.start +1;
        }
        return bucket.into_iter();
    }


}

impl <'a, E: Copy + Ord + HasKey + Send + Sync> ParallelPriorityQueue<'a, E> for ParaBqueue<&'a E> {
    fn push(&mut self, e: &'a E) {
        ParaBqueue::push(self, e);
    }
    fn pop(&mut self) -> Option<&E> {
        ParaBqueue::pop(self)
    }
    fn is_empty(&self) -> bool {
        ParaBqueue::is_empty(self)
    }
    fn bulk_process<F: Fn(&'a E) -> Option<&'a E> + Sync + Send>(&mut self, f: F) -> i32 {

    }
    fn bulk_push<I: ParallelIterator<Item = &'a E>>(&mut self, es: I) {
        ParaBqueue::bulk_push(self, es);
    }

    fn bulk_pop(&mut self) -> impl ParallelIterator<Item = &'a E> {
        ParaBqueue::bulk_pop(self)
    }
}

#[cfg(test)]
mod tests {

    use rand::Rng;

    use super::*;

    impl HasKey for f64 {
        fn key(&self) -> OrderedFloat<f64> {
            OrderedFloat(*self)
        }
    }
    
    #[test]
    fn it_works() {
        let max = 500;
        let div = 5;
        let total = max*div;
        let value = 500.0;

        let mut heap1: ParaBqueue<&f64> = ParaBqueue::new(max+1,1.0);
        assert_eq!(heap1.is_empty(), true);
        heap1.push(&value);
        assert_eq!(heap1.is_empty(), false);
        assert_eq!(heap1.len(), 1);
        assert_eq!(heap1.peek(), Some(&value));
        assert_eq!(heap1.pop(), Some(&value));
        assert_eq!(heap1.is_empty(), true);

        
        // let mut values = Vec::new();
        // for i in 1..=total {
        //     let y = (i as f64)/div as f64;
        //     values.push(y);
        // }
        // for i in 0..total {
        //     heap1.push(&values[i]);
        //     assert_eq!(heap1.len(), i + 1);
        //     assert_eq!(heap1.peek(), Some(&(1.0/div as f64)));
        // }
        // assert_eq!(heap1.len(), total);
        // for i in 1..=total {
        //     let y = (i as f64)/div as f64;
        //     assert_eq!(heap1.peek(), Some(&y.floor()));
        //     assert_eq!(heap1.pop(), Some(&y));
        //     assert_eq!(heap1.len(), total-i);
            
        // }
        // assert_eq!(heap1.is_empty(), true);

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
        
        //bulk test pray for me

        let mut rng = rand::thread_rng();

        let mut vector = Vec::new();

        for _i in 1..=total {
            let n:f64 = rng.gen_range(0.0..=max as f64);
            vector.push(n)
        }
        let mut sortvec = vector.clone();
        sortvec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        
        heap1.bulk_push(vector.iter());     
        assert_eq!(heap1.len(), total);

        while !heap1.is_empty() {
            let min = sortvec.remove(0);
            sortvec.retain(|i| {
                ((i.key()/heap1.bucketwidth).floor() as usize) > ((min.key()/heap1.bucketwidth).floor() as usize)
            });
            let bulkpopped = heap1.bulk_pop();
            for pop in bulkpopped {
                assert_eq!(pop.floor(), min.floor());
            }
        }
        assert_eq!(heap1.is_empty(), true);

        //bulk remove test, pray even harder may god hear my screams and be merciful upon this humble believer 


    }
}