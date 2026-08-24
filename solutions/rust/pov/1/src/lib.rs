use std::{fmt::Debug, iter::once, mem::swap, ptr::null_mut};

#[derive(Debug, Eq)]

pub struct Tree<T: Debug + PartialOrd + Ord + PartialEq + Eq> {
    key: T,
    children: Vec<Tree<T>>,
}

impl<T: Debug + PartialOrd + Ord + PartialEq + Eq> PartialEq for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.key != other.key {
            return false;
        }
        if self.children.len() != other.children.len() {
            return false;
        }
        if self.children.is_empty() {
            return true;
        }
        self.children.iter().all(
            |tree| match other.children.iter().find(|t| t.key == tree.key) {
                None => false,
                Some(t) => tree == t,
            },
        )
    }
}

impl<T: Debug + PartialOrd + PartialEq + Ord + Eq> Tree<T> {
    pub fn new(label: T) -> Self {
        Self {
            key: label,
            children: vec![],
        }
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(self, child: Self) -> Self {
        Self {
            key: self.key,
            children: self.children.into_iter().chain(once(child)).collect(),
        }
    }

    fn get_pov_from(&mut self, from: &T) -> (bool, *mut Tree<T>) {
        if self.key == *from {
            return (true, null_mut());
        }
        for i in 0..self.children.len() {
            let (has_from, new_parent) = self.children[i].get_pov_from(from);
            if has_from {
                if !new_parent.is_null() {
                    let mut t = self.children.swap_remove(i);
                    swap(self, &mut t);
                    unsafe {
                        (*new_parent).children.push(t);
                        return (true, (*new_parent).children.last_mut().unwrap());
                    }
                } else {
                    let mut t = self.children.swap_remove(i);
                    swap(self, &mut t);
                    self.children.push(t);
                    return (true, self.children.last_mut().unwrap());
                }
            }
        }
        (false, null_mut())
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        self.get_pov_from(from).0
    }

    fn find_key<'a>(&'a self, to: &'a T, path: &mut Vec<&'a T>) -> bool {
        path.push(&self.key);
        if self.key == *to {
            return true;
        }
        for child in &self.children {
            if child.find_key(to, path) {
                return true;
            }
        }
        path.pop();
        false
    }

    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if !self.pov_from(from) {
            return None;
        }
        let mut path = vec![];
        let found = self.find_key(to, &mut path);
        if found { Some(path) } else { None }
    }
}