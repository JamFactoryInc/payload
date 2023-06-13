use std::ops::Deref;
use std::thread::current;

struct Stack<T> {
    top: Option<*const StackNode<T>>,
    current: StackNode<T>,
    cursor: usize
}

struct StackNode<T> {
    contents: [Option<T>; 8],
    next: Option<*const StackNode<T>>
}

impl<T> Stack<T> {
    fn pop(&mut self) -> T {
        todo!();
    }

    fn new_stack_node() -> *mut StackNode<T> {
        &mut StackNode {
            contents: [None, None, None, None, None, None, None, None],
            next: None
        } as *mut StackNode<T>
    }
    fn push_mov(&mut self, val: T) -> Self<> {
        if self.cursor >= 8 {
            Self::new_stack_node();

            match &mut self.top {
                Some(top) => {
                    self.current.next = Some(*top);
                    self.top = Some(&self.current as *const StackNode<T>);
                    self.current = StackNode {
                        contents: [None, None, None, None, None, None, None, None],
                        next: None
                    }
                },
                None => todo!()
            }

        }
        todo!()
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        self
    }
}

struct Residual<T> {
    owner: *const Stack<T>
    resident : Option<T>,
    refugee: Option<T>
}
impl<T> Drop for Residual<T> {
    fn drop(&mut self) {
        if let Some(resident) = self.resident.take() {

        }
    }
}

#[test]
fn test() {

}