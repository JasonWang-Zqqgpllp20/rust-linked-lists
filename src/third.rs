/* A Persistent Singly-Linked Stack */
/* 
    This means we are going to move from single ownership to shared ownership 
    by writing a persistent immutable singly-linked list with **Rc**.
*/

/* 
    We can use **Arc** instead of Rc to make it thread safety.
    Almost every type is Send and Sync. Send is safe to move and Sync is safe to share. If T is Sync, &T is Send.
    Cells only works in a single-threaded while Rc is, locks work in a multi-threaded context while Arc is.
*/
use std::rc::Rc;
pub struct List<T> {
    head: Link<T>,
}
type Link<T> = Option<Rc<Node<T>>>;
struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn prepend(&self, elem: T) -> List<T> { // the same as push() in second.rs
        List { head: Some(Rc::new(Node {
            elem: elem,
            next: self.head.clone(),
        }))}
    }

    pub fn tail(&self) -> List<T> { // the same as pop() in second.rs
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone())
            // https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then
            // and_then() returns None if option is None, otherwise calls the clousure and returns the result
        }
    }

    pub fn head(&self) -> Option<&T> {  // the same as peek() in second.rs
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

// can't implement IntoIter or IterMut for this type
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}



#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}