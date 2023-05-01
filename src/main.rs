mod ini;

fn main() {
  let mut test = ini::INI::new("test.ini");
  test.load();
  
  for (title, block) in test.data.iter() {
    println!("[ {} ]", title);
    for line in block {
      println!("{}", line);
    }
  }
}
