#![allow(dead_code)]

use core::fmt;

use super::stack_iter::StackIter;

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
        let mut stack = Stack { items: Vec::new() };

        for item in iter {
            stack.items.push(item);
        }

        stack
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<T: std::clone::Clone> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
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
            .rev()
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

    pub fn iter(&self) -> StackIter<'_, T> {
        StackIter {
            iter: self.items.iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn should_be_lifo() {
        let mut stack = Stack::from_iter([1, 2, 3]);

        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 1);
        assert!(stack.is_empty());
    }

    #[test]
    fn should_pop_many() {
        let mut stack = Stack::from_iter([1, 2, 3, 4, 5]);

        assert_eq!(stack.pop_many(0), vec![]);
        assert_eq!(stack.pop_many(3), vec![5, 4, 3]);
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn should_pop_many_as_many_as_can() {
        let mut stack = Stack::from_iter([1, 2, 3, 4, 5]);

        assert_eq!(stack.pop_many(10), vec![5, 4, 3, 2, 1]);
        assert!(stack.is_empty());
    }

    #[test]
    fn iter_should_be_lifo() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5];

        let stack_from_input = Stack::from_iter(input.clone());

        let mut output_from_stack = Vec::from_iter(stack_from_input.iter().map(|number| *number));
        output_from_stack.reverse();

        assert_eq!(input, output_from_stack);
    }
}
