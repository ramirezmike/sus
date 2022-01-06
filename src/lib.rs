/// Wraps macro input with an unsafe block. That's it. That's all it does.
///
/// # Example
///
/// ```
/// # extern crate sus;
/// # use sus::sus;
/// fn dangerous() {}
/// 
/// sus! {
///     dangerous()
/// }
/// ```
#[macro_export]
macro_rules! sus {
    ($($body:tt)*) => {
        unsafe {
            $($body)*
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::sus;

    unsafe fn dangerous() {}

    #[test]
    fn sus_with_parenthesis() {
        sus!(dangerous());
    }

    #[test]
    fn sus_with_curly_brace() {
        sus! {
            dangerous()
        };
    }

    #[test]
    fn pointer_dereference() {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        sus! {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }
}

