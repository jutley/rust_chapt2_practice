const CELS_TO_FAHR_OFFSET: f32 = 32.0;
const CELS_TO_FAHR_SCALE:  f32 = 1.8;

fn fahrenheit_to_celsius(deg_fahr: f32) -> f32 {
  (deg_fahr - CELS_TO_FAHR_OFFSET) / CELS_TO_FAHR_SCALE
}

fn celsius_to_fahrenheit(deg_cels: f32) -> f32 {
  (deg_cels * CELS_TO_FAHR_SCALE) + CELS_TO_FAHR_OFFSET
}

pub fn test() {

  //     c       f
  // -40.0   -40.0
  //   0.0    32.0
  // 100.0   212.0

  let c_to_f_1 = -40.0;
  let c_to_f_2 = 0.0;
  let c_to_f_3 = 100.0;
  let f_to_c_1 = -40.0;
  let f_to_c_2 = 32.0;
  let f_to_c_3 = 212.0;

  println!("{} should be {}", celsius_to_fahrenheit(c_to_f_1), f_to_c_1);
  println!("{} should be {}", celsius_to_fahrenheit(c_to_f_2), f_to_c_2);
  println!("{} should be {}", celsius_to_fahrenheit(c_to_f_3), f_to_c_3);
  println!("{} should be {}", fahrenheit_to_celsius(f_to_c_1), c_to_f_1);
  println!("{} should be {}", fahrenheit_to_celsius(f_to_c_2), c_to_f_2);
  println!("{} should be {}", fahrenheit_to_celsius(f_to_c_3), c_to_f_3);

}
