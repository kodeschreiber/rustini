use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, BufWriter};
use std::collections::HashMap;


pub struct INI {
  pub data: HashMap<String, Vec<String>>,
  pub prefix: String,
  pub delim: char,
}


impl INI {
  pub fn new() -> Self {
    Self { data: HashMap::new(), prefix: String::new(), delim: '=' }
  }
  
  pub fn get_kvp(&self, key: &String) -> HashMap<String, String> {
    let mut ret: HashMap<String, String> = HashMap::new();
    
    if !self.data.contains_key(key) {
      panic!("Could not locate key {}", key);
    }
    
    let mut block = self.data.get(key).unwrap().clone();
    block.retain(|a| a.len() > 0);  // Remove empty lines
    block.retain(INI::is_blank);    // Remove blank lines
    block.retain(INI::is_comment);  // Remove comments
    
    for line in block {
      let didx = line.chars().position(|c| c == self.delim).unwrap();
      let key = String::from(line[0..didx].trim());
      let val = String::from(line[didx+1..].trim());
      ret.insert(key, val);
    }
    
    ret
  }
  
  pub fn set_kvp(&mut self, key: &String, val: HashMap<String, String>) {    
    let mut block: Vec<String> = Vec::new();
    
    for (k, v) in val.iter() {
      block.push(format!("{} {} {}", k, self.delim, v));
    }
    
    self.data.entry(key.clone()).or_insert(block);
  }
  
  pub fn load(&mut self, ini_path: &str) {
    let path = Path::new(ini_path);
    let file = match File::open(&path) {
      Err(why) => panic!("Couldn't open {}: {}", ini_path, why),
      Ok(file) => file,
    };
    
    let buffer = BufReader::new(file).lines();
    let mut title: String = String::new();
    let mut block: Vec<String> = Vec::new();
    let mut eidx: usize;
    let mut lineno: usize = 0;
    let mut is_title: bool;
    
    for line in buffer {
      lineno += 1;
      if let Ok(data) = line {
        if data.len() == 0 {
          continue;
        }
        
        is_title = match data.chars().nth(0).unwrap() {
          '[' => true,
          _ => false,
        };
        
        if is_title {
          block.push(data);
        } else {
          eidx = match data.chars().position(|c| c == ']') {
            Some(idx) => idx,
            None => panic!("Found opening bracket without matching brace: Line {}", lineno),
          };
          
          if title.len() > 0 {  // Currently processing a block => Save block
            self.data.entry(title.clone()).or_insert(block.clone());
          }
          
          title.clear();
          title.push_str(data.to_string()[1..eidx].trim());
          block.clear();
        }
      }
    }
    
    if title.len() > 0 {
      self.data.entry(title.clone()).or_insert(block.clone());
    }
  }
  
  pub fn save(&self, ini_path: &str) {
    let path = OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(ini_path)
                            .unwrap();
    let mut buffer = BufWriter::new(path);
    
    for (title, block) in self.data.iter() {
      buffer.write_all(b"[").unwrap();
      buffer.write_all(title.as_bytes()).unwrap();
      buffer.write_all(b"]\n").unwrap();
      for line in block {
        buffer.write_all(line.as_bytes()).unwrap();
        buffer.write_all(b"\n").unwrap();
      }
    }
  }
  
  fn is_blank(check: &String) -> bool {
    for c in check.chars() {
      if c == ' ' || c == '\t' {
        continue;
      }
      
      return c != '\n';
    }
    
    false
  }
  
  fn is_comment(check: &String) -> bool {
    for c in check.chars() {
      if c == ' ' || c == '\t' {
        continue;
      }
      
      return c != '#';
    }
    
    false
  }
}
