// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = None;
        let cur = &mut ret;
        while let Some(mut maybe_head) = head {
            let mut next = maybe_head.next.take();
            maybe_head.next = cur.take();
            cur.replace(maybe_head);
            head = next.take();
        }
        cur.take()
    }
}

