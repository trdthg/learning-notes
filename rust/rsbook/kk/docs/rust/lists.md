# Too-Many-Lists

## A Bad Stack 

### 引例
/**  
 * 用这种方式会有标签带来的额外开销, 属于函数式编程语言的默认方法  
 * 所以用下面的C-like结构体形式占用空间更小,  
 * 而且单个节点能承载更多内容  
 */  
```rust
#[derive(Debug)]
pub enum List<T> {
    Empty,
    Elem(T, Box<List<T>>),
}
```

### 完整案例
```rust
// 2.1. Layout
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
use std::mem;
impl List {
    // 2.2. New
    pub fn new() -> Self {
        List {
            head: Link::Empty,
        }
    }
    // 2.4. Push
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node { 
            elem, 
            next: mem::replace(&mut self.head, Link::Empty), 
        });
        self.head = Link::More(new_node)
    }
    // 2.5. Pop 
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
```

### Drop的手动编写  
尝试根据尾递归写出drop函数, 失败
```rust
// 2.7. Drop
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
```
::: danger
>We can't drop the contents of the Box after deallocating, so there's no way to drop in a tail-recursive manner! 
>Instead we're going to have to manually write an iterative drop for List that hoists nodes out of their boxes.
:::
So, the next is the real way to write drop ourselfs
```rust
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
```

### unimplemented!
::: warning
```rust
// 返回 unimplemented!()
impl List {
    pub fn pop(&mut self) -> Option<i32> {
        match &self.head {
            Link::Empty => {
                // TODO
            },
            Link::More(node) => {
                // TODO
            }
        }
        unimplemented!()
    }
}
```
:::

## An Ok Singly-Linked Stack

### 优化目标
1. 3.1: Option 简化语法
 * use Option to replace our own Enum
 * use Option.take() to replace mem::replace(&self, None)
 * use option.take().map(|elem {}|) to replace match option { None => None, Some(x) => Some(y) }
2. 3.2 Generic 通用性
 * just use T to replace i32
3. 3.3 Peek(偷看) 实现peek
 * 
 * 注意3者的区别
 * self.head.take()    -> self       -> Option(T)
 * self.head.as_ref()  -> &self      -> Option(&T)
 * self.head.as_mut()  -> &mut self  -> Option(&mut T)
4. 3.4 - 3.6 三种迭代器
 * IntoIter - T
 * IterMut - &mut T
 * Iter - &T   
> map(): Maps an Option<T> to Option<U> by applying a function to a contained value.
> as_ref()->&self: Converts from &Option<T> to Option<&T>.
> as_mut()->&mut self: Converts from &mut Option<T> to Option<&mut T>.
> take()->&mut self: Takes the value out of the option, leaving a [None] in its place

### Option & Generic
```rust
use std::mem;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    // 2.2. New
    pub fn new() -> Self {
        List { head: None }
    }
    // 2.4. Push
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node { 
            elem, 
            // next: mem::replace(&mut self.head, None), 
            next: self.head.take(), 
        });
        self.head = Some(new_node);
    }
    // 2.5. Pop 
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem  //  node.elem not need to be wrapped by Some()
        })
    }
    
}
```

### Peek
```rust
impl<T> List<T> {
    /*
    impl<T> Option<T> {
        pub fn as_ref(&self) -> Option<&T>;
    }
    */

    pub fn peek(&self) -> Option<&T> {

        // Converts from &Option<T> to Option<&T>.

        // self             -> &List  
        // self.head        -> &Option< Box<Node<T>>>
        // self.head.as_ref ->  Option<&Box<Node<T>>>
        // map(node)        ->         &Box<Node<T>>
        // node.elem        ->                   T
        // &node.elem       ->         &         T
        // map->&node.elem  ->  Option<&         T  >
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // Converts from &mut Option<T> to Option<&mut T>.

        // self             -> &mut List  
        // self.head        -> &mut Option<     Box<Node<T>>>
        // self.head.as_mut ->      Option<&mut Box<Node<T>>>
        // map(node)        ->             &mut Box<Node<T>>
        // node.elem        ->                           T
        // &mut node.elem   ->             &mut          T
        // map->&node.elem  ->      Option<&mut          T  >
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
    pub fn peek_(&mut self) -> Option<T> {
        // warning(not sure): mut_only_in_this_fu_but_only_read_after_read
        // Takes the value out of the option, leaving a [None] in its place

        // self             -> &mut List  
        // self.head        -> &mut Option<     Box<Node<T>>>
        // self.head.take   ->      Option<&mut Box<Node<T>>>

        // self.head        -> &mut Option<None>
        // temp             -> &mut Option<     Box<Node<T>>>

        // map(node)        ->                  Box<Node<T>>
        // node.elem        ->                           T
        // map->node.elem   ->      Option<              T  >
        self.head.take().map(|node| {
            node.elem
        })
    }
    
}

#[test]
fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1); list.push(2); list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
    // match list.peek_mut() {
    //     Some(k) => {*k = 30},
    //     None => {}
    // }
    list.peek_mut().map(|val| {
        *val = 30;
    });
    assert_eq!(list.peek_mut(), Some(&mut 30));
    if let Some(val) = list.peek_mut() {
        println!("{}", val);
        *val = 10;
    }
    assert_eq!(list.peek_mut(), Some(&mut 10));
    assert_eq!(list.pop(), Some(10));
    assert_eq!(list.peek(), Some(&2));
    if let Some(val) = list.peek_() {
        println!("{}", val);
    }
}
```

### IntoIter
```rust
// 3.4 IntoIter------------------------------------------------------------------------------

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[test]
fn into_iter_test() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
}

```

### Iter
```rust
// 3.5 Iter------------------------------------------------------------------------------
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<T> List<T> {
    // 1. initial
    // pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    // 2. apply lifetime elision
    // pub fn iter(&self) -> Iter<T> {
    // 3. Or, if you're not comfortable "hiding" that a struct contains a lifetime, you can use the Rust 2018 "explicitly elided lifetime" syntax, '_:
    pub fn iter(&self) -> Iter<'_, T> {
        
        // self             -> &List                    | self                -> &List  
        // self.head        -> &Option< Box<Node<T>>>   | self.head           -> &Option< Box<Node<T>>> 
        // self.head.as_ref ->  Option<&Box<Node<T>>>   | self.head.as_deref  -> &Option<     Node<T> >
        // map(node)        ->         &Box<Node<T>>    |
        // *node            ->          Box<Node<T>>    |
        // **node           ->              Node<T>     |
        // &**node          ->             &Node<T>     |
        // map->node        ->  Option<    &Node<T> >   |

        // can be replaced by the next line
        Iter { next: self.head.as_ref().map(|node| { &**node }) }

        // Iter { next: self.head.as_deref() }

        // node: expected struct `second::Node`, found struct `std::boxed::Box`
        // *node: expected `&second::Node<T>`, found struct `std::boxed::Box`
        // **node: expected `&second::Node<T>`, found struct `second::Node`
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {

        // self                  -> &mut Iter                           | self                   -> &mut Iter  
        // self.next             -> &mut Option<         &Node<T> >     | self.next              -> &mut Option<         &Node<T>>  
        // map(node)             -> &mut                 &Node<T>       | map(node)              -> &mut                 &Node<T>
        //     node.next         -> &mut Option<         &Node<T> >     |     node.next          -> &mut Option<     Box< Node<T>>>
        //     node.next.as_ref  ->      Option<&mut Box< Node<T>>>     |     node.next.as_deref -> &mut Option<          Node<T> > 
        //     map(inner_node)   ->             &mut Box< Node<T>>      |
        //     *inner_node       ->                  Box< Node<T>>      |
        //     **inner_node      ->                       Node<T>       |
        //     &**inner_node     ->                      &Node<T>       |
        //     map(inner_node)   ->      Option<         &Node<T> >     |
        // self.next             -> &mut Option<         &Node<T>>      | self.next              -> &mut Option<& Node<T>>
        // node.elem             -> &mut                       T        |
        // &node.elem            -> &mut                      &T        |
        // map->&node.elem       -> &mut Option<              &T >      |

        self.next.map(|node| {
            self.next = (*node).next.as_ref().map(|node| &**node);
            // self.next = node.next.as_deref();
            // self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            // node.next = Some(Box::new(Node{elem: 1, next: None}));
            &node.elem
        })
    }
}

#[test]
fn iter() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}
```

### 

## A Persistent Stack

### 实现目标
```
list1 = A -> B -> C -> D
list2 = tail(list1) = B -> C -> D
list3 = push(list2, X) = X -> B -> C -> D

list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+
```

### Basic

```rust

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

```

### Iter
```rust
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

```

### Drop
多了判断ref count的过程
```rust
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

```

### Test
```rust
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

```

## A Bad Safe Deque
加入了Rc和RefCell

### Layout
```rust
se std::rc::Rc;
use std::cell::{ Ref, RefMut, RefCell };

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
} 
```

### Basic

```rust
impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None, tail: None }
    }
    // 5.2 Building下`
    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            },
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }
    // 5.3 Breaking
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            // 临时借用old_head, 并take了next的ownership
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
    // 5.4 Peeking
    // pub fn peek(&self) -> Option<&T> {
    //     self.head.as_ref().map(|node| {
    //         // Rc::try_unwrap(node).ok().unwrap().into_inner().elem
    //         // &node.borrow().elem
    //     })
    // }
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }
    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }
    // Back ---------------------------------------------------------
    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            } 
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }
    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            elem,
            next:None,
            prev: None,
        }))
    }
}
```

### Drop
```rust
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    } 
}

```

### IntoIter
另一个方向直接实现DoubleEndedIterator Trait就行
```rust
// IntoIter
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

// natural
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}
// reverse
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

```

### Iter(danger)
::: danger
无法实现而且根本看不懂, 一头雾水
:::
```rust
// Iter
pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_ref().map(|head| {head.borrow()}))
        // 若为 Ref<T>
        // Iter(self.head.as_ref().map(|head| {Ref::map(head.borrow(), |head| &head.elem)}))
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;
    fn next(&mut self) -> Option<Ref<'a, T>> {
        self.0.take().map(|node_ref| {
            // self.0 = node_ref.next.as_ref().map(|head| {head.borrow()});
            // // node_ref在闭包内被借用
            // Ref::map(node_ref, |node| &node.elem)
            // // node_ref在闭包外再次被借用
            let (next, elem) = Ref::map_split(node_ref, |node| {
                (&node.next, &node.elem)
            });
            self.0 = next.as_ref().map(|head| head.borrow());
            elem
        })
    }   
}

```

## An Unsafe Queue

### Safe Rust
Well, pushing is actually fine.

#### Push
```rust
// An Unsafe Queue

/*
input list:
[Some(ptr)] -> (A, Some(ptr)) -> (B, None)

flipped push X:
[Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)

*/

// Layout
pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// Basic
impl<'a, T> List<'a, T> {
    pub fn new() -> List<'a, T> {
        List { head: None, tail: None }
    }

    pub fn push(&'a mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem: elem,
            // When you push onto the tail, your next is always None
            next: None,
        });

        // Put the box in the right place, and then grab a reference to its Node
        match self.tail.take() {
            Some(old_tail) => {
                // If the old tail existed, update it to point to the new tail
                old_tail.next = Some(new_tail);
                self.tail = old_tail.next.as_deref_mut()
            }
            None => {
                // Otherwise, update the head to point to it
                self.head = Some(new_tail);
                self.tail = self.head.as_deref_mut()
            }
        };
    }
}
```

#### Pop
>However pop is another story. If they're popping elements outside of our range, it should still be fine. We can't see those nodes so nothing will happen. However if they try to pop off the node we're pointing at... >everything will blow up! In particular when they go to unwrap the result of the try_unwrap, it will actually fail, and the whole program will panic.
```rust
pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|head| {
        let head = *head;
        self.head = head.next;
        if self.head.is_none() {
            self.tail = None;
        }
        head.elem
    })
}
```

### Layout
```rust
// Layout
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

use std::ptr;
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail:  ptr::null_mut()}
    }
    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut()
            };
            head.elem
        })
    }
} 
```

### Extras
```rust
pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<T> List<T> {
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
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
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
```

### Test
```rust

mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}

```
