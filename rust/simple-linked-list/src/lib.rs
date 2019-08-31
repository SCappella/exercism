#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new_with_next(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut curr = &self.head;
        while let Some(node) = curr {
            count += 1;
            curr = &node.next;
        }
        count
    }

    pub fn push(&mut self, element: T) {
        let tail = self.head.take();
        let new_node = Node::new_with_next(element, tail);
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        if let Some(node) = head {
            let Node { data, next } = *node;
            self.head = next;
            Some(data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> Self {
        let mut output = Self::new();
        let mut curr_node = &self.head;
        while let Some(node) = curr_node {
            output.push(node.data.clone());
            curr_node = &node.next;
        }
        output
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(items: &[T]) -> Self {
        let mut output = Self::new();
        for item in items {
            output.push(item.clone());
        }
        output
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut output = Vec::new();
        while let Some(item) = self.pop() {
            output.push(item);
        }
        output.reverse();
        output
    }
}
