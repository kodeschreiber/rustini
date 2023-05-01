mod ini;

use std::collections::HashMap;

fn main() {
  let mut test = ini::INI::new();
  test.load("test.ini");
  
  for (title, block) in test.data.iter() {
    println!("[ {} ]", title);
    for line in block {
      println!("{}", line);
    }
  }
  
  let dict: HashMap<String, String> = test.get_kvp(&String::from("kvp"));
  
  println!("\nPrinting Key-Value Pairs:");
  for (key, val) in dict.iter() {
    println!("{} = {}", key, val);
  }
}
