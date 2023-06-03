/// https://leetcode.com/problems/add-two-numbers

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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
         // match both node
         match (l1,l2){
             (None,None)=>None,
             ((Some(n),None)|(None,Some(n)))=>Some(n),
             (Some(n1),Some(n2))=>{
                 let ans = n1.val+n2.val;
                 if ans<10{
                    Some(
                     Box::new(
                         ListNode{
                             val:ans,
                             next:Solution::add_two_numbers(n1.next,n2.next)
                         }
                    
                     )
                 )
                 }
                 else{
                     let carry = Some(
                     Box::new(
                         ListNode::new(1)
                     ));
                     Some(
                         Box::new(
                             ListNode{
                                 val:ans-10,
                                 next:Solution::add_two_numbers(Solution::add_two_numbers(n1.next,carry),n2.next)
                             }
                         )
                     )
                 
                 }
                 

             }
         }
    }
}
