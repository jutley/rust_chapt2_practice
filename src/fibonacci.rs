fn fibonacci(n: u32) -> u32 {
  match n {
    0 => 0,
    1 => 1,
    n => fibonacci(n - 2) + fibonacci(n - 1)
  }
}

fn test_case(test_val: u32, expected: u32) {
  let res = fibonacci(test_val);
  println!("{} should be {}: {}", res, expected, res == expected);
}

pub fn test() {
  test_case(0, 0);
  test_case(1, 1);
  test_case(2, 1);
  test_case(3, 2);
  test_case(4, 3);
  test_case(5, 5);
  test_case(10, 55);
  test_case(20, 6765);
}