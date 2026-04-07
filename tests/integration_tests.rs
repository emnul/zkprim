use zkprim::field::Fp;

#[test]
fn field_element_is_accessible() {
    let a = Fp::new(7);
    assert_eq!(a.value, 7);
}
