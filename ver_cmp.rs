fn ver_cmp(v1: &str, v2: &str) -> i32 {
  println!("{}",v1);
  println!("{}",v2);
  let strings: Vec<_> = v1.split(".").collect();
  println!("{:?}", strings);
  let numbers: Result<Vec<i32>, _> = strings.iter().map(|x| x.parse()).collect();
  println!("{:?}", numbers);
  return 0;
}

fn main() {
  let v1 = "1.1";
  let v2 = "1.2";
  let res = ver_cmp(v1,v2);
  println!("{}",res);
}
