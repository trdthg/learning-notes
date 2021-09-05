use std::cell::RefMut;

use super::*;

pub fn insert_leaf(node: &Rc<RefCell<LeafNode>>, id: usize, data: DataNode) -> usize {
    let mut node = node.borrow_mut();
    let ids: Vec<usize> = node
        .ids
        .iter()
        .filter_map(|t| Some(t.id.unwrap()))
        .collect();
    let pos = ids.binary_search(&id).unwrap_or_else(|x| x);
    node.ids.insert(pos, Node::new(id, LinkType::Data(data)));
    node.ids.len()
}

pub fn splite_leaf(node: &Rc<RefCell<LeafNode>>) -> Rc<RefCell<BranchNode>> {
    let tmp = node.borrow_mut().ids.clone();
    let mut new_top = BranchNode {
        ids: vec![],
        father: None,
    };
    let p_t = Rc::new(RefCell::new(new_top));
    let new_right = LeafNode {
        ids: tmp[2..5].to_owned(),
        next: None,
        father: Some(Rc::downgrade(&p_t)),
    };
    let p_r = Rc::new(RefCell::new(new_right));
    let new_left = LeafNode {
        ids: tmp[0..2].to_owned(),
        next: Some(p_r.clone()),
        father: Some(Rc::downgrade(&p_t)),
    };
    let p_l = Rc::new(RefCell::new(new_left));
    p_t.borrow_mut().ids = vec![
        Node {
            id: tmp.get(2).unwrap().id,
            link: LinkType::Leaf(p_l.clone()),
        },
        Node {
            id: None,
            link: LinkType::Leaf(p_r.clone()),
        },
    ];
    p_t.clone()
}

pub fn leaf_merge_with_father(
    node: &mut RefMut<BranchNode>,
    father: Rc<RefCell<BranchNode>>,
    leaf_node: &Rc<RefCell<LeafNode>>,
    pos: usize,
) -> usize {
    let new_top = splite_leaf(leaf_node);
    match &node.ids.get(pos).unwrap().id {
        Some(_) => {
            node.ids.insert(pos, new_top.borrow().ids[0].clone());
            // 新的father
            if let LinkType::Leaf(leaf) = &new_top.borrow_mut().ids[0].link {
                leaf.borrow_mut().father = Some(Rc::downgrade(&father));
            }
            if let LinkType::Leaf(leaf) = &new_top.borrow_mut().ids[1].link {
                leaf.borrow_mut().father = Some(Rc::downgrade(&father));
            }

            // // 新的next
            if let LinkType::Leaf(leaf) = &node.ids[pos + 1].link {
                if let LinkType::Leaf(new_next) = &new_top.borrow().ids[1].link {
                    leaf.borrow_mut().next = Some(new_next.clone());
                }
            }
            if pos != 0 {
                if let LinkType::Leaf(leaf) = &node.ids[pos - 1].link {
                    if let LinkType::Leaf(new_next) = &new_top.borrow().ids[0].link {
                        leaf.borrow_mut().next = Some(new_next.clone());
                    }
                }
            }
            // 修正
            node.ids[pos + 1].link = new_top.borrow().ids[1].link.clone();
        }
        None => {
            // 新的father
            if let LinkType::Leaf(leaf) = &new_top.borrow_mut().ids[0].link {
                leaf.borrow_mut().father = Some(Rc::downgrade(&father));
            }
            if let LinkType::Leaf(leaf) = &new_top.borrow_mut().ids[1].link {
                leaf.borrow_mut().father = Some(Rc::downgrade(&father));
            }
            // 新的next
            if let LinkType::Leaf(leaf) = &node.ids[pos - 1].link {
                if let LinkType::Leaf(new_next) = &new_top.borrow().ids[0].link {
                    leaf.borrow_mut().next = Some(new_next.clone());
                }
            }
            // 新的数值
            node.ids[pos].id = new_top.borrow().ids[0].id;
            node.ids[pos].link = new_top.borrow().ids[0].link.clone();
            node.ids.push(new_top.borrow().ids[1].clone());
        }
    }
    node.ids.len()
}

pub fn splite_branch(node: &mut RefMut<BranchNode>) -> Rc<RefCell<BranchNode>> {
    // 记录之前的节点
    let tmp = node.ids.clone();

    // new_top
    let mut new_top = BranchNode {
        ids: vec![],
        father: None,
    };
    let p_t = Rc::new(RefCell::new(new_top));

    // 左右
    let new_left = BranchNode {
        ids: tmp[0..2].to_owned(),
        father: Some(Rc::downgrade(&p_t)),
    };
    // 不用保留重复节点
    let new_right = BranchNode {
        ids: tmp[3..6].to_owned(),
        father: Some(Rc::downgrade(&p_t)),
    };
    let p_l = Rc::new(RefCell::new(new_left));
    let p_r = Rc::new(RefCell::new(new_right));
    p_t.borrow_mut().ids = vec![
        Node {
            id: tmp.get(2).unwrap().id,
            link: LinkType::Branch(p_l.clone()),
        },
        Node {
            id: None,
            link: LinkType::Branch(p_r.clone()),
        },
    ];
    p_t.clone()
}

pub fn branch_merge_with_father(
    father: Rc<RefCell<BranchNode>>,      // node是father
    branch_node: &mut RefMut<BranchNode>, // branch是已经满了的节点
) -> usize {
    let mut node = father.borrow_mut();
    let ids: Vec<usize> = father.borrow()
        .ids
        .iter()
        .filter(|t| t.id.is_some())
        .map(|t| t.id.unwrap())
        .collect();

    let id = branch_node.ids[2].id.unwrap();
    let pos = ids.binary_search(&id).unwrap_or_else(|x| x);

    let new_top = splite_branch(branch_node);
    match &node.ids.get(pos).unwrap().id {
        Some(_) => {
            if let LinkType::Branch(branch) = &new_top.borrow_mut().ids[0].link {
                branch.borrow_mut().father = Some(Rc::downgrade(&father));
            }
            if let LinkType::Leaf(leaf) = &new_top.borrow_mut().ids[1].link {
                leaf.borrow_mut().father = Some(Rc::downgrade(&father));
            }
            node.ids.insert(pos, new_top.borrow_mut().ids[0].clone());
            // 修正
            node.ids[pos + 1].link = new_top.borrow().ids[1].link.clone();
        }
        None => {
            // 新的father
            if let LinkType::Branch(branch) = &new_top.borrow_mut().ids[0].link {
                branch.borrow_mut().father = Some(Rc::downgrade(&father));
            }
            if let LinkType::Leaf(leaf) = &new_top.borrow_mut().ids[1].link {
                leaf.borrow_mut().father = Some(Rc::downgrade(&father));
            }
            // 新的数值
            node.ids[pos].id = new_top.borrow().ids[0].id;
            node.ids[pos].link = new_top.borrow().ids[0].link.clone();
            node.ids.push(new_top.borrow().ids[1].clone());
        }
    }
    node.ids.len()
}

pub fn merge(_node: &mut RefMut<BranchNode>) -> Option<Rc<RefCell<BranchNode>>> {
    // 已经是一个满了的branch_node

    // splite父节点
    if _node.father.clone().is_none() {
        let new_top = splite_branch(_node);
        return Some(new_top);
    } else {
        // 是否递归与父节点合并

        // !!! 无法重复借出father节点
        if let Some(fa) = _node.father.clone().unwrap().upgrade() {
            if branch_merge_with_father(fa.clone(), _node) == MAX_DEGREE + 1 {
                // 父节点也满了, 需要向上递归
                merge(
                    &mut _node
                        .father
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .borrow_mut(),
                );
            }
        }
    }
    None
}

pub fn find_leaf(
    _node: &Rc<RefCell<BranchNode>>,
    id: usize,
    data: DataNode,
) -> Option<Rc<RefCell<BranchNode>>> {
    let father = _node.clone();
    let mut node = _node.borrow_mut();
    let ids: Vec<usize> = node
        .ids
        .iter()
        .filter(|t| t.id.is_some())
        .map(|t| t.id.unwrap())
        .collect();
    let pos = ids.binary_search(&id).unwrap_or_else(|x| x);
    match &node.ids.get(pos).unwrap().link.clone() {
        LinkType::Leaf(leaf_node) => {
            // 先插入后看是否满再进行下一步操作
            if insert_leaf(leaf_node, id, data) == MAX_DEGREE {
                // leaf节点满了, 父节点有None节点
                if leaf_merge_with_father(&mut node, father, leaf_node, pos) == MAX_DEGREE + 1 {
                    if let Some(new_root) = merge(&mut node) {
                        return Some(new_root);
                    }
                }
            }
        }
        LinkType::Branch(lower_node) => {
            // 递归查找下一层
            return find_leaf(lower_node, id, data);
        }
        _ => {}
    }
    None
}
