fn calculate_apple_price(apple_count: i32) -> i32 {
if apple_count > 40 {
    return apple_count
}
apple_count * 2
}

#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
