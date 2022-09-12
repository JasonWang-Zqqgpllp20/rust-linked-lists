/* A bad but safe doubly-linked deque */
use std::rc::Rc;
use std::cell::RefCell;
/*
    There are two Cell types in Rust: Cell<T> and RefCell<T>, both of them provide **interior mutability**.
    But Cell<T> only compatible with types that impl Copy trait, for other types, one must use RefCell<T>.
    RefCell<T> implement dynamic borrowing which are tracked at runtime rather than complite time.
    RefCell<T> can mutate a imutable variant which wrapped by Rc<RefCell<>>, for example:
        let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(Hashmap::new()));
        shard_map.borrow_mut().insert("zz", 123);
    Rc is for single-threaded scenarios, and we can use Mutex<RefCell<T>> for multi-threaded situation.
*/

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}
impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            // old_head.into_inner().elem // Cannot take the value out of Rc
            // Rc::try_unwrap(old_head).unwrap().into_inner().elem // Result::unwrap() is invalid because <T> is not Debug.
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem // use ok() to transfer Result to Option
        })
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}
impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
       

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}