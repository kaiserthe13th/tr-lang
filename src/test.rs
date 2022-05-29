use crate::{make_function, ffi::{Context, FfiError, FfiFunction}};

#[test]
fn test_make_function() {
    fn my_fn(_ctx: Context) -> Result<(), FfiError> { Ok(()) }
    let left: *const _ = make_function!(my_fn).fun.as_ref();
    let right: *const _ = FfiFunction {
        fun: Box::new(my_fn)
    }.fun.as_ref();
    assert_eq!(left, right);
}

