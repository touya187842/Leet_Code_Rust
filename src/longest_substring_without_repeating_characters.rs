pub mod solution {
    use std::cmp;
    use std::collections::HashMap;
    pub  fn length_of_longest_substring(s: String) -> i32 {
        let vec = s.into_bytes();
        let mut max: usize = 0;
        let mut lookup: HashMap<u8, usize> = HashMap::new();

        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < vec.len() {
            let pre = lookup.insert(vec[i], i+1);
            if pre.is_some() {
                j = cmp::max(j, pre.unwrap());
            }
            max = cmp::max(max, i-j+1);
            i+=1;
        }
        max as i32
    }
}
