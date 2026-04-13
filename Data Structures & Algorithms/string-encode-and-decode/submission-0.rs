impl Solution {
    pub fn encode(mut strs: Vec<String>) -> String {
        let mut enc = String::new();
        for string in strs.iter_mut() {
            string.push('\n'); 
            let mut len = string.len().to_string(); 
            len.push('_');
            enc.push_str(&len);
            enc.push_str(&string);
        }
        enc
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut chars = s.chars();
        let mut res = Vec::new();
        let mut st = String::new();
        let mut is_find_num = true;
        let mut cur_len = 0;
        for c in chars {
            if is_find_num {
                if c == '_' {
                    cur_len = st.parse::<usize>().unwrap();
                    is_find_num = false;
                    st.clear();
                } else {
                    st.push(c);
                }
            } else {
                st.push(c);
                cur_len -= 1;
                if cur_len == 0 {
                    res.push(st.clone());
                    is_find_num = true;
                    st.clear();
                }
            }
        }
        for s in res.iter_mut(){
            s.pop();
        }
        res
    }
}
