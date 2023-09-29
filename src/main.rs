use std::ops::Deref;

use crate::List::*;

enum List {
    // List is a recursive type, so we store it in a heap since we can't determine its size by compile time
    Cons(i32, Box<List>),
    Nil
}

impl List {
    fn new() -> List {
        Nil
    }

    fn tail(&mut self) -> &mut List {
        let mut node = self;

        while let Cons(_, tail) = node {
            node = tail;
        }

        node
    }

    fn append(mut self, item: i32) -> List {
        let tail = self.tail();
        *tail = Cons(item, Box::new(Nil));

        self
    }

    fn iter(&self) -> IterList {
        IterList {
            current: &self
        }
    }
}

struct IterList<'a> {
    current: &'a List
}

impl <'a> Iterator for IterList<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Cons(num, next) = self.current {
            self.current = next;
            
            return Some(*num)
        }

        None
    }
}

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let mut list = List::new();
    list = list.append(1)
        .append(2)
        .append(3)
        .append(4);

    let cons_list_iter = list.iter();

    let my_box = MyBox::new(5);  // custom implementation of box, stored on the stack

    for num in cons_list_iter {
        println!("{num}")
    }

    println!("{}", 5 == *my_box);
}
