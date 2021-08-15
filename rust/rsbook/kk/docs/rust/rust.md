# Rust

## Path

### stdlib
```rust
// 从 `&'static str` 创建一个 `Path`
let path = Path::new(".");

// `display` 方法返回一个可显示（showable）的结构体
let display = path.display();

// `join` 使用操作系统特定的分隔符来合并路径到一个字节容器，并返回新的路径
let new_path = path.join("a").join("b");

// 将路径转换成一个字符串切片
match new_path.to_str() {
    None => panic!("new path is not a valid UTF-8 sequence"),
    Some(s) => println!("new path is {}", s),
}
```

## lifetime

### 自动处理
```rust
// Only one reference in input, so the output must be derived from that input
fn foo(&A) -> &B; // sugar for:
fn foo<'a>(&'a A) -> &'a B;

// Many inputs, assume they're all independent
fn foo(&A, &B, &C); // sugar for:
fn foo<'a, 'b, 'c>(&'a A, &'b B, &'c C);

// Methods, assume all output lifetimes are derived from `self`
fn foo(&self, &B, &C) -> &D; // sugar for:
fn foo<'a, 'b, 'c>(&'a self, &'b B, &'c C) -> &'a D;
```

## Rc & Arc

### Introduce
The key to our design is the RefCell type. The heart of RefCell is a pair of methods:
```rust
fn borrow(&self) -> Ref<'_, T>;
fn borrow_mut(&self) -> RefMut<'_, T>;
```

### try_unwrap()
Get T from Rc\<T\>
try to use `try_unwrap()`, which moves out the contents of an Rc if its refcount is 1.
::: warning 
unwrap on Result requires that you can debug-print the error case. RefCell<T> only implements Debug if T does. Node doesn't implement Debug.
try: Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
:::

### From Std Lib
>Introducing inherited mutability roots to shared types
Shared smart pointer types, including Rc<T> and Arc<T>, provide containers that can be cloned and shared between multiple parties. Because the contained values may be multiply-aliased, they can only be borrowed as shared references, not mutable references. Without cells it would be impossible to mutate data inside of shared boxes at all!

>It's very common then to put a RefCell<T> inside shared pointer types to reintroduce mutability:
```rust
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    shared_map.borrow_mut().insert("africa", 92388);
    shared_map.borrow_mut().insert("kyoto", 11837);
    shared_map.borrow_mut().insert("piccadilly", 11826);
    shared_map.borrow_mut().insert("marbles", 38);
}
```
>Note that this example uses Rc<T> and not Arc<T>. RefCell<T>s are for single-threaded scenarios. Consider using Mutex<T> if you need shared mutability in a multi-threaded situation.

## Cell & RefCell

### From Std Lib
>Shareable mutable containers.

>Values of the Cell<T> and RefCell<T> types may be mutated through shared references (i.e. the common &T type), whereas most Rust types can only be mutated through unique (&mut T) references. We say that Cell<T> and RefCell<T> provide 'interior mutability', in contrast with typical Rust types that exhibit 'inherited mutability'.

>Cell types come in two flavors: Cell<T> and RefCell<T>. Cell<T> provides get and set methods that change the interior value with a single method call. Cell<T> though is only compatible with types that implement Copy. For other types, one must use the RefCell<T> type, acquiring a write lock before mutating.

>RefCell<T> uses Rust's lifetimes to implement 'dynamic borrowing', a process whereby one can claim temporary, exclusive, mutable access to the inner value. Borrows for RefCell<T>s are tracked 'at runtime', unlike Rust's native reference types which are entirely tracked statically, at compile time. Because RefCell<T> borrows are dynamic it is possible to attempt to borrow a value that is already mutably borrowed; when this happens it results in thread panic.

## Ref & RefMut

### Ref::map()
1. mine
```rust
map(Ref< T>, f: F) -> Ref<U>
// Get Ref<T> from Ref<Node<T>>
// my example
pub fn peek_front(&self) -> Option<Ref<T>> {
    self.head.as_ref().map(|node| {
        Ref::map(node.borrow(), |node| &node.elem)
    })
}
```
2. from std lib
```rust

Makes a new Ref for a component of the borrowed data.

The RefCell is already immutably borrowed, so this cannot fail.

This is an associated function that needs to be used as Ref::map(...). A method would interfere with methods of the same name on the contents of a RefCell used through Deref.

Examples
use std::cell::{RefCell, Ref};

let c = RefCell::new((5, 'b'));
let b1: Ref<(u32, char)> = c.borrow();
let b2: Ref<u32> = Ref::map(b1, |t| &t.0);
assert_eq!(*b2, 5)
```

### RefMut::map()
同上

##