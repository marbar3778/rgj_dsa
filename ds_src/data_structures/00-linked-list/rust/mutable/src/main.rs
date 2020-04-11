fn main() {
    println!("hello world")
}
pub struct List<T> {
    start: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { start: None }
    }

    pub fn add(&mut self, elem: T) {
        let node = Box::new(Node {
            val: elem,
            next: self.start.take(),
        });

        self.start = Some(node);
    }

    pub fn remove(&mut self) -> Option<T> {
        self.start.take().map(|node| {
            self.start = node.next;
            node.val
        })
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.start.as_ref().map(|node| &node.val)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.start.as_mut().map(|node| &mut node.val)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.start.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.remove()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.start.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &**node);
            &node.val
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.start.as_mut().map(|node| &mut **node),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node
                .next
                .as_mut()
                .map::<&mut Node<T>, _>(|node| &mut **node);
            &mut node.val
        })
    }
}

#[test]
fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.add(1);
    list.add(2);
    list.add(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
}

#[test]
fn into_iter() {
    let mut list = List::new();
    list.add(1);
    list.add(2);
    list.add(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
    let mut list = List::new();
    list.add(1);
    list.add(2);
    list.add(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

#[test]
fn iter_mut() {
    let mut list = List::new();
    list.add(1);
    list.add(2);
    list.add(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}
