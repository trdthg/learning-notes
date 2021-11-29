//
/**
 * title: An Ok Singly-Linked Stack
 *
 * 3.1: Option
 * 1. use Option to replace our own Enum
 * 2. use Option.take() to replace mem::replace(&self, None)
 * 3. use option.take().map(|elem {}|) to replace match option { None => None, Some(x) => Some(y) }
 *
 * 3.2 Generic
 * 1. just use T to replace i32
 *
 * 3.3 Peek(偷看)
 * 注意3者的区别
 * 1. self.head.take()    -> self       -> Option(T)
 * 2. self.head.as_ref()  -> &self      -> Option(&T)
 * 3. self.head.as_mut()  -> &mut self  -> Option(&mut T)
 *
 * 3.4 - 3.6
 * IntoIter - T
 * IterMut - &mut T
 * Iter - &T
 * 1. IntoIter
 */
// ------------------------------------------------------------------------------
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
        // match mem::replace(&mut self.head, None) {
        // // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem //  node.elem not need to be wrapped by Some()
        })
    }
}
// ------------------------------------------------------------------------------

impl<T> List<T> {
    // 3.3 Peek
    /*
    impl<T> Option<T> {
        pub fn as_ref(&self) -> Option<&T>;
    }
    */

    // Maps an Option<T> to Option<U> by applying a function to a contained value.
    pub fn peek(&self) -> Option<&T> {
        // peek的map不需要修改值, map是不可变借用
        // pop的map需要, map是可变借用

        // Converts from &Option<T> to Option<&T>.

        // self             -> &List
        // self.head        -> &Option< Box<Node<T>>>
        // self.head.as_ref ->  Option<&Box<Node<T>>>
        // map(node)        ->         &Box<Node<T>>
        // node.elem        ->                   T
        // &node.elem       ->         &         T
        // map->&node.elem  ->  Option<&         T  >
        self.head.as_ref().map(|node| &node.elem)
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
        self.head.as_mut().map(|node| &mut node.elem)
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
        self.head.take().map(|node| node.elem)
    }
}

#[test]
fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1);
    list.push(2);
    list.push(3);

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

// 3.5 Iter------------------------------------------------------------------------------
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
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
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }

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
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

// 3.6 IterMut------------------------------------------------------------------------------
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        // IterMut { next: self.head.as_mut().map(|node| &mut **node) }
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        // 不同于Iter的实现使用self.next, 这里使用了self.next.take(),
        // iter是不可变引用, 获取所有权实际上是clone一份&
        // iter_mut是mut, 需要take()掉所有权
        // 所以iter中的self.next == self.next.clone()
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            // self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[test]
fn iter_mut() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}
