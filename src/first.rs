/* A Bad Single-Linked Stack */
use std::mem;

/* 
    1.但是这样不能避免额外的junk存储开销，也不能有统一的内存分配(enum的两种名称大小不一样)
      还不能利用Rust的enum中特有的null pointer优化（一个字段是A，另一个字段B中只包含了一个非空指针），形如：
            enum Foo { A, B(ContainsANonNullPtr) }
*/
// pub enum List {
//     Empty,
//     Elem(i32, Box<List>)
// }

/* 
    2.尝试设计成头指针，想让List变public而让Node变private，但是Rust的enum要求起内部必须全是共有的
*/
// struct Node {
//     elem: i32,
//     next: List,
// }

// pub enum List {
//     Empty,
//     More(Box<Node>),
// }

/* 
    3.接上条，只能把enum改成一个嵌套enum的struct，让List编程public
*/
pub struct List {
    head: Link,
}
enum Link {     // 在More字段中只有一个非空指针，符合Rust enum中的null pointer优化
    Empty,
    More(Box<Node>)
}
struct Node {
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty), // self.head需要copy，只能用mem::replace
            // https://doc.rust-lang.org/std/mem/fn.replace.html
        };

        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {   // 不能直接把node.elem移出去
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
        // unimplemented!()
    }
}

impl Drop for List {    // Box不满足tail-recursive形式的drop，只能手动为List实现iterative drop
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty); // 为每个next置空
        }
    }
}

mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();
        
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}