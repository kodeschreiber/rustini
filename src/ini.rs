use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, BufWriter, ErrorKind};
use std::io::ErrorKind::NotFound;
use std::collections::HashMap;

/// The fields can be modified before using their relative function calls
/// See 'load()' for 'prefix and 'get/set_kvp()' for 'delim', respectively.
pub struct INI {
  pub data: HashMap<String, Vec<String>>,
}


impl INI {
  pub fn new() -> Self {
    Self { data: HashMap::new() }
  }
  
  pub fn get(&self, key: &String) -> Result<Vec<String>,ErrorKind> {
    match self.data.get(key) {
      Some(val) => Ok(val.clone()),
      None => Err(NotFound),
    }
  }
  
  // Replaces orginal vector; unable to update the old vector due
  // to mutability problems.
  pub fn set(&mut self, key: &String, val: Vec<String>) {
    self.data.insert(key.to_string(), val.clone());
  }
  
  /// Default delimiter is '='
  pub fn get_kvp(&self, key: &String, delimiter: Option<char>) -> HashMap<String, String> {
    if !self.data.contains_key(key) {
      panic!("Could not locate key {}", key);
    }
    
    let delim: char = match delimiter {
      Some(val) => val.clone(),
      None => '=',
    };
    
    let mut ret: HashMap<String, String> = HashMap::new();
    
    let mut block = self.data.get(key).unwrap().clone();
    block.retain(|a| a.len() > 0);  // Remove empty lines
    block.retain(INI::is_blank);    // Remove blank lines
    block.retain(INI::is_comment);  // Remove comments
    
    for line in block {
      let didx = match line.chars().position(|c| c == delim) {
        Some(d) => d,
        None => {
          println!("Could not locate delimiter in line: {}", line);
          continue;
        }
      };
      
      let key = String::from(line[0..didx].trim());
      let val = String::from(line[didx+1..].trim());
      ret.insert(key, val);
    }
    
    ret
  }
  
  /// Default delimiter is '='
  pub fn set_kvp(&mut self, key: &String, val: HashMap<String, String>, delimiter: Option<char>) {    
    let mut block: Vec<String> = Vec::new();
    
    let delim: char = match delimiter {
      Some(val) => val.clone(),
      None => '=',
    };
    
    for (k, v) in val.iter() {
      block.push(format!("{} {} {}", k, delim, v));
    }
    
    self.data.insert(key.clone(), block);
  }
  
  /// Be sure to add an underscore if desired for 'prefix'.
  pub fn load(&mut self, ini_path: &str, prefix: Option<String>) -> Result<(), String> {
    let path = Path::new(ini_path);
    let file = match File::open(&path) {
      Err(why) => panic!("Couldn't open {}: {}", ini_path, why),
      Ok(file) => file,
    };
    
    let pre: String = match prefix {
      Some(val) => val.clone(),
      None => String::new(),
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
        
        if !is_title {
          block.push(data);
        } else {
          eidx = match data.chars().position(|c| c == ']') {
            Some(idx) => idx,
            None => Err(format!("Found opening bracket without matching bracket: Line {}", lineno).to_string())?,
          };
          
          if title.len() > 0 {  // Currently processing a block => Save block
            self.data.entry(pre.clone() + title.as_str()).or_insert(block.clone());
          }
          
          block.clear();
          title.clear();
          title.push_str(data.to_string()[1..eidx].trim());
        }
      }
    }
    
    if title.len() > 0 {
      self.data.entry(pre.clone() + title.as_str()).or_insert(block.clone());
    }
    
    Ok(())
  }
  
  pub fn save(&self, ini_path: &str) {
    let path = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(ini_path)
                            .unwrap();
    let mut buffer = BufWriter::new(path);
    
    for (title, block) in self.data.iter() {
      buffer.write_all(b"[").unwrap();
      buffer.write_all(title.as_bytes()).unwrap();
      buffer.write_all(b"]\n").unwrap();
      for line in block {
        buffer.write_all(line.as_bytes()).unwrap();
        buffer.write_all(b"\n\n").unwrap();
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
