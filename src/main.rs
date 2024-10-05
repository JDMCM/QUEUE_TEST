pub(crate) use std::{collections::BinaryHeap, f64::consts::PI, time::Instant}; 

mod csvreader;
mod bucketqueue;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

#[derive(Debug,Clone,Copy)]
pub struct Keyval {
    pub key:OrderedFloat<f64>, //the time the pair collides at
    pub val:csvreader::Rec, //all information p1,p2,p1x,p2x .. etc
    pub id:(f64,f64),        //p1,p2
    pub index: usize        //so it can be looked up easy within the data matrix
}
// keyval needs to be ordered so I can stick it in a priority queue
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

fn main() {
    let mut arecord = csvreader::csvcon().unwrap();

    let mut data : Vec<Vec<Keyval>> = vec![Vec::new();500];
    
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
        data[index].push(Keyval{key:OrderedFloat(poppy.time),val:poppy,id:(poppy.p1,poppy.p2),index: ind});

    }
    let now = Instant::now();

    let mut heapt: BinaryHeap<Keyval> = BinaryHeap::new();

    for i in 0..data.len() {
        //find a set of the first occuring unique index events
        let mut unq_set: Vec<Keyval> = data[i].clone();
        //sort them by index then order the indexs by time
        unq_set.sort_by(|a,b| (a.id,a.key).partial_cmp(&(b.id,b.key)).unwrap());
        unq_set.dedup_by(|a,b| a.id.eq(&b.id));
        //add the first unique ids to the priority queue

        for k in unq_set {
            heapt.push(k);
        }
        while !heapt.is_empty() {
            //pop the first element
            let elem = heapt.pop().unwrap();
            let id = elem.id;
            let index = elem.index;
            //if the set contains another element with the same id push the first occuring element into the priority queue
            for k in &data[i][index+1..] {
                if k.id == id {
                    heapt.push(data[i][k.index]);  
                } 
            }
        }
    }


    let elapsed = now.elapsed();
    println!("Binary Heap Elapsed: {:.2?}", elapsed);

    let now1 = Instant::now();

    let delta:f64 = 2.0*PI*1E-4 - 2.0*PI*1E-5;


    let mut heap1: bucketqueue::Bqueue<Keyval> = bucketqueue::Bqueue::new(((max/delta).ceil()+1.0) as usize,delta); //intialize the Bucket queue

    for i in 0..data.len() {
        //find a set of the first occuring unique index events
        let mut unq_set: Vec<Keyval> = data[i].clone();
        //sort them by index then order the indexs by time
        unq_set.sort_by(|a,b| (a.id,a.key).partial_cmp(&(b.id,b.key)).unwrap());
        unq_set.dedup_by(|a,b| a.id.eq(&b.id));
        //add the first unique ids to the priority queue

        for k in unq_set {
            heap1.push(k,*k.key);
        }
        while !heap1.is_empty() {
            //pop the first element
            let elem = heap1.pop().unwrap();
            let id = elem.id;
            let index = elem.index;
            //if the set contains another element with the same id push the first occuring element into the priority queue
            for k in &data[i][index+1..] {
                if k.id == id {
                    heap1.push(data[i][k.index], *k.key);  
                } 
            }
        }
    }


    let elapsed1 = now1.elapsed();
    println!("Bucket Queue Elapsed: {:.2?}", elapsed1);
    //println!("{}",data[100].len());
    //println!("first p1: {}",arecord[0].p1);



}
