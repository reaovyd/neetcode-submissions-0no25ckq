use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for (idx, str) in strs.iter().enumerate() {
            let mut st = str.chars().collect::<Vec<_>>();
            st.sort();
            map.entry(
                String::from_utf8(st.into_iter().map(|c| c as u8).collect::<Vec<u8>>()).unwrap(),
            )
            .or_insert(vec![])
            .push(idx);
        }
        map.into_values()
            .map(|vec| vec.into_iter().map(|idx| strs[idx].clone()).collect::<Vec<String>>())
            .collect()
    }
}
