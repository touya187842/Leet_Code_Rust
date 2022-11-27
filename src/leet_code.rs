use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }

  pub fn from_slice(vals: &[Option<i32>]) -> Option<Rc<RefCell<Self>>> {
    Self::from_slice_inner(vals, 0)
  }

  fn from_slice_inner(vals: &[Option<i32>], index: usize) -> Option<Rc<RefCell<Self>>> {
    if index < vals.len() && vals[index].is_some() {
      return Some(Rc::new(RefCell::new(TreeNode {
        val: vals[index].unwrap(), 
        left: Self::from_slice_inner(vals, (index << 1) + 1),
        right: Self::from_slice_inner(vals, (index << 1) + 2) 
      })))
    }
    None
  }

  pub fn to_vec(&self) -> Vec<Option<i32>> {
    let mut vec: Vec<Option<i32>> = Vec::new();
    vec.push(Some(self.val));
    Self::push_to_vec(&self.left, &mut vec, 1);
    Self::push_to_vec(&self.right, &mut vec, 2);
    vec
  }

  fn push_to_vec(node: &Option<Rc<RefCell<Self>>>, vec: &mut Vec<Option<i32>>, index: usize) {
    if node.is_none() {
      return;
    }

    while vec.len() <= index {
        vec.push(None);
    }

    let inner = node.as_ref().unwrap().borrow();
    vec[index] = Some(inner.val);
    Self::push_to_vec(&inner.left, vec, (index << 1) + 1);
    Self::push_to_vec(&inner.right, vec, (index << 1) + 2);
  }
}