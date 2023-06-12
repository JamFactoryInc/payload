enum Node<'a, T> {
    Leaf(&'a T, usize),
    Root(&'a T, &'a Node<'a, T>, &'a Node<'a, T>),
    Branch(&'a T, &'a Node<'a, T>, &'a Node<'a, T>),
}

struct BinTree<'a, T> {
    root: &'a Node<'a, T>,
}
struct BinTreeIter<'a, T> {
    root: &'a Node<'a, T>,
    cursor: &'a Node<'a, T>,
}

impl<'a, T> IntoIterator for BinTree<'a, T> {
    type Item = T;
    type IntoIter = BinTreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return BinTreeIter {
            root: self.root,
            cursor: self.root,
        }
    }
}
impl<'a, T> Iterator for BinTreeIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor {
            Node::Leaf(val, depth) => {
                self.cursor = self.root;
            },
            Node::Branch(val, left, right) => {
                
            },
            Node::Root(val, left, right) => {
                
            }
        }
        todo!()
    }
}