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
        pub val:csvreader::Rec,
        pub id:(f64,f64)
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
        data[index].push(Keyval{key:OrderedFloat(poppy.time),val:poppy,id:(poppy.p1,poppy.p2)});

    }
    let now = Instant::now();

    let mut heap: BinaryHeap<Keyval> = BinaryHeap::new();
    let mut exc: Vec<(f64,f64)> = Vec::new();


    for j in 0..data.len() {
        exc.push(data[j][0].id);
        heap.push(data[j][0]);
        for i in 0..data[j].len() {
            if  !exc.contains(&data[j][i].id)  {
                exc.push(data[j][i].id);
                heap.push(data[j][i]);
            } else if !heap.is_empty()  {
                let mut y = heap.pop().unwrap();
                exc.retain(|x| x != &y.id );
                while y.id != data[j][i].id && !heap.is_empty() {
                    y = heap.pop().unwrap();
                    exc.retain(|x| x != &y.id );
                }   
            }
            
        }
        while !heap.is_empty() {
            heap.pop();
        }
    }


    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    //println!("{}",data[100].len());
    //println!("first p1: {}",arecord[0].p1);



}
