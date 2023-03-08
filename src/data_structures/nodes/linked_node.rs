pub struct LinkedNode<T>{
    value: T,
    next: Option<Box<LinkedNode<T>>>
}

impl<T> LinkedNode<T> {

    pub fn new(value: T) -> Self {
        Self {
            value, 
            next: None
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn next(&self) -> &Option<Box<LinkedNode<T>>> {
        &self.next
    }

    pub fn set_next(&mut self, node: Option<Box<LinkedNode<T>>>) {
        self.next = node
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }
}