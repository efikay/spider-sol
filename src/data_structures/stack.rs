#![allow(dead_code)]

use core::fmt;

#[derive(Debug, Clone)]
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "┌─────── Stack ───────┐")?;

        for (i, item) in self.elements.iter().rev().enumerate() {
            if i == 0 {
                write!(f, "│ → {:17} │", item)?; // Arrow for top element
            } else {
                write!(f, "│   {:17} │", item)?;
            }
            writeln!(f)?;
        }

        write!(f, "└─────────────────────┘")
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = Stack {
            elements: Vec::new(),
        };

        for item in iter {
            c.elements.push(item);
        }

        c
    }
}

impl<T: std::clone::Clone> Stack<T> {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn clear(&mut self) {
        self.elements.clear()
    }
}
