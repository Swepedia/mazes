use std::any::Any;
use std::cell::RefCell;
use std::collections::hash_map::*;
use std::hash::{Hash, Hasher};
use std::rc::*;

type LinkStrong = Rc<RefCell<Node>>;
type LinkWeak = Weak<RefCell<Node>>;

#[derive(Default, Clone, Debug)]
pub struct Node {
    pub row: usize,
    pub column: usize,
    links: HashMap<LinkWeak, LinkWeak>,
    north: Option<LinkWeak>,
    east: Option<LinkWeak>,
    south: Option<LinkWeak>,
    west: Option<LinkWeak>,
    self_rc: LinkWeak,
}

impl Node {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
            links: HashMap::new(),
            north: None,
            east: None,
            south: None,
            west: None,
            self_rc: Weak::new(),
        }
    }

    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn link(&mut self, cell: LinkStrong) {
        if let Some(new_link) = cell.borrow().as_any().downcast_ref::<Node>() {
            let other: LinkWeak = Rc::downgrade(&Rc::clone(&new_link.self_rc.upgrade().unwrap()));
            self.links.insert(other, other);
        }
    }

    pub fn unlink(&mut self, cell: Rc<Node>) {
        self.links.remove(&cell);
    }

    pub fn links(&self) -> Keys<'_, Rc<Node>, Rc<Node>> {
        self.links.keys()
    }

    pub fn is_linked(&self, cell: Rc<Node>) -> bool {
        self.links.contains_key(&cell)
    }

    //pub fn set_north(&mut self, node: Node<'a>) {
    //    self.north = Some(&node);
    //}

    //pub fn set_south(&mut self, node: Node<'a>) {
    //    self.south = Some(&node);
    //}

    //pub fn set_east(&mut self, node: Node<'a>) {
    //    self.east = Some(&node);
    //}

    //pub fn set_west(&mut self, node: Node<'a>) {
    //    self.west = Some(&node);
    //}

    pub fn neighbors(&self) -> Vec<Rc<Node>> {
        let mut neighbors = Vec::new();
        self.north.map(|node| {
            neighbors.push(node);
        });
        self.south.map(|node| {
            neighbors.push(node);
        });
        self.east.map(|node| {
            neighbors.push(node);
        });
        self.west.map(|node| {
            neighbors.push(node);
        });
        neighbors
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.column.hash(state);
    }
}

#[cfg(test)]
mod test {
    use super::Node;
    use std::rc::Rc;

    #[test]
    fn link() {
        let mut node_1 = Rc::new(Node::new(0, 0));
        let node_2 = Rc::new(Node::new(1, 0));

        node_1.link(node_2);
        assert_eq!(node_1.is_linked(node_2), true);
        assert_eq!(node_2.is_linked(node_1), false);

        for node in node_1.links() {
            assert_eq!(**node, node_2);
        }

        node_1.unlink(node_2);
        assert_eq!(node_1.is_linked(node_2), false);
        assert_eq!(node_2.is_linked(node_1), false);
    }

    #[test]
    fn neighbors() {
        let node_1 = Rc::new(Node::new(0, 0));
        let mut node_2 = Rc::new(Node::new(1, 0));

        node_2.north = Some(node_1.clone());

        assert_eq!(node_2.neighbors().first(), node_1)
    }
}
