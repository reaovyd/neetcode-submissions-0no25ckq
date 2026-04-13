use std::collections::{HashMap, HashSet};

struct CountSquares {
    map_x: HashMap<i32, HashSet<Vec<i32>>>,
    map_y: HashMap<i32, HashSet<Vec<i32>>>,
    map_co: HashMap<Vec<i32>, i32>,
}

impl CountSquares {
    fn new() -> Self {
        Self { map_x: HashMap::new(), map_y: HashMap::new(), map_co: HashMap::new() }
    }
    fn add(&mut self, point: Vec<i32>) {
        let point_x = point.clone();
        let point_y = point.clone();
        self.map_x.entry(point_x[0]).or_default().insert(point_x);
        self.map_y.entry(point_y[1]).or_default().insert(point_y);
        *self.map_co.entry(point).or_insert(0) += 1;
    }
    fn count(&self, point: Vec<i32>) -> i32 {
        let x = point[0];
        let y = point[1];
        let mut res = 0;
        if let Some(x_iter) = self.map_x.get(&x) {
            for point_x in x_iter {
                let dist_squared_x = _get_dist_squared(point_x, &point);
                if dist_squared_x == 0 {
                    continue;
                }
                if let Some(y_iter) = self.map_y.get(&y) {
                    for point_y in y_iter {
                        let dist_squared_y = _get_dist_squared(point_y, &point);
                        if dist_squared_y == 0 {
                            continue;
                        }
                        if dist_squared_x != dist_squared_y {
                            continue;
                        }
                        let xy = vec![point_y[0], point_x[1]];
                        if 2 * dist_squared_x != _get_dist_squared(&xy, &point) {
                            continue;
                        }
                        let v1 = *self.map_co.get(point_x).unwrap();
                        let v2 = *self.map_co.get(point_y).unwrap();
                        if let Some(v3) = self.map_co.get(&xy) {
                            res += v1 * v2 * v3
                        }
                    }
                }
            }
        }
        res
    }
}
fn _get_dist_squared(point_x: &[i32], point_y: &[i32]) -> i32 {
    let p1 = point_y[0] - point_x[0];
    let p2 = point_y[1] - point_x[1];
    let res = (p1 * p1) + (p2 * p2);
    res
}
