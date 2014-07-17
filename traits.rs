fn print_vec<T: std::fmt::Show>(v: &[T]) {
  for i in v.iter() {
      println!("{}", i)
  }
}

fn main() {
    let vec = [1i, 2i, 3i];

    print_vec(vec);
}
