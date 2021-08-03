/**
 * title: A Persistent Stack
 * 
 * list1 = A -> B -> C -> D
 * list2 = tail(list1) = B -> C -> D
 * list3 = push(list2, X) = X -> B -> C -> D
 * 
 * list1 -> A ---+
 *               |
 *               v
 * list2 ------> B -> C -> D
 *               ^
 *               |
 * list3 -> X ---+
 */

use std::rc::Rc;
// basic -------------------------------------------------------------------
pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }
    pub fn prepend(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node { elem, next: self.head.clone() })) }
    }
    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

// Iter ------------------------------------------------------------------
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}
impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}
// drop ------------------------------------------------------------
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break
            }
        }
    }
}


#[cfg(Test)]
mod test {
    use super::List;
    #[test]
    fn basics_test() {
        let list = List::new();
        list.prepend(1).prepend(2).prepend(3);
        
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
    fn iter_test() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

}
