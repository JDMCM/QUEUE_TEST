pub(crate) use std::{collections::BinaryHeap, f64::consts::PI, time::Instant}; 

mod csvreader;
mod bucketqueue;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;


fn main() {
    let mut arecord = csvreader::csvcon().unwrap();



    let mut data : Vec<Vec<Keyval>> = vec![Vec::new();500];

    
    let mut max:f64= 0.0;
    //was orginally just a key value pair but, for the exculsion list to work
    // an id to idenitfy what index pair of rocks is in the this key value was added 
    #[derive(Debug,Clone,Copy)]
    pub struct Keyval {
        pub key:OrderedFloat<f64>, //the time the pair collides at
        pub val:csvreader::Rec, //all information p1,p2,p1x,p2x .. etc
        pub id:(f64,f64)        //p1,p2
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
        data[index].push(Keyval{key:OrderedFloat(poppy.time),val:poppy,id:(poppy.p1,poppy.p2)});

    }
    let now = Instant::now();

    let mut heap: BinaryHeap<Keyval> = BinaryHeap::new(); //intialize the queue

    // j and i got flipped from standard notation i nested in j instead of the the other way around
    for j in 0..data.len() {
        let mut exc = Vec::new(); //each bif loop has a new exculsion list as the heap is emptied each j iteration
        exc.push(data[j][0].id);
        heap.push(data[j][0]);
        for i in 0..data[j].len() {
            // if the id of the item to be added is not already in the exculsion list 
            // it is added to the heap then it's id is added to the exclusion list
            if  !exc.contains(&data[j][i].id)  { 

                exc.push(data[j][i].id);
                heap.push(data[j][i]);
            } else if !heap.is_empty()  {
                while  !heap.is_empty() && heap.peek().unwrap().id != data[j][i].id {
                    let y = heap.pop().unwrap().id;
                    exc.retain(|x| x != &y ); //still has to remove the ids that are popped from the heap before the repeat id
                }
            }
            
        }
        //clears the queue after everything in a big timestep has been processed 
        while !heap.is_empty() { 
            heap.pop();
            //exc.pop();
            
        }
    }


    let elapsed = now.elapsed();
    println!("Binary Heap Elapsed: {:.2?}", elapsed);

    let now1 = Instant::now();

    let delta:f64 = 2.0*PI*1E-4 - 2.0*PI*1E-5;


    let mut heap1: bucketqueue::Bqueue<Keyval> = bucketqueue::Bqueue::new(((max/delta).ceil()+1.0) as usize,delta); //intialize the Bucket queue

    // j and i got flipped 
    for j in 0..data.len() {
        let mut exc1 = Vec::new(); //each j loop has a new exculsion list as the heap is emptied each j iteration
        exc1.push(data[j][0].id);
        heap1.push(data[j][0],data[j][0].val.time);
        for i in 0..data[j].len() {
            // if the id of the item to be added is not already in the exculsion list 
            // it is added to the heap then it's id is added to the exclusion list
            if  !exc1.contains(&data[j][i].id)  { 

                exc1.push(data[j][i].id);
                heap1.push(data[j][i], data[j][i].val.time);
            } else if !heap1.is_empty()  {
                while  !heap1.is_empty() && heap1.peek().unwrap().id != data[j][i].id {
                    let y = heap1.pop().unwrap().id;
                    exc1.retain(|x| x != &y ); //still has to remove the ids that are popped from the heap before the repeat id
                }
            }
            
        }
        //clears the queue after everything in a big timestep has been processed 
        while !heap1.is_empty() { 
            heap1.pop();
            //exc.pop();
            
        }
    }


    let elapsed1 = now1.elapsed();
    println!("Bucket Queue Elapsed: {:.2?}", elapsed1);
    //println!("{}",data[100].len());
    //println!("first p1: {}",arecord[0].p1);



}
