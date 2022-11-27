use leet_code_rust::leet_code::ListNode;
use leet_code_rust::leet_code::TreeNode;

#[test]
pub fn build_list_node_from_slice_and_to_vec() {
    let slice1 = &[1, 5, 4, 3, 2];
    assert_eq!(ListNode::from_slice(slice1).map_or(Vec::new(),|x| x.to_vec()), slice1);
    let slice2: &[i32] = &[];
    assert_eq!(ListNode::from_slice(slice2).map_or(Vec::new(),|x| x.to_vec()), slice2);
}

#[test]
pub fn build_tree_from_slice_and_to_vec() {
    let slice1: &[Option<i32>] = &[Some(1), None, Some(6), None, None, Some(3)];
    assert_eq!(TreeNode::from_slice(slice1).map_or(Vec::new(),|x| x.borrow().to_vec()), slice1);
    let slice2: &[Option<i32>] = &[];
    assert_eq!(TreeNode::from_slice(slice2).map_or(Vec::new(),|x| x.borrow().to_vec()), slice2);
    let slice3: &[Option<i32>] = &[Some(3), None, None, Some(2)];
    assert_eq!(TreeNode::from_slice(slice3).map_or(Vec::new(),|x| x.borrow().to_vec()), slice3[..1]);
}
