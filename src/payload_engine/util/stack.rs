struct Stack<'a, T> {
    top: Option<&'a mut StackNode<'a, T>>,
    current: StackNode<'a, T>,
    cursor: usize
}

struct StackNode<'a, T> {
    contents: [Option<T>; 8],
    next: Option<&'a StackNode<'a, T>>
}

impl<'a, T> Stack<'a, T> {
    fn pop(&mut self) -> T {
        todo!()
    }
    fn push(&mut self, val: T) {
        if self.cursor >= 8 {
            let new_top = &mut StackNode {
                contents: [None, None, None, None, None, None, None, None],
                next: None
            };
            match &mut self.top {
                Some(top) => {
                    std::mem::swap(*top, new_top);
                    top.next = Some(new_top)
                },
                None => todo!()
            }




            todo!()
        }
    }
}

#[test]
fn test() {

}