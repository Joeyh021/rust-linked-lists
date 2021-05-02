use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    // node constructor
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
    //nodes always are pointed to twice
    pub fn push_front(&mut self, elem: T) -> () {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                //non-empty list
                //set old heads prev to be new head
                old_head.borrow_mut().prev = Some(new_head.clone());
                //set new heads next to be old head
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                //clone new head and make it the tail
                self.tail = Some(new_head.clone());
                //move new head into the head
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // need to take the old head, ensuring it's -2
        self.head.take().map(|old_head| {
            // -1 old
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // -1 new
                    // not emptying list
                    new_head.borrow_mut().prev.take(); // -1 old
                    self.head = Some(new_head); // +1 new
                                                // total: -2 old, +0 new
                }
                None => {
                    // emptying list
                    self.tail.take(); // -1 old
                                      // total: -2 old, (no new)
                }
            }
            old_head.elem
        })
    }
}