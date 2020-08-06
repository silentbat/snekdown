use crate::elements::document::Element;
use std::mem;
use std::sync::{Arc, RwLock};

/// An anchor node that is used for parallel parsing.
///
/// When encountering an import the parser creates a new anchor node at the current location
/// and puts it to the elements of its own anchor node.
/// The anchor node is then used to store all elements of the import.
#[derive(Clone, Debug)]
pub struct AnchorNode {
    elements: Arc<RwLock<Vec<Element>>>,
}

impl AnchorNode {
    pub fn new() -> Self {
        Self {
            elements: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// adds an element to the anchor node
    pub fn add_element(&self, element: Element) {
        self.elements.write().unwrap().push(element)
    }

    /// returns a clone of the nodes elements
    pub fn get_elements(&self) -> Vec<Element> {
        self.elements.read().unwrap().clone()
    }

    /// returns the original vector of elements leaving
    /// the stored vector empty.
    pub fn get_elements_once(&self) -> Vec<Element> {
        mem::replace(&mut (*self.elements.write().unwrap()), Vec::new())
    }

    /// adds an anchor node at the current location and returns it
    pub fn get_anchor(&self) -> AnchorNode {
        let node = AnchorNode::new();
        self.elements
            .write()
            .unwrap()
            .push(Element::AnchorNode(node.clone()));

        node
    }

    /// flattens the anchor into a single vector of elements by taking all the elements of child anchors
    pub fn flatten(&self) {
        let index = 0;
        let mut elements = self.elements.write().unwrap();
        elements.reverse();
        let mut new_elements: Vec<Element> = Vec::with_capacity(elements.len());

        while let Some(element) = elements.pop() {
            if let Element::AnchorNode(a) = &elements[index] {
                a.flatten();
                let mut anchor_elements = a.get_elements_once();
                anchor_elements.reverse();
                elements.append(&mut anchor_elements);
            } else {
                new_elements.push(element);
            }
        }
        elements.append(&mut new_elements);
        elements.reverse();
    }
}
