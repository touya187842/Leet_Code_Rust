#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val: val
    }
  }

  pub fn from_slice(vals: &[i32]) -> Option<Box<Self>> {
    if vals.len() > 0 {
      return Some(Box::new(ListNode { val: vals[0], next: Self::from_slice(&vals[1..]) }))
    }
    None
  }

  pub fn to_vec(&self) -> Vec<i32>{
    let mut vec: Vec<i32> = Vec::new();
    vec.push(self.val);
    let mut next = &self.next;
    while next.is_some() {
      let node = next.as_ref().unwrap();
      vec.push(node.val);
      next = &node.next;
    }
    vec
  }
}