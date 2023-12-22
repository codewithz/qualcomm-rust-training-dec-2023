use regex::Regex;

fn main() {
    let word_list=vec!["","","",""];
    let pattern="";

    let regex=Regex::new(pattern).expect("Failed to create regex");

    for word in word_list{
        if regex.is_match(word){
          println!("{} MATCHES for {}", word, pattern);
        } else {
            println!("{} doesn't match for {}", word, pattern);
        }
    }
}
