use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

type NodeRef<T> = Rc<RefCell<T>>;
type ChildrenMap<T> = HashMap<String, NodeRef<TreeNode<T>>>;
type WeakNodeRef<T> = Weak<RefCell<T>>;

pub struct TreeNode<T> {
    pub value: T,
    pub children: Option<ChildrenMap<T>>,
    pub parent: RefCell<Option<WeakNodeRef<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: Some(HashMap::new()),
            parent: RefCell::new(None),
        }
    }

    fn set_parent(&mut self, parent: NodeRef<TreeNode<T>>) {
        *self.parent.borrow_mut() = Some(Rc::downgrade(&parent));
    }
}

pub struct HashedTreeMap<T> {
    pub root: NodeRef<TreeNode<T>>,
    pub map: ChildrenMap<T>,
}

impl<T> HashedTreeMap<T> {
    pub fn new(id: String, root: TreeNode<T>) -> Self {
        let rc_root = Rc::new(RefCell::new(root));
        let map_rc_root = Rc::clone(&rc_root);
        Self {
            root: rc_root,
            map: HashMap::from([(id, map_rc_root)]),
        }
    }

    pub fn insert(&mut self, parent: NodeRef<TreeNode<T>>, id: String, name: String, child: T) {
        let mut node = TreeNode::new(child);
        node.set_parent(Rc::clone(&parent));
        let rc_node = Rc::new(RefCell::new(node));
        let map_node = Rc::clone(&rc_node);
        self.map.insert(id, map_node);
        parent
            .borrow_mut()
            .children
            .as_mut()
            .unwrap_or(&mut HashMap::new())
            .insert(name, Rc::clone(&rc_node));
    }

    pub fn get(&self, id: &str) -> Option<NodeRef<TreeNode<T>>> {
        self.map.get(id).map(|rc| Rc::clone(rc))
    }

    pub fn remove(&mut self, id: &str) {
        if let Some(node_ref) = self.map.remove(id) {
            let mut node = node_ref.borrow_mut();

            if let Some(children) = node.children.take() {
                for child_id in children.keys() {
                    let id = format!("{}/{}", id, child_id);
                    self.remove_child(&id);
                }
            }

            if let Some(parent_weak) = node.parent.replace(None) {
                if let Some(parent) = parent_weak.upgrade() {
                    parent.borrow_mut().children.as_mut().unwrap().remove(id);
                }
            }
        }
    }

    fn remove_child(&mut self, child_id: &str) {
        if let Some(node_ref) = self.map.remove(child_id) {
            let mut node = node_ref.borrow_mut();

            if let Some(children) = node.children.take() {
                for child_id in children.keys() {
                    let id = format!("{}/{}", child_id, child_id);
                    self.remove_child(&id);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn insertion() {
        use super::*;
        let mut tree = HashedTreeMap::new("root".to_string(), TreeNode::new("root"));
        let root = tree.get("root").unwrap();
        tree.insert(root, "1".to_string(), "1".to_string(), "1");
        let one = tree.get("1").unwrap();
        assert_eq!(one.borrow().value, "1");
        tree.insert(one, "2".to_string(), "2".to_string(), "2");
        let two = tree.get("2").unwrap();
        assert_eq!(two.borrow().value, "2");
    }
}
