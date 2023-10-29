use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec;
use alloc::vec::Vec;

/// a struct records resource allocation
pub struct DeadLockDetector {
    /// a Mutex/Semaphore is an item in available vector
    available: Vec<Option<i32>>,
    /// work
    work: Vec<Option<i32>>,
    /// need matrix
    need: Vec<Option<BTreeMap<usize, i32>>>,
    /// inner vector stands for tasks
    allocation: Vec<Option<BTreeMap<usize, i32>>>,
}

impl DeadLockDetector {

    /// create an empty detector
    pub fn new() -> DeadLockDetector {
        Self {
            available: Vec::new(),
            work: Vec::new(),
            need: Vec::new(),
            allocation: Vec::new(),
        }
    }

    /// insert a new resource into detector
    pub fn insert_new_resource(&mut self, resource_id: usize, available: i32) {
        assert!(resource_id <= self.available.len());
        if resource_id == self.available.len() {
            self.available.push(Some(available));
            self.work.push(Some(available));
            self.need.push(Some(BTreeMap::new()));
            self.allocation.push(Some(BTreeMap::new()));
        } else {
            self.available[resource_id] = Some(available);
            self.work[resource_id] = Some(available);
            self.need[resource_id] = Some(BTreeMap::new());
            self.allocation[resource_id] = Some(BTreeMap::new());
        }
    }

    /// try allocate a resource
    pub fn try_allocate(&mut self, task_id: usize, resource_id: usize, count: i32) -> bool {
        assert!(self.work[resource_id].is_some());
        let mut work = self.work.clone();
        let mut need = self.need.clone();
        let allocation = self.allocation.clone();
        *need[resource_id].as_mut().unwrap().entry(task_id).or_insert(0) += count;
        let mut task_ids = self.extract_ids(&need, &allocation);

        while !task_ids.is_empty() {
            let mut found: Vec<usize> = Vec::new();
            for &task in &task_ids {
                let mut ok = true;
                for i in 0..work.len() {
                    if work[i].is_none() {
                        continue;
                    }
                    let old_work = work[i].unwrap();
                    let now_need = need[i].as_ref().unwrap().get(&task).map_or(0, |v| *v);
                    if now_need > old_work {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    found.push(task);
                    for i in 0..work.len() {
                        if work[i].is_none() {
                            continue;
                        }
                        let old_work = work[i].unwrap();
                        let now_allocation = allocation[i].as_ref().unwrap().get(&task).map_or(0, |v| *v);
                        work[i] = Some(old_work + now_allocation);
                    }
                    break
                }
            }
            if found.is_empty() {
                break;
            } else {
                task_ids.remove(&found[0]);
            }
        }
        let ok = task_ids.is_empty();
        if ok {
            self.do_allocate(task_id, resource_id, count);
        }
        ok
    }

    fn do_allocate(&mut self, task_id: usize, resource_id: usize, count: i32) {
        let old_work = self.work[resource_id].unwrap();
        *self.need[resource_id].as_mut().unwrap().entry(task_id).or_insert(0) += count;
        let now_need = *self.need[resource_id].as_ref().unwrap().get(&task_id).unwrap();
        let alloc = old_work.min(now_need);
        self.work[resource_id] = Some(old_work - alloc);
        self.need[resource_id].as_mut().unwrap().insert(task_id, now_need - alloc);
        *self.allocation[resource_id].as_mut().unwrap().entry(task_id).or_insert(0) += alloc;
    }

    fn extract_ids(&self, need: &Vec<Option<BTreeMap<usize, i32>>>, allocation: &Vec<Option<BTreeMap<usize, i32>>>) -> BTreeSet<usize> {
        let mut task_ids = BTreeSet::new();
        let data = vec![need, allocation];
        for v in data {
            for m in v {
                match m {
                    Some(map) => {
                        for &id in map.keys() {
                            task_ids.insert(id);
                        }
                    },
                    None => {},
                }
            }
        }
        task_ids
    }
}