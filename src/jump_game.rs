#[allow(dead_code)]
pub mod solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let target = len - 1;
        let mut max:usize = 0;
        let mut i:usize = 0;
        
        while i < len && i <= max {
            let reach = nums[i] as usize + i;
            if reach > max {
                max = reach;
            }
            if max >= target {
                return true;
            }
            i+=1;
        }
        false
    }
}
