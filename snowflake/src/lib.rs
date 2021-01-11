#![feature(test)]

use std::time::{SystemTime, Instant, UNIX_EPOCH};
use std::sync::{Arc, Mutex};

const TW_EPOCH: i64 = 1288834974657;
const WORKER_ID_BITS: usize = 5;
const DATACENTER_ID_BITS: usize = 5;
const MAX_WORKER_ID: i64 = 0x1F;
const MAX_DATACENTER_ID: i64 = 0x1F;
const SEQUENCE_BITS: usize = 21;
const WORKER_ID_SHIFT: usize = SEQUENCE_BITS;
const DATACENTER_ID_LEFT_SHIFT: usize = SEQUENCE_BITS + WORKER_ID_BITS;
const TIMESTAMP_LEFT_SHIFT: usize = SEQUENCE_BITS + WORKER_ID_SHIFT + DATACENTER_ID_BITS;
const SEQUENCE_MASK: i64 = -1 ^ (-1 << SEQUENCE_BITS);
const MAX_NEXT_IDS_NUM: usize = 100;

#[derive(Debug, Clone)]
pub struct IDWork {
    work_id: i64,
    tw_epoch: i64,
    datacenter_id: i64,
    last_timestamp: i64,
    sequence: i64,
}

impl IDWork {
    pub fn new(work_id: i64, datacenter_id: i64, tw_epoch: i64) -> Self {
        if work_id > MAX_WORKER_ID || work_id < 0 {
            panic!("work id cannot be negative or gt {}", MAX_WORKER_ID);
        }
        if datacenter_id > MAX_DATACENTER_ID || datacenter_id < 0 {
            panic!("datacenter id cannot be negative or gt {}", MAX_DATACENTER_ID);
        }
        IDWork {
            work_id,
            tw_epoch,
            datacenter_id,
            last_timestamp: -1,
            sequence: 0,
        }
    }

    pub fn next_id(&mut self) -> Result<i64, &'static str> {
        self.inner_next_id()
    }

    pub fn next_ids(&mut self, num: usize) -> Result<Vec<i64>, &'static str> {
        if num > MAX_NEXT_IDS_NUM || num <= 0 {
            panic!("next_ids num must be lt MAX_NEXT_IDS_NUM and gt zero");
        }
        let mut ids = Vec::with_capacity(num);
        for i in 0..num {
            ids.push(self.inner_next_id()?);
        }
        Ok(ids)
    }

    fn inner_next_id(&mut self) -> Result<i64, &'static str> {
        let mut timestamp = Self::time_gen();
        if timestamp < self.last_timestamp {
            return Err("clock moved backwards.  refusing to generate id");
        }
        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & SEQUENCE_MASK;
            if self.sequence == 0 {
                timestamp = Self::til_next_millis(self.last_timestamp);
            }
        } else {
            self.sequence = 0;
        }
        self.last_timestamp = timestamp;
        println!("{:?}", self);
        let time_bits = (timestamp - self.tw_epoch) << TIMESTAMP_LEFT_SHIFT;
        let data_bits = self.datacenter_id << DATACENTER_ID_LEFT_SHIFT;
        let work_bits = self.work_id << WORKER_ID_SHIFT;
        println!("time_bits: {}, data_bits: {}, work_id: {}, sequence: {}", time_bits, data_bits, work_bits, self.sequence);
        Ok(time_bits | data_bits | work_bits | self.sequence)
    }

    fn time_gen() -> i64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
    }

    fn til_next_millis(last_timestamp: i64) -> i64 {
        let mut timestamp = Self::time_gen();
        while timestamp <= last_timestamp {
            timestamp = Self::time_gen();
        }
        timestamp
    }
}


#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;
    use crate::IDWork;
    use std::sync::{Arc, Mutex};
    use std::thread::{spawn, sleep, JoinHandle};
    use std::time::Duration;

    #[test]
    fn it_works() {
        let work = Arc::new(Mutex::new(IDWork::new(1, 1, 1288834974657)));
        let mut res: Arc<Mutex<Vec<i64>>> = Arc::new(Mutex::new(Vec::new()));
        let mut computation: Vec<JoinHandle<_>> = Vec::new();
        for i in 0..10 {
            let res = res.clone();
            let work = work.clone();
            computation.push(spawn(move || {
                for i in 0..100 {
                    match work.lock().unwrap().next_id() {
                        Ok(id) => res.lock().unwrap().push(id),
                        _ => {}
                    }
                }
            }));
        }
        for join in computation {
            join.join().unwrap();
        }
        res.lock().unwrap().sort();
        println!("{:?}", res.lock().unwrap());
        println!("{:?}", res.lock().unwrap().len());
    }

    #[bench]
    fn bench_id(b: &mut Bencher) {
        let mut work = IDWork::new(1, 1, 1288834974657);
        b.iter(|| {
            work.next_id();
        })
    }

    #[test]
    fn generate() {
        let mut work = IDWork::new(1, 1, 1288834974657);
        for i in 0..10 {
            println!("{}", work.next_id().unwrap());
        }
    }
}
