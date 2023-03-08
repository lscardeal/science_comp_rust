use crate::data_structures::nodes::linked_node::LinkedNode;

pub struct LinkedList<'a, T: 'static> {
    head: Option<Box<LinkedNode<T>>>,
    tail: Option<&'a Box<LinkedNode<T>>>,
    size: usize
}

impl<'a, T> LinkedList<'a, T> {

    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0
        }
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.size = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn has_one_node(&self) -> bool {
        self.size == 1
    }

    pub fn add_first(&mut self, value: T) {
        let mut node: LinkedNode<T> = self.create_node(value);
        node.set_next(self.head.take());
        self.head = Some(Box::from(node));

        if self.tail.is_none() {
            self.tail = self.head.as_ref();
        }

        self.size += 1;
    }

    pub fn add_last(&mut self, value: T) {
        let node: Option<Box<LinkedNode<T>>> = self.create_assignable_node(value);
        
        if self.is_empty() {
            self.head = node;
            self.tail = self.head.as_ref()
        } else {
            let tail = &(self.tail.take()).unwrap();
            let tail = tail.as_mut();
            
            tail.set_next(node);
            self.tail = tail.next().as_ref();
        }
        
        self.size += 1;
    }

    pub fn get_first(&self) -> Option<&Box<LinkedNode<T>>> {
        self.head.as_ref()
    }

    pub fn get_last(&self) -> Option<&Box<LinkedNode<T>>> {
        self.tail
    }

    // public T get(int index) throws IndexOutOfBoundsException {

    // public void add(int index, T value) throws IndexOutOfBoundsException {

    pub fn remove_first(&mut self) {
        if self.is_empty() {
            return
        } else if self.has_one_node() {
            self.clear();
        }
        
        let head = *(self.head.take().unwrap());
        let next = *(head.next().unwrap());

        self.head = Some(Box::from(next));
        self.size -= 1;
    }

    pub fn remove_last(&mut self) {
        if self.is_empty() {
            return
        } else if self.has_one_node() {
            self.clear()
        }

        let mut previous: &mut LinkedNode<T> = unsafe { self.head.unwrap().as_mut() };
        let mut current: &mut LinkedNode<T> = unsafe { previous.next().unwrap().as_mut() };
        
        while (current.has_next()) {
            previous = current;
            current = unsafe { previous.next().unwrap().as_mut() };
        }

        previous.set_next(None);
        self.size -= 1;
    }

    // public void remove(int index) throws IndexOutOfBoundsException {

    // public int indexOf(T value) {

    // public boolean contains(T value) {

    // public void remove(T value) {

    pub fn create_node(&self, value: T) -> LinkedNode<T> {
        LinkedNode::new(value)
    }

    fn create_assignable_node(&self, value: T) -> Option<Box<LinkedNode<T>>> {
        let node = self.create_node(value);
        Some(Box::from(node))
    }

    // private void checkIndex(int index) throws IndexOutOfBoundsException {

}