pub mod ini;

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use std::path::Path;
  use std::fs::remove_file;

  use crate::ini::INI;

  #[test]
  fn load() {
    let mut file: INI = INI::new();
    file.load("test/normal.ini", None).unwrap();
    
    assert_eq!(file.data.keys().len(), 4);
  }
  
  #[test]
  fn access() {
    let mut file: INI = INI::new();
    file.load("test/normal.ini", None).unwrap();
    
    let block: &Vec<String> = file.data.get(&"simple".to_string()).unwrap();
    
    assert_eq!(block.first(), Some(&"basic stuff".to_string()));
  }
  
  #[test]
  fn kvp() {
    let mut ini: INI = INI::new();
    ini.load("test/normal.ini", None).unwrap();
    
    let chosen_key: String = String::from("dict_test");
    let mut dict: HashMap<String, String> = ini.get_kvp(&chosen_key, None);
    
    assert_eq!(dict.keys().len(), 4);
    
    for (k, v) in dict.iter() {
      assert!(k.len() > 0);
      assert!(v.len() > 0);
    }
    
    assert_eq!(dict.get(&"key1".to_string()), Some(&"val1".to_string()));
    
    dict.insert("stuff".to_string(), "things".to_string());
    
    ini.set_kvp(&chosen_key, dict, None);
    
    let new_dict: HashMap<String, String> = ini.get_kvp(&chosen_key, None);
    
    assert_eq!(new_dict.get(&"stuff".to_string()), Some(&"things".to_string()));
  }
  
  #[test]
  fn save() {
    let mut file: INI = INI::new();
    file.load("test/normal.ini", None).unwrap();
    
    file.save("test/saved.ini");
    
    assert!(Path::new("test/saved.ini").exists());
    
    remove_file("test/saved.ini").unwrap();
  }
  
  #[test]
  fn bad_title() {
    let mut file: INI = INI::new();
    file.load("test/bad_title.ini", None).unwrap();
    
    assert!(!file.data.contains_key(&"Bad Title".to_string()));
  }
  
  #[test]
  fn missing_end() {
    let mut file: INI = INI::new();
    match file.load("test/missing_end.ini", None) {
      Ok(()) => panic!("Internal filters did not catch the last bracket"),
      Err(_) => (),
    }
  }
  
  #[test]
  fn setter_getter() {
    let mut ini: INI = INI::new();
    ini.load("test/normal.ini", None).unwrap();
    
    let key = String::from("dict_test");
    let mut block = ini.get(&key).unwrap();
    
    block.push(String::from("added_key = new"));
    
    ini.set(&key, block);
    
    ini.save("new.ini");
    
    let mut ini2: INI = INI::new();
    ini2.load("new.ini", None).unwrap();

    let mut block2 = ini.get(&key).unwrap();
        
    assert!(block2.last().unwrap().starts_with("added_key = new"));
    
    remove_file("new.ini");
  }
}
