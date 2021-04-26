use std::mem;

pub struct List {
    head: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    //mutable borrows the list
    pub fn push(&mut self, elem: i32) {
        // make a new node in a box (the new head)
        let node = Box::new(Node {
            elem,
            // set the next node to be the old head, setting head to empty
            // mem::replace swaps puts src where dest was, returning dest
            next: mem::replace(&mut self.head, Link::Empty),
        });
        //set this node to be the new head
        self.head = Link::More(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        //replace the head with empty, pattern match on it
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None, // if empty, return None
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem) //if not, set the head to the next node and return the element
            }
        }
    }
}

//implement Drop (destructor)
impl Drop for List {
    fn drop(&mut self) {
        //get the head again, setting old head to empty
        let mut current_link = mem::replace(&mut self.head, Link::Empty);
        //while let -- while the patten matches
        while let Link::More(mut boxed_node) = current_link {
            //replace every link with empty, then let the node go out of scope
            current_link = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}
#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        //check popping from an empty list returns None
        assert_eq!(list.pop(), None);

        //add to the list
        list.push(1);
        list.push(2);
        list.push(3);

        //check pop works
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        //push a little more
        list.push(4);
        list.push(5);

        //remove everything
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
