use std::{collections::BinaryHeap, env, error::Error, ffi::OsString, fs::File, process, time::Instant}; 

mod csvreader;
mod bucketqueue;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

fn main() {
    let mut arecord = csvreader::csvcon().unwrap();



    let mut data : Vec<Vec<Keyval>> = vec![Vec::new();500];

    
    let mut max:f64= 0.0;

    #[derive(Debug,Clone,Copy)]
    pub struct Keyval {
        pub key:OrderedFloat<f64>,
        pub val:csvreader::Rec
    }

    impl Ord for Keyval {
        fn cmp(&self, other: &Self) -> Ordering {
            self.key.cmp(&other.key)
        }
    }

    impl PartialOrd for Keyval {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Keyval {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    impl Eq for Keyval {}




    for i in 0..arecord.len() {
        let poppy = arecord[i];
        if(poppy.time > max) {
            max = poppy.time;
        }
    }

    while !arecord.is_empty() {
        let poppy = arecord.pop().unwrap();
        let index:usize = (poppy.time/(max/500.0)).floor() as usize;
        data[index].push(Keyval{key:OrderedFloat(poppy.time),val:poppy});

    }
    let now = Instant::now();

    let mut heap: BinaryHeap<Keyval> = BinaryHeap::new();


    for j in 0..data.len() {
        heap.push(data[j][0].clone());
        for i in 0..data[j].len() {
            if  !(Some(heap.peek().unwrap().val.p1) != Some(data[j][i].val.p1) && Some(heap.peek().unwrap().val.p2) != Some(data[j][i].val.p2)) {
                heap.pop();
            } 
            heap.push(data[j][i].clone());
        }
        while !heap.is_empty() {
            heap.pop();
        }
    }


    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{}",data[100].len());
    //println!("first p1: {}",arecord[0].p1);



}
