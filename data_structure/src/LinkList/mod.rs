/// rust linklist
/// rust实现链表

use std::rc::Rc;
use std::cell::RefCell;
type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;
// 链表节点
pub struct ListNode<T> {
    pub val: T,
    pub next: Link<T>,
}
// 链表
pub struct LinkedList<T> {
    pub head: Link<T>,
    pub tail: Link<T>,
    pub length: usize,
}
// 创建链表节点
impl <T> ListNode<T> {
    pub fn new(val: T) -> Link<T> {
       Rc::new(RefCell::new(ListNode {
           val,
           next: None,
       })
    }
}