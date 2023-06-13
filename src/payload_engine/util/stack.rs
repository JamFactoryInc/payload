use std::ops::Deref;

struct Stack<'a, T> {
    temp_top: &'a mut StackNode<'a, T>,
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
        todo!();
    }

    fn get_new_stack(&mut self) {
        self.temp_top = &mut StackNode {
            contents: [None, None, None, None, None, None, None, None],
            next: None
        }
    }
    fn push_mov(&mut self, val: T) {
        if self.cursor >= 8 {
            self.get_new_stack();

            match &mut self.top {
                Some(top) => {
                    std::mem::swap(*top, self.());
                },
                None => todo!()
            }

        }
        todo!()
    }
}

#[test]
fn test() {

}