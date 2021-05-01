pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem,
            // take moves the value out of the option and replaces it with None
            next: self.head.take(),
        });
        //set this node to be the new head
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

//implement Drop (destructor)
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        //take the head
        let mut current_link = self.head.take();
        //while let -- while the patten matches
        while let Some(mut boxed_node) = current_link {
            //replace every link with empty, then let the node go out of scope
            current_link = boxed_node.next.take();
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
    #[test]
    fn peek() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
}
