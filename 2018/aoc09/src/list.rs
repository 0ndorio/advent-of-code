use std::{cell::RefCell, rc::Rc};

use crate::Result;

pub struct Circle<ValueT> {
    pub current: NodeLink<ValueT>,
}

impl<ValueT> Circle<ValueT>
where
    ValueT: std::fmt::Debug,
{
    pub fn new(node: Node<ValueT>) -> Self {
        let initial_link = Rc::new(RefCell::new(node));

        {
            let mut root = initial_link.borrow_mut();
            root.prev = Some(Rc::clone(&initial_link));
            root.next = Some(Rc::clone(&initial_link));
        }

        Circle {
            current: initial_link,
        }
    }

    pub fn move_forward(&mut self, steps: u32) {
        self.current = self.peek_forward(steps);
    }

    fn peek_forward(&self, steps: u32) -> NodeLink<ValueT> {
        let mut peek = Rc::clone(&self.current);
        for _ in 0..steps {
            let next = match &peek.borrow().next {
                Some(link) => Rc::clone(&link),
                _ => unreachable!("This can only trigger if our remove logic is flawed"),
            };

            peek = next;
        }

        peek
    }

    pub fn move_backwards(&mut self, steps: u32) {
        self.current = self.peek_backwards(steps);
    }

    fn peek_backwards(&self, steps: u32) -> NodeLink<ValueT> {
        let mut peek = Rc::clone(&self.current);
        for _ in 0..steps {
            let prev = match &peek.borrow().prev {
                Some(link) => Rc::clone(&link),
                _ => unreachable!("This can only trigger if our remove logic is flawed"),
            };

            peek = prev;
        }

        peek
    }

    pub fn insert(&mut self, value: ValueT) {
        let new_link = Rc::new(RefCell::new(Node::new(value)));

        let next = self.peek_forward(1);
        if Rc::ptr_eq(&self.current, &next) {
            let mut current_node = self.current.borrow_mut();
            current_node.next = Some(Rc::clone(&new_link));
            current_node.prev = Some(Rc::clone(&new_link));

            let mut new_node = new_link.borrow_mut();
            new_node.next = Some(Rc::clone(&self.current));
            new_node.prev = Some(Rc::clone(&self.current));
        } else {
            let mut current_node = self.current.borrow_mut();
            current_node.next = Some(Rc::clone(&new_link));

            let mut old_next = next.borrow_mut();
            old_next.prev = Some(Rc::clone(&new_link));

            let mut new_node = new_link.borrow_mut();
            new_node.next = Some(Rc::clone(&next));
            new_node.prev = Some(Rc::clone(&self.current));
        }

        self.current = new_link;
    }

    pub fn remove(&mut self) -> Result<ValueT> {
        let next = self.peek_forward(1);
        let prev = self.peek_backwards(1);

        if Rc::ptr_eq(&prev, &self.current) {
            return Err("Can't remove last circle element.")?;
        }

        {
            let mut next_node = next.borrow_mut();
            next_node.prev = Some(Rc::clone(&prev));

            let mut prev_node = prev.borrow_mut();
            prev_node.next = Some(Rc::clone(&next));
        }

        let removed = Rc::clone(&self.current);
        self.current = next;

        let value = Rc::try_unwrap(removed)
            .unwrap_or_else(|_| unreachable!("If this fails our remove logic failed."))
            .into_inner()
            .value;

        Ok(value)
    }
}

pub type NodeLink<ValueT> = Rc<RefCell<Node<ValueT>>>;

#[derive(Debug)]
pub struct Node<ValueT> {
    value: ValueT,
    next: Option<NodeLink<ValueT>>,
    prev: Option<NodeLink<ValueT>>,
}

impl<ValueT> Node<ValueT> {
    pub fn new(value: ValueT) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}
