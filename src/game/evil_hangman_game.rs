use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

pub struct EvilHangmanGame {
    word_groups: Option<HashMap<String,Rc<HashSet<String>>>>,
    current_set: Option<String>,
    used_char: Option<BTreeSet<String>>,
}


impl EvilHangmanGame {
    pub fn new() -> Self {
        Self {word_groups: None, current_set: None, used_char:None}
    }


    pub fn start_game(&mut self,dictionary:&mut File,word_length:usize) -> Result<&str,&str> {
        self.word_groups = Some(HashMap::new());
        self.used_char = Some(BTreeSet::new());
        let mut blank_set = "".to_string();
        for _i in 0..word_length {
           blank_set.push('-'); 
        }
        self.current_set = Some(blank_set.clone());

        let mut file_contents:String = "".to_string();
        match dictionary.read_to_string(&mut file_contents) {
            Err(_) => return Err("Error occured while accessing dictionary"),
            Ok(_) => {}
        }

        let words: Vec<&str> = file_contents.split_whitespace().collect();

        /*if words.len() <= 1 {
            return Err("empty dictionary");
        }*/
       
        let mut temp_set: HashSet<String> = HashSet::new();
        for word in words {
            if word.len() == word_length {
                //println!("{}",word);
                temp_set.insert(word.to_lowercase().to_string());
            }
        }
        
        if temp_set.len() < 1 {
            return Err("empty dictionary");
        }
        self.word_groups.as_mut().unwrap().insert(blank_set,Rc::new(temp_set));
            

        Ok("Setup succeeded")
    }

    pub fn make_guess(&mut self, guess:char) -> Result<&Rc<HashSet<String>>,&str> {
        let lower_guess = guess.to_lowercase();

        if self.used_char.as_mut().unwrap().contains(&lower_guess.to_string()) {
            return Err("Guess already made");
        }
        else {
            self.used_char.as_mut().unwrap().insert(lower_guess.to_string());
        }

        let current_words = self.word_groups.as_mut().unwrap().remove(self.current_set.as_ref().unwrap()).unwrap();
        self.word_groups = Some(HashMap::new());

        let set_length = self.current_set.as_ref().unwrap().len();

        for i in 0..set_length {
            let mut new_key_lv1 = self.current_set.as_ref().unwrap().clone(); 
            new_key_lv1.replace_range(i..=i, &lower_guess.to_string());


            for j in i..set_length {
                let mut new_key_lv2 = new_key_lv1.clone();
                new_key_lv2.replace_range(j..=j, &lower_guess.to_string());

                for k in j..set_length {
                    let mut new_key_lv3 = new_key_lv2.clone();
                    new_key_lv3.replace_range(k..=k, &lower_guess.to_string());

                    let mut temp_set:HashSet<String> = HashSet::new();

                    for word in current_words.iter() {
                        if word.get(i..=i).unwrap() == lower_guess.to_string().as_str() && word.get(j..=j).unwrap() == lower_guess.to_string().as_str() &&
                               word.get(k..=k).unwrap() == lower_guess.to_string().as_str() {
                            let mut word_count=0;
                            let mut key_count=0;

                            for x in 0..set_length {
                                if word.get(x..=x).unwrap() == lower_guess.to_string().as_str() {
                                    word_count += 1;
                                }
                                if new_key_lv3.get(x..=x).unwrap() == lower_guess.to_string().as_str() {
                                    key_count += 1;
                                }
                            }
                            if key_count == word_count {
                                //println!("{}",word);
                                temp_set.insert(word.to_string());
                            }

                        }
                    }

                    if temp_set.len() != 0 {
                        //println!("{} words {}",new_key_lv3, temp_set.len());
                        self.word_groups.as_mut().unwrap().insert(new_key_lv3, Rc::new(temp_set));
                    }

                }

            }
        }
        let mut temp_set = HashSet::new();
        for word in current_words.iter() {
            if !word.contains(lower_guess.to_string().as_str()) {
                temp_set.insert(word.to_string());
            }
        }
        //println!("{} words {}",self.current_set.as_ref().as_ref().unwrap(), temp_set.len());
        self.word_groups.as_mut().expect("no hashmap").insert(self.current_set.as_ref().as_ref().expect("no current set").to_string(), Rc::new(temp_set));

        let mut new_key: Option<&String> = None;
        let mut new_set: Option<&Rc<HashSet<String>>> = None;
        
        for (key, val) in self.word_groups.as_ref().unwrap().iter() {
            if new_key.is_none() {
                new_key = Some(key);
                new_set = Some(val);
            }
            else if new_set.unwrap().len() < val.len() {
                new_key = Some(key);
                new_set = Some(val);
            }
            else if new_set.unwrap().len() == val.len() {
                if !new_key.unwrap().contains(lower_guess.to_string().as_str()) {
                    continue;
                }
                else if !key.contains(lower_guess.to_string().as_str()) {
                    new_key = Some(key);
                    new_set = Some(val);
                }
                else if new_key.unwrap().find(lower_guess.to_string().as_str()) < key.find(lower_guess.to_string().as_str()) {
                    new_key = Some(key);
                    new_set = Some(val);
                }
                else if Self::rightness(&new_key.unwrap(),lower_guess.to_string().as_str()) > Self::rightness(key,lower_guess.to_string().as_str()) {
                    new_key = Some(key);
                    new_set = Some(val);
                }
            }
            //println!("{} words {}", new_key.unwrap(), new_set.unwrap().len());
        }


        self.current_set = Some(new_key.unwrap().to_string());

        Ok(new_set.unwrap())
    }

    fn rightness(key:&String, guess:&str) -> u32 {
        let mut sum:u32 = 0;
        let mut pos:u32 = 1;
        for i in key.len()-1..0 {
           if key.get(i..i).unwrap() == guess {
            sum += pos;
           } 
           pos *= 10;
        }
        sum
    }

    pub fn get_guessed_letters(&mut self) -> &BTreeSet<String> {
        self.used_char.as_ref().unwrap()
    }

    pub fn get_current_set(&mut self) -> &String {
        self.current_set.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DICTIONARY:&str = "dictionary.txt";
    const SMALL_DICTIONARY:&str = "small.txt";
    const EMPTY_DICTIONARY:&str = "empty.txt";
    const EMPTY_DICT_ERROR:Result<&str,&str> = Err("empty dictionary");
    const SUCCESSFUL_LOAD:Result<&str,&str> = Ok("Setup succeeded");
    const GUESS_ALREADY_MADE:Result<&Rc<HashSet<String>>,&str> = Err("Guess already made");

    fn setup() -> EvilHangmanGame {
        EvilHangmanGame { word_groups: None, current_set: None, used_char: None }
    }

    #[test]
    fn test_empty_file_load() {
        let mut game = setup();
        assert_eq!(EMPTY_DICT_ERROR,game.start_game(&mut File::open(EMPTY_DICTIONARY).expect("dictionary not found"), 4),"Failed to return empty dictionary error");
        assert_eq!(EMPTY_DICT_ERROR,game.start_game(&mut File::open(EMPTY_DICTIONARY).expect("dictionary not found"), 1),"Failed to return empty dictionary error");
        assert_eq!(EMPTY_DICT_ERROR,game.start_game(&mut File::open(EMPTY_DICTIONARY).expect("dictionary not found"), 15),"Failed to return empty dictionary error");
    }

    #[test]
    fn test_word_length() {
        let mut game = setup();
        assert_eq!(EMPTY_DICT_ERROR,game.start_game(&mut File::open(EMPTY_DICTIONARY).expect("dictionary not found"),0),"Failed to return empty dictionary error");
    }

    #[test]
    fn test_load_files() {
        let mut game = setup();
        assert_eq!(SUCCESSFUL_LOAD,game.start_game(&mut File::open(DICTIONARY).expect("dictionary not found"),2),"Loading file with dictionary gave an error");
        assert_eq!(SUCCESSFUL_LOAD,game.start_game(&mut File::open(DICTIONARY).expect("dictionary not found"),10),"Loading file with dictionary gave an error");
        assert_eq!(SUCCESSFUL_LOAD,game.start_game(&mut File::open(SMALL_DICTIONARY).expect("dictionary not found"),10),"Loading file with dictionary gave an error");
    }

    #[test]
    fn test_guess_already_made() {
        let mut game = setup();

        game.start_game(&mut File::open(DICTIONARY).expect("dictionary not found"),2).expect("Empty Dictionary");

        game.make_guess('a').expect("Error");

        assert_eq!(GUESS_ALREADY_MADE,game.make_guess('a'),"Failed to return Guess already made error.");
        assert_eq!(GUESS_ALREADY_MADE,game.make_guess('A'),"Failed to return Guess already made error with uppercase letter.");

        game.make_guess('E').expect("Guessing a letter after a previously guess letter gave an error");

        assert_eq!(GUESS_ALREADY_MADE,game.make_guess('E'),"Failed to return Guess already made error with uppercase letter.");
        assert_eq!(GUESS_ALREADY_MADE,game.make_guess('a'),"Failed to return Guess already made error with previously guessed letter.");
    }

    #[test]
    fn test_2_letter_word() {
        let mut game = setup();

        game.start_game(&mut File::open(DICTIONARY).expect("dictionary not found"),2).expect("Dictionary that contains words is counted as empty");

        let possible_words = game.make_guess('a').unwrap();

        assert_eq!(68,possible_words.len(),"Incorrect set size");
        let temp_vec: Vec<String> = ["be","bi","bo","by","de","do","ef","eh","el","em","en","er","es","et","ex","go","he","hi","hm","ho","id","if","in","is","it","jo","li","lo","me","mi","mm","mo","mu","my","ne","no","nu","od","oe","of","oh","om","on","op","or","os","ow","ox","oy","pe","pi","re","sh","si","so","ti","to","uh","um","un","up","us","ut","we","wo","xi","xu","ye"].map(String::from).to_vec();
        let correct_possibilities:HashSet<String> = temp_vec.into_iter().collect();

        assert_eq!(correct_possibilities,**possible_words, "Incorrect set contents after 1 guess");

        assert_eq!(GUESS_ALREADY_MADE,game.make_guess('a'), "Set changed on duplicate guess");

        let possible_words = game.make_guess('e').unwrap();

        assert_eq!(49, possible_words.len(), "Incorrect set size after second guess");
        let temp_vec: Vec<String> = ["bi","bo","by","do","go","hi","hm","ho","id","if","in","is","it","jo","li","lo","mi","mm","mo","mu","my","no","nu","od","of","oh","om","on","op","or","os","ow","ox","oy","pi","sh","si","so","ti","to","uh","um","un","up","us","ut","wo","xi","xu"].map(String::from).to_vec();
        let correct_possibilities:HashSet<String> = temp_vec.into_iter().collect();

        assert_eq!(correct_possibilities,**possible_words, "Incorrect set contents after second guess");
    }
}
