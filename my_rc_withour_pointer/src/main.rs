use crate::my_rc::MyRc;

mod my_rc;

fn main() {
    let a = MyRc::new(5);
    let b = MyRc::clone(&a);
    assert_eq!(MyRc::strong_count(&b), 2);
    // println!("{}", *a);
    // println!("{}", MyRc::strong_count(&b));
    assert_eq!(5, *a);
    assert_eq!(MyRc::strong_count(&a), 2);
}
