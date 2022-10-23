use leet_code_rust::jump_game::solution;

#[test]
pub fn solution_can_be_accepted() {
    assert_eq!(solution::can_jump(vec![2,3,1,1,4]), true);
    assert_eq!(solution::can_jump(vec![3,2,1,0,4]), false);
    assert_eq!(solution::can_jump(vec![2,5,0,0]), true);
    assert_eq!(solution::can_jump(vec![2,0,0]), true);
    assert_eq!(solution::can_jump(vec![0]), true);
}
