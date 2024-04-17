trait MySlug {
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

impl <T> MySlug for T where T: AsRef<str> + {
    fn is_slug(&self) -> bool{
        let mut condition:bool = true;
        const SUBS_N:&str = "abcdefghijklmnopqrstuvwxyz0123456789-";
        for letter in self.as_ref().chars(){
            if !SUBS_N.contains(letter){
                condition = false;
            }
        }
        if self.as_ref().ends_with('-') && self.as_ref().len() != 1 {
            condition = false;
        }
        condition
    }
    fn to_slug(&self) -> String {
        let mut result = String::new();
        const SUBS_N:&str = "abcdefghijklmnopqrstuvwxyz0123456789";
        let mut current:char = ' ';
        for letter in self.as_ref().to_lowercase().chars(){
            if SUBS_N.contains(letter) {
                result.push(letter);
                current = letter;
            }else{
                if conv(letter) != '-' {
                    result.push(conv(letter));
                }else{
                    if current != '-' {
                        result.push(conv(letter));
                    }
                }
                current = conv(letter);
            }
        }
        while result.ends_with("-") && result.len() != 1 {
            result.pop();
        }
        result
    }
}

fn conv(c: char) -> char {
    const SUBS_I:&str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O:&str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";
    match SUBS_I.chars().position(|x| x == c) {
        Some(index) => {
            SUBS_O.chars().nth(index).unwrap_or('-')
        }
        None => '-',
    }
}

fn main() {
    let s1 = String::from("Hello String");
    let s2 = "hello-slice";
    println!("{}",s1.is_slug()); // false
    println!("{}",s2.is_slug()); // true

    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();

    println!("s3:{} s3:{}", s3, s4); // prints: s3:hello-string s4:hello-slice
}
