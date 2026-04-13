use std::collections::HashMap;
struct TimeMap {
    map: HashMap<String, Vec<(String, i32)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self { map: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_insert(Vec::new()).push((value, timestamp));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(lst) = self.map.get(&key) {
            let idx = find_max_smallest_number(lst, timestamp);
            if idx < 0 {
                return String::new();
            }
            if let Some(val) = lst.get(idx as usize) {
                val.0.clone()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }
}

fn find_max_smallest_number(lst: &[(String, i32)], timestamp: i32) -> i32 {
    let mut i = 0;
    let mut j = lst.len() as i32;
    let mut res = -1;
    while i < j {
        let m = (i + j) >> 1;

        let ts = lst[m as usize].1;
        if timestamp >= ts {
            res = m;
            i = m + 1;
        } else {
            j = m;
        }
    }
    res
}
