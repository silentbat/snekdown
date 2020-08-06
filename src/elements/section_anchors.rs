use crate::elements::anchor_node::AnchorNode;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct SectionNode {
    pub(crate) parent: SectionEntry,
    pub(crate) children: Vec<SectionEntry>,
    pub(crate) level: u8,
}

#[derive(Clone, Debug)]
pub struct RootSectionNode {
    pub(crate) children: Vec<SectionEntry>,
}

#[derive(Clone, Debug)]
pub enum SectionEntry {
    Root(Arc<RwLock<SectionNode>>),
    Node(Arc<RwLock<SectionNode>>),
    Anchor(AnchorNode),
}

impl RootSectionNode {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl SectionEntry {
    pub fn get_level(&self) -> u8 {
        match self {
            SectionEntry::Node(node) => node.read().unwrap().level,
            _ => 0,
        }
    }

    /// Creates a new child node for the current node and returns a reference to it
    /// If the current node is an Anchor Node no node is created
    pub fn create_child(&self) -> Option<Arc<RwLock<SectionNode>>> {
        match self {
            SectionEntry::Node(n) => {
                let node = Arc::new(RwLock::new(SectionNode::new(SectionEntry::Node(
                    Arc::clone(&n),
                ))));
                n.write()
                    .unwrap()
                    .children
                    .push(SectionEntry::Node(Arc::clone(&node)));

                Some(node)
            }
            SectionEntry::Root(r) => {
                let node = Arc::new(RwLock::new(SectionNode::new(SectionEntry::Root(
                    Arc::clone(r),
                ))));
                r.write()
                    .unwrap()
                    .children
                    .push(SectionEntry::Node(Arc::clone(&node)));

                Some(node)
            }
            _ => None,
        }
    }
}

impl SectionNode {
    pub fn new(parent: SectionEntry) -> Self {
        Self {
            level: parent.get_level(),
            parent,
            children: Vec::new(),
        }
    }
}
