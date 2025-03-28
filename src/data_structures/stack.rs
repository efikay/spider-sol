#![allow(dead_code)]

use core::fmt;

#[derive(Debug, Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "┌─────── Stack ───────┐")?;

        for (i, item) in self.items.iter().rev().enumerate() {
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
            items: Vec::new(),
        };

        for item in iter {
            c.items.push(item);
        }

        c
    }
}

impl<T: std::clone::Clone> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn pop_many(&mut self, desired_amount: usize) -> Vec<T> {
        let amount = if self.items.len() < desired_amount {
            self.items.len()
        } else {
            desired_amount
        };

        self.items
            .drain(self.items.len() - amount..)
            .collect()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear()
    }
}
