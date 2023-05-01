```

                                                                                                    
                 ..:::::::..                      ..........::........::.....                       
            :!JPGBBBB###BGGGPP5YJ7!~!7?777!!7JY55PPPPGGPGPPPPPPPPPPPGGBGPPP5555YYJ7~:.              
        .!YG#&&#BB##BBBBBBGGGGGBB#BB###B#BBBBBBGGGGGGBGGGGGPPPPPPPGGGGGGBGPPPPPGPGGGGP5J!:          
      ^?G&&&##BBGGGBBBBBGGGGPGGGBP5PBBBGGBGGPPPPPPGGP5PGGPPGPPPPPPPPPGGGPPPP5PPPPPPPPGBBBG57:       
     7B######BGGGGPG  ______   _    _   ______  _______ _____  ______  _____  PGPPPG5YPGGGP!      
   .5##BBBBBGGGGPGPP | |  | \ | |  | | / |        | |    | |  | |  \ \  | |   PGGP5YJY5BPGGB7     
   !###BPPPGPGGPPPPG | |__| | | |  | | '------.   | |    | |  | |  | |  | |   PPPGYJ?YPGPPPPBG^    
   ^GB##BG5PPPPPPPPP |_|  \_\ \_|__|_|  ____|_/   |_|   _|_|_ |_|  |_| _|_|_  GPJ?JYPPGGGPPPG~    
    !5GBBBGGPPPPPGGG                                                          JJJY5PGPPPPGGGB~    
     ^?5GBGGPGPPPGGGGBBGGGBG5PGGGGGGGGGGGGBBP??Y5PGGGGGGGGGBBPYJ?JPPGGGGGGGGGP5JYY555PPP55PPGGY.    
       ^7Y5555PPGGGGGBBBBBBBBBBGGGGGGBGGGGP555PPPPPPPPPPGGPP5YJY5555555P5555YYJJJJYYYYYYY55P57      
          :^~!?JY5PPGBBBBBBBBBBBBBBGPPP55J??JJJJYYJJJYYYYYJ???7??????J????7!!7777???JJJJJY?!.       
               .:~!7?JYY5PGGGGGP55J??7!77~~~~~~~~~~^^^^:::::::::::::::........:::^^~~!~~^.          
                        ..:::::...                                                                  

```

## Introduction
Rustini is an INI parser for RUST. You can:
- Parse INI files into a HashMap in the INI struct
- Write a HashMap to a file
- Parse a block of data from a tag as Key-Value pairs
- Set Key-Value pairs as blocks within the HashMap

The Structure of the INI is:
- data: HashMap<String, String>
- prefix: String
- delim: char

`prefix` can be used to set a custom key prefix for elements that 
are loaded from an INI file. `delim` allows you to choose the 
delimiter for parsing key-Value pairs (defaults to `'='`)

## Building
Simply run `make`. Cargo will build the library, documentation, and tests.

## Documentation
After building this package, you can open the documentation in your browser. 
The easiest place to start with will be: `./target/doc/rustini/index.html`

## Additional Notes:
- 'Rustini' is a play on 'Crostini' (Hence the bread-like ASCII-art)
- Future Improvement(?): Cross compile support
