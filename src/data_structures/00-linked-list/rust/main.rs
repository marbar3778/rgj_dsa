
pub struct List<T> {
  head: Link<T>,
  tail: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  value: T,
  next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn append(&mut self, value: T) {
    let new_node = Box::new(Node {
      value: value,
      next: None
    });
    let old_tail = mem::replace(&mut self.tail, Some(new_node))
  match old_tail {
    Some(mut old_tail) => {
      old_tail.next = Some(new_node);
    }
    None => {
      self.head = new_node;
    }
  }
  }

  pub fn deleteHead(&mut self) -> Option<T> {
    self.head.take().map(|node| {
      self.head = node.next;
      node.value
    })
  }

  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| {
      &node.value
    })
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| {
      &mut node.value
    })
  }

  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }

  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Iter { next: self.head.as_ref().map::<&Node<T>, _>(|node| &node) }
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    IterMut { next: self.head.as_mut().map(|node| &mut **node) }
  }
}

pub trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
      &node.value
  })
  }
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.delete()
  }
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| {
        self.next = node.next.as_mut().map(|node| &mut **node);
        &mut node.value
    })
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut cur_link = self.head.take();
    while let Some(mut boxed_node) = cur_link {
      cur_link = boxed_node.next.take();
    }
  }
}

#[test]
fn append() {
  let mut list = List::new();
  assert_eq!(list.peek(), None);
  list.append(1); list.append(2);
  assert_eq!(list.peek(), Some(&2));
}

#[test]
fn into_iter() {
  let mut list = List::new();
  list.append(1);list.append(2);list.append(3);

  let mut iter = list.into_iter();
  assert_eq!(iter.next(), Some(3));
  assert_eq!(iter.next(), Some(2));
  assert_eq!(iter.next(), Some(1));
  assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
  let mut list = List::new();
  list.append(1);list.append(2);list.append(3);

  let mut iter = list.iter();
  assert_eq!(iter.next(), Some(&3));
  assert_eq!(iter.next(), Some(&2));
  assert_eq!(iter.next(), Some(&1));
}

#[test]
fn iter_mut() {
    let mut list = List::new();
  list.append(1);list.append(2);list.append(3);

  let mut iter = list.iter_mut();
  assert_eq!(iter.next(), Some(&mut 3));
  assert_eq!(iter.next(), Some(&mut 2));
  assert_eq!(iter.next(), Some(&mut 1));
}