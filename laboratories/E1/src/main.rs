use std::{char, str};
use clap::Parser;
#[derive(Parser, Debug)]
struct Args {
    // input string
    slug_in: String,
    repeat: i32,
    verbose: bool,
}

fn slugify(s: &str) -> String {
    let mut result = String::new();
    const SUBS_N:&str = "abcdefghijklmnopqrstuvwxyz0123456789";
    let mut current:char = ' ';
    for letter in s.to_lowercase().chars(){
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

fn main(){
    let args = Args::parse();
    let param:&str = &args.slug_in;
    for i in 0..args.repeat {
        let result = slugify(param);
        println!("{}",result);
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn accent(){
        let value = "à";
        assert_eq!(slugify(value),"a");
    }
    #[test]
    fn normal(){
        let value = "a";
        assert_eq!(slugify(value),"a");
    }
    #[test]
    fn unrecognized_normal(){
        let value = "+";
        assert_eq!(slugify(value),"-");
    }
    #[test]
    fn unrecognized_accent(){
        let value = "ŵ";
        assert_eq!(slugify(value),"-");
    }
    #[test]
    fn multi_words_sentence(){
        let value = "Ciao a tutti";
        assert_eq!(slugify(value),"ciao-a-tutti");
    }
    #[test]
    fn multi_words_accented(){
        let value = "È sé stesso";
        assert_eq!(slugify(value),"e-se-stesso");
    }
    #[test]
    fn empty(){
        let value = "";
        assert_eq!(slugify(value),"");
    }
    #[test]
    fn multi_blank(){
        let value = "Troppi   spazi!";
        assert_eq!(slugify(value),"troppi-spazi");
    }
    #[test]
    fn only_unrecognized(){
        let value = "+ * - = -";
        assert_eq!(slugify(value),"-");
    }
    #[test]
    fn multi_unrecognized(){
        let value = "C++";
        assert_eq!(slugify(value),"c");
    }
    #[test]
    fn ending_blank(){
        let value = "Spazio ";
        assert_eq!(slugify(value),"spazio");
    }
    #[test]
    fn multi_ending_unrecognized(){
        let value = "Finito!!!!!";
        assert_eq!(slugify(value),"finito");
    }
}

