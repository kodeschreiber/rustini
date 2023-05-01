use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::{self, BufRead, BufWriter};
use std::collections::HashMap;
use std::io::prelude::*;

enum INIMode {
  Undefined,
  Title,
  Block,
}

pub struct INI {
  pub path: String,
  pub data: HashMap<String, Vec<String>>,
}


impl INI {
  pub fn new(path: &str) -> Self {
    Self { path: String::from(path), data: HashMap::new() }
  }

  pub fn get_kvp(&self, key: &String, delim: Option<char>) -> HashMap<&str, &str> {
    let mut ret: HashMap<&str, &str> = HashMap::new();
    
    if !self.data.contains_key(key) {
      panic!("Could not locate key {}", key);
    }
    
    let delimiter: char = match delim {
      Some(d) => d,
      None => '=',
    };
    
    let block = self.data.get(key).unwrap();
    
    for line in block {
      let didx = line.chars().position(|c| c == delimiter).unwrap();
      let key = line[0..didx].trim();
      let val = line[didx+1..].trim();
      ret.insert(key, val);
    }
    
    ret
  }
  
  pub fn set_kvp(&mut self, key: &String, val: HashMap<&str, &str>, delim: Option<char>) {
    let delimiter: char = match delim {
      Some(d) => d,
      None => '=',
    };
    
    let mut block: Vec<String> = Vec::new();
    
    for (k, v) in val.iter() {
      block.push(format!("{} {} {}", k, delimiter, v));
    }
    
    self.data.entry(key.clone()).or_insert(block);
  }
  
  pub fn load(&mut self) {
    let path = Path::new(self.path.as_str());
    let display = path.display();
    let file = match File::open(&path) {
      Err(why) => panic!("Couldn't open {}: {}", display, why),
      Ok(file) => file,
    };
    
    let buffer = io::BufReader::new(file).lines();
    let mut title: String = String::new();
    let mut block: Vec<String> = Vec::new();
    let mut eidx: usize;
    let mut lineno: usize = 0;
    let mut mode: INIMode;
    
    for line in buffer {
      lineno += 1;
      if let Ok(data) = line {
        if data.len() == 0 {
          continue;
        }
        
        mode = match data.chars().nth(0).unwrap() {
          '[' => INIMode::Title,
          _ => INIMode::Block,
        };
        
        if let INIMode::Block = mode {
          block.push(data);
        } else if let INIMode::Title = mode {
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
  
  pub fn save(&self) {
    let path = OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(self.path.as_str())
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
}
