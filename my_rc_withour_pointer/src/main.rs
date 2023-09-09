use crate::my_rc::MyRc;

mod my_rc;

fn main() {
    let a = MyRc::new(5);
    let b = MyRc::clone(&a);
    {
        let c = MyRc::clone(&b);
        assert_eq!(5, *c);
        assert_eq!(MyRc::strong_count(&a), 3);
    }
    assert_eq!(5, *a);
    assert_eq!(MyRc::strong_count(&a), 2);
    assert_eq!(MyRc::strong_count(&b), 2);
}
