// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut vec = vec![];
        let mut carry = 0;
        let mut p = l1;
        let mut q = l2;
        while p != None || q != None {
            if p != None {
                let tmp = p.take().unwrap();
                carry += tmp.val;
                p = tmp.next;
            }
            if q != None {
                let tmp = q.take().unwrap();
                carry += tmp.val;
                q = tmp.next;
            }
            vec.push(carry % 10);
            carry /= 10;
        }

        if carry != 0 {
            vec.push(carry);
        }

        let mut head = None;
        for val in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(*val));
            node.next = head.take();
            head = Some(node);
        }
        head
    }
}
/*
这是官方题解，时间复杂度和另一个解法一样，对于有C语言等其他语言基础的人来说，这种解法应该更容易理解。
*/
