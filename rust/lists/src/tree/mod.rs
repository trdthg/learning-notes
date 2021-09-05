pub mod insert;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use insert::*;

const MAX_DEGREE: usize = 5;

#[derive(Clone, Debug)]
pub enum LinkType {
    Branch(Rc<RefCell<BranchNode>>),
    Leaf(Rc<RefCell<LeafNode>>),
    Data(DataNode),
}

#[derive(Clone, Debug)]
pub struct DataNode {}
#[derive(Clone, Debug)]
pub struct Node {
    pub id: Option<usize>,
    pub link: LinkType,
}
impl Node {
    pub fn new(id: usize, link: LinkType) -> Node {
        Node { id: Some(id), link }
    }
}

#[derive(Clone, Debug)]
pub struct BranchNode {
    pub ids: Vec<Node>,
    pub father: Option<Weak<RefCell<BranchNode>>>,
}
impl BranchNode {
    pub fn new(ids: Vec<Node>) -> BranchNode {
        BranchNode { ids, father: None }
    }
}

#[derive(Clone, Debug)]
pub struct LeafNode {
    pub ids: Vec<Node>,
    pub father: Option<Weak<RefCell<BranchNode>>>,
    pub next: Option<Rc<RefCell<LeafNode>>>,
}
impl LeafNode {
    pub fn new(id: usize, data: DataNode) -> LeafNode {
        let link = LinkType::Data(data);
        let ids = vec![Node::new(id, link)];
        LeafNode {
            ids,
            next: None,
            father: None,
        }
    }
}
#[derive(Debug)]
pub struct BPlusTree {
    pub root: Option<LinkType>,
}

impl BPlusTree {
    pub fn new() -> BPlusTree {
        BPlusTree { root: None }
    }

    fn insert(&mut self, id: usize, data: DataNode) {
        match &self.root.clone() {
            Some(LinkType::Leaf(node)) => {
                let len = insert_leaf(node, id, data);
                if len == MAX_DEGREE {
                    self.root = Some(LinkType::Branch(splite_leaf(&node.clone())));
                }
            }
            Some(LinkType::Branch(_node)) => {
                if let Some(new_root) = find_leaf(_node, id, data) {
                    self.root = Some(LinkType::Branch(new_root));
                }
            }
            None => {
                let new_branch = LeafNode::new(id, data);
                self.root = Some(LinkType::Leaf(Rc::new(RefCell::new(new_branch))));
            }
            _ => {}
        }
    }

    fn aaa(node: Rc<RefCell<BranchNode>>) {
        for id in node.borrow().ids.clone() {
            match id.link {
                LinkType::Branch(branch) => {
                    for d in branch.borrow().clone().ids {
                        if d.id.is_some() {
                            println!("{}", d.id.unwrap());
                        }
                    }
                }
                LinkType::Leaf(leaf) => {
                    for d in leaf.borrow().clone().ids {
                        if d.id.is_some() {
                            println!("{}", d.id.unwrap());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn travel(&self) {
        // TODO
        match self.root.clone() {
            Some(LinkType::Branch(node)) => {
                for id in node.borrow().ids.clone() {
                    match id.link {
                        LinkType::Branch(branch) => {
                            for d in branch.borrow().clone().ids {
                                if d.id.is_some() {
                                    println!("{}", d.id.unwrap());
                                }
                            }
                        }
                        LinkType::Leaf(leaf) => {
                            for d in leaf.borrow().clone().ids {
                                if d.id.is_some() {
                                    println!("{}", d.id.unwrap());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {

            }
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn a() {
        let now = std::time::Instant::now();
        let mut tree = BPlusTree::new();
        // tree.insert(5, DataNode {});
        // tree.insert(8, DataNode {});
        // tree.insert(10, DataNode {});
        // tree.insert(15, DataNode {});
        // tree.insert(16, DataNode {});
        // tree.insert(17, DataNode {});
        // tree.insert(18, DataNode {});
        // tree.insert(19, DataNode {});
        // tree.insert(20, DataNode {});
        // tree.insert(21, DataNode {});
        // tree.insert(22, DataNode {});
        // tree.insert(23, DataNode {});
        // tree.insert(24, DataNode {});
        // tree.insert(25, DataNode {});
        // tree.insert(26, DataNode {});
        // tree.insert(7, DataNode {});
        // tree.insert(2, DataNode {});
        // tree.insert(30, DataNode {});
        // tree.insert(31, DataNode {});
        // tree.insert(32, DataNode {});
        // tree.insert(33, DataNode {});

        // tree.insert(34, DataNode {});
        // tree.insert(35, DataNode {});
        // tree.insert(36, DataNode {});
        for i in 1..19 {
            // tree.insert(i, DataNode {});
            // std::thread::sleep(std::time::Duration::from_nanos(10000));
            tree.insert(i, DataNode{});
        }
        tree.insert(19, DataNode{});
        
        tree.travel();
        println!("{:?}", now.elapsed());
        // tree.insert(2, DataNode {});

        println!("{:?}", tree);
        println!("{:#?}", tree);
    }

    #[test]
    fn b() {
        let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let num = 42;
        // match s.binary_search(&num) {
        //     Ok(pos) => {} // element already in vector @ `pos`
        //     Err(pos) => s.insert(pos, num),
        // }
        let mut s = vec![1, 5, 9];
        let num = 10;
        let idx = s.binary_search(&num).unwrap_or_else(|x| x);
        match s.get(idx) {
            Some(_) => {
                // 若有就插入
            }
            None => {
                // 若为空就修改
            }
        }
        s.insert(idx, num);
        println!("{:#?} {:?}", idx, s);
    }

    #[test]
    fn c() {
        let a = vec![1, 1, 1, 1, 1];
        let b = a[0..1].to_owned();
        println!("{:?}", &a[1..4]);
    }
}
