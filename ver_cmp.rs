use std::env; // for argc,argv

// Compile: rustc ver_cmp.rs
// Run: ./ver_cmp 1.1 1.2

// converts list
fn ver_str_to_ints(v: &str) -> Vec<i16> {
  let strings: Vec<_> = v.split(".").collect();
  let numbers: Vec<i16> = strings.iter().flat_map(|x| x.parse()).collect();
  return numbers;
}

// compare 2 version strings of the form "1.1.2" "3.3"
// returns integer
// < 0 if v1<v2
// > 0 if v1>v2
// = 0 if v1==v2
fn ver_cmp(v1: &str, v2: &str) -> i16 {
  let n1 = ver_str_to_ints(v1);
  let n2 = ver_str_to_ints(v2);
  let len1 = n1.len();
  let len2 = n2.len();
  let minab = if len1 < len2 { len1 } else { len2 };
  let mut ii=0;
  while ii<minab {
    if n1[ii]!=n2[ii] {
      break
    } else {
      ii = ii+1
    }
  }
  if ii<minab {
    return n1[ii]-n2[ii]
  } else if ii==minab {
    if len1 >minab {
      return 1
    } else if len2 > minab {
      return -1
    } else {
      return n1[minab-1]-n2[minab-1]
    }
  }
  return 0;
}

// main starts here
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    println!("USAGE: ./this-script <version-string1> <version-string2>");
    std::process::exit(-1);
  }
  let v1 = &args[1];
  let v2 = &args[2];
  // v1 and v2 of type &str
  let res = ver_cmp(v1,v2);
  if res>0 {
    println!(" {} is greater than {} ",v1,v2)
  } else if res<0 {
    println!(" {} is less than {} ",v1,v2)
  } else {
    println!("both ({}),({}) are equal",v1,v2)
  }
}
