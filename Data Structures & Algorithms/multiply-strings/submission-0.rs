impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let chars1 = num1.chars().rev();
        let mut cur_vec = vec![0; 0];
        for (zeroes, c1) in chars1.enumerate() {
            let mut vec = vec![0; zeroes];
            let num1 = (c1 as u8) - b'0';
            let chars2 = num2.chars().rev();
            for c2 in chars2 {
                let num2 = (c2 as u8) - b'0';
                let res = num1 * num2;
                vec.push(res as i32);
            }
            add_vecs(&mut cur_vec, &mut vec);
        }
        let mut i = 0;
        let n = cur_vec.len();
        cur_vec.push(0);
        while i < n {
            let num = cur_vec[i] % 10;
            let num2 = cur_vec[i] / 10;
            cur_vec[i] = num;
            cur_vec[i + 1] += num2;
            i += 1;
        }
        while *cur_vec.last().unwrap() == 0 && cur_vec.len() > 1 {
            cur_vec.pop();
        }

        let mut res = String::new();
        for num in cur_vec.into_iter().rev() {
            res.push_str(&num.to_string());
        }
        res
    }
}

fn add_vecs(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>) {
    let mut n = vec1.len();
    let mut m = vec2.len();
    if n < m {
        while n < m {
            vec1.push(0);
            n += 1;
        }
    } else {
        while m < n {
            vec2.push(0);
            m += 1;
        }
    }
    let mut i = 0;
    while i < n {
        vec1[i] += vec2[i];
        i += 1;
    }
}
