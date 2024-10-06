use std::collections::HashSet;
pub(crate) use std::{collections::BinaryHeap, f64::consts::PI, time::Instant}; 

mod csvreader;
mod bucketqueue;
use ordered_float::OrderedFloat;
use rayon::iter::IntoParallelIterator;
use std::cmp::Ordering;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct KeyVal {
    pub key:OrderedFloat<f64>, //the time the pair collides at
    pub val:csvreader::Rec, //all information p1,p2,p1x,p2x .. etc
    pub id:(u32,u32),        //p1,p2
    pub index: usize        //so it can be looked up easy within the data matrix
}
// KeyVal needs to be ordered so I can stick it in a priority queue
impl Ord for KeyVal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for KeyVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for KeyVal {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for KeyVal {}

impl bucketqueue::HasKey for KeyVal {
    fn key(&self) -> OrderedFloat<f64> {
        self.key
    }
}

pub trait SeqentialPriorityQueue<'a, E: Ord> {
    fn push(&mut self, e: &'a E);
    fn pop(&mut self) -> Option<&E>;
    fn is_empty(&self) -> bool;
}

impl <'a, E: Ord> SeqentialPriorityQueue<'a, E> for BinaryHeap<&'a E> {
    fn push(&mut self, e: &'a E) {
        BinaryHeap::push(self, e);
    }
    fn pop(&mut self) -> Option<&E> {
        BinaryHeap::pop(self)
    }
    fn is_empty(&self) -> bool {
        BinaryHeap::is_empty(self)
    }
}

impl <'a, E: Ord + bucketqueue::HasKey> SeqentialPriorityQueue<'a, E> for bucketqueue::Bqueue<&'a E> {
    fn push(&mut self, e: &'a E) {
        bucketqueue::Bqueue::push(self, e);
    }
    fn pop(&mut self) -> Option<&E> {
        bucketqueue::Bqueue::pop(self)
    }
    fn is_empty(&self) -> bool {
        bucketqueue::Bqueue::is_empty(self)
    }
}

pub trait ParrallelPriorityQueue<'a, E: Ord + 'a> {
    fn bulk_process<F: Fn(&'a E) -> Option<&'a E>>(&mut self, f: F);
    // fn bulk_push<I: Iterator<Item = Option<&'a E>>>(&mut self, es: I);
    // fn bulk_pop(&mut self) -> &Vec<&'a E>;
}

fn time_seqential<'a, PQ: SeqentialPriorityQueue<'a, KeyVal>>(data : &'a Vec<Vec<KeyVal>>, heap: &'a mut PQ) -> (Duration, i64) {
    let now = Instant::now();
    let mut count = 0;

    for i in 0..data.len() {
        // Add initial population of events.
        let mut ids = HashSet::new();        
        for k in &data[i] {
            if !ids.contains(&k.id) {
                heap.push(k);
                ids.insert(k.id);
            }
        }
        // Process events in that step
        while !heap.is_empty() {
            //pop the first element
            let elem = heap.pop().unwrap();
            let id = elem.id;
            let index = elem.index;
            //if the set contains another element with the same id push the first occuring element into the priority queue
            count += 1;
            data[i][index+1..].into_iter().find(|k| k.id == id).iter().for_each(|k| heap.push(k));
        }
    }
    (now.elapsed(), count)
}

fn time_parallel<'a, PQ: SeqentialPriorityQueue<'a, KeyVal> + ParrallelPriorityQueue<'a, KeyVal>>(data : &'a Vec<Vec<KeyVal>>, heap: &'a mut PQ) -> (Duration, i64) {
    let now = Instant::now();

    for i in 0..data.len() {
        // Add initial population of events.
        let mut ids = HashSet::new();        
        for k in &data[i] {
            if !ids.contains(&k.id) {
                heap.push(k);
                ids.insert(k.id);
            }
        }
        // Process events in that step
        while !heap.is_empty() {
            heap.bulk_process(|elem| {
                let id = elem.id;
                let index = elem.index;
                //if the set contains another element with the same id push the first occuring element into the priority queue
                data[i][index+1..].into_iter().find(|k| k.id == id)
            });
        }
    }
    (now.elapsed(), 0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut arecord = csvreader::csvcon(&args[1]).unwrap();

    let mut data : Vec<Vec<KeyVal>> = vec![Vec::new();500];
    
    let mut max:f64= 0.0;
    //was orginally just a key value pair but, for the exculsion list to work
    // an id to idenitfy what index pair of rocks is in the this key value was added 


    //finds the end time of the entire data set
    for i in 0..arecord.len() {
        let poppy = arecord[i];
        if poppy.time > max {
            max = poppy.time;
        }
    }

    while !arecord.is_empty() {
        //here is why arecord is a dequeue as the entries into data need to be in the same order 
        //as the entries into arecord
        let poppy = arecord.pop_front().unwrap();
        //uses the max to find which global timestep each record belongs to then places them 
        //in the corresponding vector
        let index:usize = (poppy.time/(max/500.0)).floor() as usize;
        let ind = data[index].len();
        data[index].push(KeyVal{key:OrderedFloat(poppy.time),val:poppy,id:(poppy.p1 as u32,poppy.p2 as u32),index: ind});

    }
    // let now = Instant::now();

    let mut heapt: BinaryHeap<&KeyVal> = BinaryHeap::new();
    let elapsed = time_seqential(&data, &mut heapt);
    println!("Binary Heap Elapsed: {:.2?}", elapsed);

    // let now1 = Instant::now();

    let delta:f64 = 2.0*PI*1E-4 - 2.0*PI*1E-5;


    let mut heap1: bucketqueue::Bqueue<&KeyVal> = bucketqueue::Bqueue::new(((max/delta).ceil()+1.0) as usize,delta); //intialize the Bucket queue
    let elapsed1 = time_seqential(&data, &mut heap1);
    println!("Bucket Queue Elapsed: {:.2?}", elapsed1);

    //println!("{}",data[100].len());
    //println!("first p1: {}",arecord[0].p1);



}
