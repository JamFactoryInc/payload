use std::collections::linked_list;
use std::mem;
use crate::payload_engine::lexer::token::Token;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::payload_engine::lexer::lexer::TokenType;

#[cfg(test)]

pub fn random_token_stream<'a>(len: usize) -> Vec<Token<'a>> {
    let mut start = 0;
    let mut len = 0;

    //vec![rand_token(get_rand_token(start); len];

    // fn get_rand_token(&mut start : usize) -> Token {
    //     start += 1;
    //
    //     rand_token(len, start, token_type)
    // }
    //return vec![rand_token(5, 0, 0); 0];
    todo!()
}

// pub fn rand_token<'a>(len : usize, start: usize, token_type : TokenType) -> Token<'a> {
//
//     let text : Vec<u8> = rand::thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(len)
//         .collect();
//
//     Token {
//         text : &text,
//         token_type,
//         start,
//     }
// }

use std::marker::PhantomData;
use std::ptr::NonNull;
use std::time::Instant;
use crate::test::test_utils::token_utils::ReturnValue::{CONSUMED, OFF};

enum ReturnValue {
    CONSUMED,
    OFF
}

struct PatternMock {
    modval : usize
}

impl PatternMock {
    pub fn consume(&mut self, in_val : usize) -> ReturnValue {
        return if in_val == self.modval {
            OFF
        } else {
            CONSUMED
        }
    }
}

#[test]
pub fn test() {
    let mut pattern_list : CustomLinkedList<PatternMock> = CustomLinkedList::new();
    let mut off_list : CustomLinkedList<PatternMock> = CustomLinkedList::new();

    for i in 0..100 {
        pattern_list.push_back(PatternMock {
            modval : i
        });
    }

    // for p in pattern_list {
    //     println!()
    // }

    println!("{}", pattern_list.len);

    let before = Instant::now();
    let mut total_iters = 0;

    for _ in 0..1 {
        for i in 0..100 {
            let mut iter = pattern_list.iter_mut();
            while let Some(node) = iter.next() {
                unsafe {
                    total_iters += 1;
                    match (*node.as_ptr()).element.consume(i) {
                        CONSUMED => (),
                        OFF => {
                            println!("Matched {}", i);
                            iter.pop_into(&mut off_list);
                        },
                    }
                }
            }
        }

        println!("Old sizes: {} {}", pattern_list.len, off_list.len);
        pattern_list.append(&mut off_list);
        off_list.clear();
        println!("New sizes: {} {}", pattern_list.len, off_list.len);
    }

    let after = Instant::now();

    println!("elapsed: {}ms", after.duration_since(before).as_millis());
    println!("total iters: {}", total_iters);
}

// pub struct Node<T> {
//     next : Option<Box<Node<T>>>,
//     element : T,
// }
//
// impl<T> Node<T> {
//     pub fn new(val : T) -> LinkedPatterns<T> {
//         let node = Node {
//             next : None,
//             element : val,
//         };
//         LinkedPatterns {
//             head: None,
//             tail: None,
//             len: 0,
//         }
//     }
//
//     pub fn link(&mut self, val: T) {
//         self.next = Some(Box::new(Node {
//             next: None,
//             element: val,
//         }))
//     }
// }

// pub struct LinkedPatterns<T> {
//     head: Option<Node<T>>,
//     tail: Option<Node<T>>,
//     len: usize,
// }
//
// impl<T> LinkedPatterns<T> {
//     #[inline]
//     pub fn add_last(self, val : T) {
//         self.tail.unwrap().link(val);
//     }
// }
//
// struct LinkedPatternsIntoIter<T> {
//     linked : LinkedPatterns<T>,
//     index : usize,
//     marker: Option<Box<Node<T>>>,
// }
//
// impl<T> IntoIterator for LinkedPatterns<T> {
//     type Item = Node<T>;
//     type IntoIter = LinkedPatternsIntoIter<T>;
//
//     fn into_iter(mut self) -> Self::IntoIter {
//         LinkedPatternsIntoIter {
//             marker : Some(Box::new(self.head)),
//             linked: self,
//             index: 0,
//         }
//     }
// }


// impl<T> Iterator for LinkedPatternsIntoIter<T> {
//     type Item = Node<T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let ret = self.marker.unwrap();
//         match self.marker.unwrap().next {
//             Some(val) => self.marker = Some(Box::new(*val)),
//             None => return None
//         }
//         self.marker = Some(self.marker.unwrap().next.unwrap());
//         Some(*ret)
//     }
// }

pub struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node { next: None, prev: None, element }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

pub struct CustomLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

struct CustomNode<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a mut Node<T>>,
    source: &'a mut CustomLinkedList<T>,
}

impl<'a ,T> IterMut<'a, T> {
    unsafe fn pop_into(&mut self, other_list: &mut CustomLinkedList<T>) {

        self.head.map(|head| {
            self.source.unlink_node(head);
            other_list.link_node(head);
        });
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = NonNull<Node<T>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                // Need an unbound lifetime to get 'a
                // let node = &mut *node.as_ptr();
                self.len -= 1;
                self.head = (*node.as_ptr()).next;
                node
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> CustomLinkedList<T> {
    /// Creates an empty `LinkedList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::LinkedList;
    ///
    /// let list: LinkedList<u32> = LinkedList::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        CustomLinkedList { head: None, tail: None, len: 0, marker: PhantomData }
    }

    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => mem::swap(self, other),
            Some(mut tail) => {
                // `as_mut` is okay here because we have exclusive access to the entirety
                // of both lists.
                if let Some(mut other_head) = other.head.take() {
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                        other_head.as_mut().prev = Some(tail);
                    }

                    self.tail = other.tail.take();
                    self.len += mem::replace(&mut other.len, 0);
                }
            }
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { head: self.head, tail: self.tail, len: self.len, marker: PhantomData }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { head: self.head, tail: self.tail, len: self.len, marker: PhantomData, source: self}
    }


    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn clear(&mut self) {
        *self = Self::new();
    }


    #[inline]
    #[must_use]
    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.as_ref().element) }
    }

    #[inline]
    #[must_use]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.as_mut().element) }
    }

    #[inline]
    #[must_use]
    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.as_ref().element) }
    }

    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.as_mut().element) }
    }

    pub fn push_front(&mut self, elt: T) {
        self.push_front_node(Box::new(Node::new(elt)));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }

    pub fn push_back(&mut self, elt: T) {
        self.push_back_node(Box::new(Node::new(elt)));
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_element)
    }
}

// private methods
impl<T> CustomLinkedList<T> {
    /// Adds the given node to the front of the list.
    #[inline]
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        unsafe {
            node.next = self.head;
            node.prev = None;
            let node = Some(Box::leak(node).into());

            match self.head {
                None => self.tail = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    /// Removes and returns the node at the front of the list.
    #[inline]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = None,
            }

            self.len -= 1;
            node
        })
    }

    /// Adds the given node to the back of the list.
    #[inline]
    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        unsafe {
            node.next = None;
            node.prev = self.tail;
            let node = Some(Box::leak(node).into());

            match self.tail {
                None => self.head = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    /// Removes and returns the node at the back of the list.
    #[inline]
    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(tail) => (*tail.as_ptr()).next = None,
            }

            self.len -= 1;
            node
        })
    }

    /// Unlinks the specified node from the current list.
    ///
    /// Warning: this will not check that the provided node belongs to the current list.
    ///
    /// This method takes care not to create mutable references to `element`, to
    /// maintain validity of aliasing pointers.
    #[inline]
    unsafe fn unlink_node(&mut self, mut node: NonNull<Node<T>>) {
        let node = unsafe { node.as_mut() }; // this one is ours now, we can create an &mut.

        // Not creating new mutable (unique!) references overlapping `element`.
        match node.prev {
            Some(prev) => unsafe { (*prev.as_ptr()).next = node.next },
            // this node is the head node
            None => self.head = node.next,
        };

        match node.next {
            Some(next) => unsafe { (*next.as_ptr()).prev = node.prev },
            // this node is the tail node
            None => self.tail = node.prev,
        };

        self.len -= 1;
    }

    #[inline]
    unsafe fn link_node(&mut self, mut node : NonNull<Node<T>>) {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        unsafe {
            node.as_mut().next = None;
            node.as_mut().prev = self.tail;

            match self.tail {
                None => self.head = Some(node),
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(tail) => (*tail.as_ptr()).next = Some(node),
            }

            self.tail = Some(node);
            self.len += 1;
        }
    }

    /// Splices a series of nodes between two existing nodes.
    ///
    /// Warning: this will not check that the provided node belongs to the two existing lists.
    #[inline]
    unsafe fn splice_nodes(
        &mut self,
        existing_prev: Option<NonNull<Node<T>>>,
        existing_next: Option<NonNull<Node<T>>>,
        mut splice_start: NonNull<Node<T>>,
        mut splice_end: NonNull<Node<T>>,
        splice_length: usize,
    ) {
        // This method takes care not to create multiple mutable references to whole nodes at the same time,
        // to maintain validity of aliasing pointers into `element`.
        if let Some(mut existing_prev) = existing_prev {
            unsafe {
                existing_prev.as_mut().next = Some(splice_start);
            }
        } else {
            self.head = Some(splice_start);
        }
        if let Some(mut existing_next) = existing_next {
            unsafe {
                existing_next.as_mut().prev = Some(splice_end);
            }
        } else {
            self.tail = Some(splice_end);
        }
        unsafe {
            splice_start.as_mut().prev = existing_prev;
            splice_end.as_mut().next = existing_next;
        }

        self.len += splice_length;
    }

    /// Detaches all nodes from a linked list as a series of nodes.
    #[inline]
    fn detach_all_nodes(mut self) -> Option<(NonNull<Node<T>>, NonNull<Node<T>>, usize)> {
        let head = self.head.take();
        let tail = self.tail.take();
        let len = mem::replace(&mut self.len, 0);
        if let Some(head) = head {
            // SAFETY: In a LinkedList, either both the head and tail are None because
            // the list is empty, or both head and tail are Some because the list is populated.
            // Since we have verified the head is Some, we are sure the tail is Some too.
            let tail = unsafe { tail.unwrap_unchecked() };
            Some((head, tail, len))
        } else {
            None
        }
    }

    #[inline]
    unsafe fn split_off_before_node(
        &mut self,
        split_node: Option<NonNull<Node<T>>>,
        at: usize,
    ) -> Self {
        // The split node is the new head node of the second part
        if let Some(mut split_node) = split_node {
            let first_part_head;
            let first_part_tail;
            unsafe {
                first_part_tail = split_node.as_mut().prev.take();
            }
            if let Some(mut tail) = first_part_tail {
                unsafe {
                    tail.as_mut().next = None;
                }
                first_part_head = self.head;
            } else {
                first_part_head = None;
            }

            let first_part = CustomLinkedList {
                head: first_part_head,
                tail: first_part_tail,
                len: at,
                marker: PhantomData,
            };

            // Fix the head ptr of the second part
            self.head = Some(split_node);
            self.len = self.len - at;

            first_part
        } else {
            mem::replace(self, CustomLinkedList::new())
        }
    }

    #[inline]
    unsafe fn split_off_after_node(
        &mut self,
        split_node: Option<NonNull<Node<T>>>,
        at: usize,
    ) -> Self {
        // The split node is the new tail node of the first part and owns
        // the head of the second part.
        if let Some(mut split_node) = split_node {
            let second_part_head;
            let second_part_tail;
            unsafe {
                second_part_head = split_node.as_mut().next.take();
            }
            if let Some(mut head) = second_part_head {
                unsafe {
                    head.as_mut().prev = None;
                }
                second_part_tail = self.tail;
            } else {
                second_part_tail = None;
            }

            let second_part = CustomLinkedList {
                head: second_part_head,
                tail: second_part_tail,
                len: self.len - at,
                marker: PhantomData,
            };

            // Fix the tail ptr of the first part
            self.tail = Some(split_node);
            self.len = at;

            second_part
        } else {
            mem::replace(self, CustomLinkedList::new())
        }
    }
}