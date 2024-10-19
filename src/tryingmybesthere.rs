use std::collections::VecDeque;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use crossbeam_queue::SegQueue;

use crate::ParallelPriorityQueue;

pub trait HasKey {
    fn key(&self) -> OrderedFloat<f64>;
}

#[derive(Debug)]
pub struct ParBqueue<T:Copy + PartialOrd + Sized + Send>{
    bucketwidth: f64,
    len: usize,
    data: Vec<SegQueue<T>>,
    start: usize
}

impl<'a, T:Copy + PartialOrd + HasKey + Send + Sync> ParBqueue<&'a T> {
    pub fn new(bucketnum: usize, bucketwidth: f64) -> Self {
        return Self {
            len: 0,
            start: bucketnum,
            bucketwidth,
            data: Vec::with_capacity(bucketnum)
        }
    }

    pub fn push(&mut self, elem: &'a T) {
        let index = (elem.key()/self.bucketwidth).floor() as usize;
        self.data[index].push(elem);
        self.len += 1;
        if index < self.start {
            self.start = index;
        }
    }

    pub fn pop(&mut self) -> Option<&T>{
        if self.is_empty() {
            return None
        } else {
            let y = self.data[self.start].pop();
            self.len -= 1;
            while self.start < self.data.len() && self.data[self.start].is_empty() {
                self.start = self.start +1;
            }
            return y
        }
    }

    pub fn bulk_push(&mut self, elem: Vec<&'a T>) {
        // let mathed: Vec<(T,usize)> = elem.into_par_iter().map(|i| {
        //     return (i,(i.key()/self.bucetwidth).floor() as usize)
        // }).collect();

        let mut math: Vec<&'a T> = elem;
    
        let maths = math.into_par_iter().for_each(|i| {
            self.data[(i.key()/self.bucketwidth).floor() as usize].push(i);
        });

    
        


    }


    // pub fn peek(&self) -> Option<&T> {
    //     if self.is_empty() {
    //         return None
    //     } else {
    //         return self.data[self.start].peek().copied()
    //     }
    // }

    pub fn is_empty(&self) -> bool {
        return self.start >= self.data.len();
    }

    // If we don't want this method, this version can get rid of the len field completely.
    pub fn len(&self) -> usize {
        return self.len;
    }

}

impl <'a, E: Copy + Ord + HasKey + Send + Sync> ParallelPriorityQueue<'a, E> for ParBqueue<&'a E> {
    fn push(&mut self, e: &'a E) {
        ParBqueue::push(self, e);
    }
    fn pop(&mut self) -> Option<&E> {
        ParBqueue::pop(self)
    }
    fn is_empty(&self) -> bool {
        ParBqueue::is_empty(self)
    }
    fn bulk_process<F: Fn(&'a E) -> Option<&'a E>>(&mut self, f: F) {

    }
    fn bulk_push<I: Iterator<Item = &'a E>>(&mut self, es: I) {
      
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

        let mut heap1: ParBqueue<&f64> = ParBqueue::new(max+1,1.0);
        assert_eq!(heap1.is_empty(), true);
        heap1.push(&value);
        assert_eq!(heap1.is_empty(), false);
        assert_eq!(heap1.len(), 1);
        
        assert_eq!(heap1.pop(), Some(&value));
        assert_eq!(heap1.is_empty(), true);

        
        let mut values = Vec::new();
        for i in 1..=total {
            let y = (i as f64)/div as f64;
            values.push(y);
        }
        for i in 0..total {
            heap1.push(&values[i]);
            assert_eq!(heap1.len(), i + 1);
            
        }
        assert_eq!(heap1.len(), total);
        for i in 1..=total {
            let y = (i as f64)/div as f64;
            
            assert_eq!(heap1.pop(), Some(&y));
            assert_eq!(heap1.len(), total-i);
            
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
            assert_eq!(heap1.len(), i);
            let min = &vector[0..i].iter().fold(f64::INFINITY, |a, &b| a.min(b));
           
        }
        assert_eq!(heap1.len(), total);

        for i in 1..=total {
            let min = sortvec.remove(0);
            
            assert_eq!(heap1.pop().unwrap().floor(), min.floor());
            assert_eq!(heap1.len(), total-i);
        }
        assert_eq!(heap1.is_empty(), true);

    }
}