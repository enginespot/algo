use std::collections::BTreeMap;

pub trait PriorityQueue<Element> {
    /// create a new priority queue.
    fn new() -> Self;
    /// check whether the queue has no elements.
    fn is_empty(&self) -> bool;
    /// returns the highest-priority element but does not modify the queue.
    fn peek(&self) -> Option<&Element>;
    /// add an element to the queue with an associated priority.
    fn insert(&mut self, element: Element, priority: u64);
    /// remove the element from the queue that has the highest priority, and return it.
    fn pop(&mut self) -> Option<Element>;
}

type KeyValueStore<Element> = BTreeMap<CustomQueueEntry, Element>;

// Additional requirement: the underlying data structure needs to be a key-value stores
// Note: you may simulate other data structure with key-value store
pub struct PriorityQueueImpl<Element> {
    data: KeyValueStore<Element>,
}
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]

struct CustomQueueEntry {
    priority: u64,
    index: usize,
}

impl CustomQueueEntry {
    pub fn new(index: usize, priority: u64) -> CustomQueueEntry {
        CustomQueueEntry { priority, index }
    }
}

impl<Element> From<Vec<(u64, Element)>> for PriorityQueueImpl<Element> {
    fn from(vec: Vec<(u64, Element)>) -> PriorityQueueImpl<Element> {
        let mut queue = PriorityQueueImpl::new();
        vec.into_iter().for_each(|(p, v)| queue.insert(v, p));
        queue
    }
}
impl<Element> PriorityQueueImpl<Element> {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<Element> PriorityQueue<Element> for PriorityQueueImpl<Element> {
    fn new() -> Self {
        PriorityQueueImpl {
            data: BTreeMap::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn peek(&self) -> Option<&Element> {
        self.data.iter().next_back().map(|(_, v)| v.clone())
    }

    fn insert(&mut self, element: Element, priority: u64) {
        self.data.insert(
            CustomQueueEntry::new(self.data.len(),priority),
            element,
        );
    }

    fn pop(&mut self) -> Option<Element> {
        let key = self.data.iter().next_back().map(|(k, _)| *k);
        key.and_then(|k| self.data.remove(&k))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut queue = PriorityQueueImpl::new();
        assert!(queue.is_empty());

        queue.insert(vec![0], 5);
        assert!(!queue.is_empty());
        assert_eq!(queue.peek(), Some(&vec![0]));

        queue.insert(vec![1], 10);
        queue.insert(vec![2], 3);
        queue.insert(vec![3], 4);
        queue.insert(vec![4], 6);

        assert_eq!(queue.pop(), Some(vec![1]));
        assert_eq!(queue.pop(), Some(vec![4]));
        assert_eq!(queue.pop(), Some(vec![0]));
        assert_eq!(queue.pop(), Some(vec![3]));
        assert_eq!(queue.pop(), Some(vec![2]));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut queue = PriorityQueueImpl::new();
        assert!(queue.is_empty());
        queue.insert(vec![1], 10);
        assert_eq!(queue.peek(), Some(&vec![1]));
        assert_eq!(queue.len(), 1);
        queue.insert(vec![3], 10);
        assert_eq!(queue.peek(), Some(&vec![3]));
        assert_eq!(queue.len(), 2);
        queue.insert(vec![5], 11);
        assert_eq!(queue.peek(), Some(&vec![5]));
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_empty_peek() {
        let queue = PriorityQueueImpl::<i32>::new();
        assert!(queue.is_empty());
        assert!(queue.peek().is_none());
    }

    #[test]
    fn test_empty_pop() {
        let mut queue = PriorityQueueImpl::<i32>::new();
        assert!(queue.is_empty());
        assert!(queue.pop().is_none());
    }

    #[test]
    fn test_peek_pop() {
        let mut queue = PriorityQueueImpl::from(vec![
            (5, vec![0]),
            (10, vec![1]),
            (3, vec![2]),
            (4, vec![3]),
            (6, vec![4]),
        ]);
        assert!(!queue.is_empty());
        assert_eq!(queue.peek(), Some(&vec![1]));
        assert_eq!(queue.pop(), Some(vec![1]));
        assert_eq!(queue.pop(), Some(vec![4]));
        assert_eq!(queue.peek(), Some(&vec![0]));
        assert_eq!(queue.pop(), Some(vec![0]));
        assert_eq!(queue.peek(), Some(&vec![3]));
        assert_eq!(queue.pop(), Some(vec![3]));
        assert_eq!(queue.peek(), Some(&vec![2]));
        assert_eq!(queue.pop(), Some(vec![2]));
        assert!(queue.peek().is_none());
        assert!(queue.pop().is_none());
        assert!(queue.is_empty());
    }
}
