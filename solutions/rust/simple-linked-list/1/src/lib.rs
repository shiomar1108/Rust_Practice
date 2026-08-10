pub struct SimpleLinkedList<T>(Vec<T>); 

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, element: T) {
        self.0.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.last()
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut r = self.0;
        r.reverse();
        Self(r)
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut r:Vec<T> = Vec::new(); 
        r.extend(_iter);
        SimpleLinkedList(r)
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        _linked_list.0
    }
}