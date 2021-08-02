
/**
 * title: A Bad Stack 
 */

/**
 * 用这种方式会有标签带来的额外开销, 属于函数式编程语言的默认方法
 * 所以用下面的C-like结构体形式占用空间更小,
 * 而且单个节点能承载更多内容
 */
// #[derive(Debug)]
// pub enum List<T> {
//     Empty,
//     Elem(T, Box<List<T>>),
// }


// 2.1. Layout
#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

// 2.2. New
impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty,
        }
    }
}

// 2.3. OwnerShip

// - self - Value
// - &mut self - mutable reference
// - &self - shared reference

// 2.4. Push
use std::mem;
impl List {
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node { 
            elem, 
            next: mem::replace(&mut self.head, Link::Empty), 
        });
        self.head = Link::More(new_node)
    }
}

// 2.5. Pop 

// 返回 unimplemented!()
// impl List {
//     pub fn pop(&mut self) -> Option<i32> {
//         match &self.head {
//             Link::Empty => {
//                 // TODO
//             },
//             Link::More(node) => {
//                 // TODO
//             }
//         }
//         unimplemented!()
//     }
// }

impl List {
    // pub fn pop(&mut self) -> Option<i32> {
    //     let result;
    //     match mem::replace(&mut self.head, Link::Empty) {
    //         Link::Empty => {
    //             result = None;
    //         }
    //         Link::More(node) => {
    //             result = Some(node.elem);
    //             self.head = node.next;
    //         }
    //     };
    //     result
    // }

    // 相比于上一个更常用而且更简洁的写法
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

// 2.6. Testing
#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        // TODO
        use super::*;
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

        // Check drop
        list.push(1);
        list.push(2);
        list.push(3);

    }
}

// 2.7. Drop

/*
impl Drop for List {
    fn drop(&mut self) {
        self.head.drop();
    }
}
impl Drop for Link {
    fn drop(&mut self) {
        match self {
            Link::Empty => {},
            Link::More(ref mut boxed_node) => {
                boxed_node.drop();
            }
        }
    }
}

We can't drop the contents of the Box after deallocating, so there's no way to drop in a tail-recursive manner! 
Instead we're going to have to manually write an iterative drop for List that hoists nodes out of their boxes.

impl Drop for Box<Node> {
    fn drop(&mut self) {
        self.ptr.drop();   // uh oh, not tail recursive!
        deallocate(self.ptr);
    }
}
impl Drop for Node {
    fn drop(&mut self) {
        self.next.drop();
    }
}
*/
impl Drop for List {
    fn drop(&mut self) {
        println!("开始drop");
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            println!("{}", boxed_node.elem);
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}