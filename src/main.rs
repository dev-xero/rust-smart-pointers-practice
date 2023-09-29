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

fn main() {
    let cons_list = List::new();
}
