pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut cur = &mut dummy;

        while let Some(mut node) = cur.next.take() {
            if node.val == val {
                cur.next = node.next.take();
            } else {
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }
        }

        dummy.next.take()
    }

    // ===== 여기에 LeetCode 터미널 코드 붙여넣기 =====
    // pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> { ... }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // ===== LeetCode Examples 여기에 추가 =====
    // #[test] fn example_1() { assert_eq!(Solution::함수명(args), expected); }
}
