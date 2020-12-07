use mylib::adder::add_two;
#[test]
fn it_add_two(){
    assert_eq!(4, add_two(2));
}