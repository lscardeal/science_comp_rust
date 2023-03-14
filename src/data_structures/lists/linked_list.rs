use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug)]
#[allow(dead_code)]
pub struct LinkedNode<T>{
    value: T,
    next: Option<NonNull<LinkedNode<T>>>
}

impl<T> LinkedNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value, 
            next: None
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<NonNull<LinkedNode<T>>>,
    tail: Option<NonNull<LinkedNode<T>>>,
    length: usize,
    marker: PhantomData<Box<LinkedNode<T>>>
}

#[allow(dead_code)]
impl<T> LinkedList<T> {

    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
            marker: PhantomData
        }
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.length = 0;
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn has_one_node(&self) -> bool {
        self.length == 1
    }

    pub fn add_first(&mut self, value: T) {
        let mut node = Box::new(LinkedNode::new(value));
        node.next = self.head.take();
        self.head = NonNull::new(Box::into_raw(node));
        self.length += 1;

        if self.tail.is_none() {
            self.tail = self.head;
        }
    }

    pub fn add_last(&mut self, value: T) {
        let node = Box::new(LinkedNode::new(value));
        let node_ptr = NonNull::new(Box::into_raw(node));

        if let Some(mut tail) = self.tail {
            let mut tail = unsafe { tail.as_mut() };
            tail.next = node_ptr;
        }
        self.tail = node_ptr;

        if self.head.is_none() {
            self.head = self.tail;
        }
    }

    pub fn get_first(&self) -> Option<NonNull<LinkedNode<T>>> {
        self.head
    }

    pub fn get_last(&self) -> Option<NonNull<LinkedNode<T>>> {
        self.tail
    }

    pub fn get(&mut self, index: usize) -> Option<&'static T> {
        self.get_helper(self.head, index)
    }

    fn get_helper(&mut self, node: Option<NonNull<LinkedNode<T>>>, index: usize) -> Option<&'static T>  {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).value }),
                _ => self.get_helper( unsafe {(*next_ptr.as_ptr()).next }, index - 1)
            } 
        }
    }

    /*
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

    // private void checkIndex(int index) throws IndexOutOfBoundsException {*/

}