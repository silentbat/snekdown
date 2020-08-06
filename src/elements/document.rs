use crate::elements::anchor_node::AnchorNode;
use crate::elements::node::Node;
use crate::elements::section_anchors::RootSectionNode;
use crate::elements::Section;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Clone, Debug)]
pub struct Document {
    root_anchor: AnchorNode,
    root_section: Arc<RwLock<RootSectionNode>>,
}

// TODO: move to mod.rs when finished
#[derive(Clone, Debug)]
pub enum Element {
    AnchorNode(AnchorNode),
    Section(Section),
}

impl Node for Element {}

impl Document {
    pub fn new() -> Self {
        Self {
            root_anchor: AnchorNode::new(),
            root_section: Arc::new(RwLock::new(RootSectionNode::new())),
        }
    }
}
