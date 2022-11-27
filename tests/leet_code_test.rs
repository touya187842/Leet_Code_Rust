use leet_code_rust::leet_code::ListNode;

#[test]
pub fn build_list_node_from_slice_and_to_vec() {
    let slice1 = &[1, 5, 4, 3, 2];
    assert_eq!(ListNode::from_slice(slice1).map_or(Vec::new(),|x| x.to_vec()), slice1);
    let slice2: &[i32] = &[];
    assert_eq!(ListNode::from_slice(slice2).map_or(Vec::new(),|x| x.to_vec()), slice2);
}
