use regex::Regex;

fn main() {
    let word_list=vec!["sapwood", "rosewood", "logwood",
             "teakwood", "plywood", "redwood"];
    let pattern=r"(ply|log)wood";

    let regex=Regex::new(pattern).expect("Failed to create regex");

    for word in word_list{
        if regex.is_match(word){
          println!("{} MATCHES for {}", word, pattern);
        } else {
            println!("{} doesnt match for {}", word, pattern);
        }
    }
}
// -----------------------------------------------------
// Asterisk Symbol
//  let word_list=vec!["fooaaaabar", "fooabar","fooaxbar", "foobar",
//              "fooaabar", "fooxxxbar", "fooxbar"];
//     let pattern="fooa*bar";
// -----------------------------------------------------
// Wild Card 

//   let word_list=vec!["fooabar", "fooxbar", "baryfoo",
//              "foobar", "fooxybar", "foocbar","foo3bar"];
//     let pattern="foo.bar";
// -----------------------------------------------------
// Wild Card with asterisk
// let word_list=vec! ["foobar", "barfoo", "fooabcbar",
//              "foobxcbar", "barcbyfoo", "foozbar", "barafoo", "barabfoo"];
//     let pattern="foo.*bar";
// -----------------------------------------------------
// Space 

// let word_list=vec! ["fooxxxbar", "foo   bar", "fooxbar", "fooxxbar",
//                                                   "foo bar", "foo      bar", "foobar", "fooyyybar"];
//     let pattern=r"foo\s*bar";
// -----------------------------------------------------

// Character Range Ex-1
//  let word_list=vec! ["foo", "moo", "coo", "doo",
//             "poo", "loo", "boo", "hoo"];
//     let pattern=r"[fcl]oo";
// -----------------------------------------------------
// Character Range Ex-2

//  let word_list=vec!["foo", "moo", "coo", "doo",
//             "poo", "loo", "boo", "hoo"];
//     let pattern=r"[^mh]oo";
// -----------------------------------------------------
// Character Range Ex-3

//  let word_list=vec!["joo", "boo", "koo", "loo", "poo", "moo", "zoo", "hoo"];
//     let pattern=r"[j-m]oo";
// -----------------------------------------------------
// Character Range Ex-4

//  let word_list=vec!["joo", "boo", "koo", "loo", "poo", "moo", "zoo", "hoo"];
//     let pattern=r"[j-mz]oo";
// -----------------------------------------------------
// Character Range Ex-5

    // let word_list=vec!["joo", "boo", "Koo", "Loo", "poo", "moo", "zoo", "hoo"];
    // let pattern=r"[j-mJ-Mz]oo";
// -----------------------------------------------------
// Escaping Character  Ex-1
//  let word_list=vec!["x#y", "x:y", "x.y", "x&y", "x%y"];
//     let pattern=r"x[#:.]y";
// -----------------------------------------------------
// Escaping Character  Ex-2
// let word_list=vec!["x#y", "x:y", "x^y", "x&y", "x%y"];
//     let pattern=r"x[#:\^]y";
// -----------------------------------------------------
// Escaping Character  Ex-3
//  let word_list=vec!["x#y", "x\\y", "x^y", "x&y", "x%y"];
//     let pattern=r"x[#\\\^]y";
// -----------------------------------------------------

// Caret Anchor
// let word_list=vec!["foo bar baz", "bar foo baz",
//              "baz foo bar", "bar baz foo", "foo baz bar", "baz bar foo"];
//     let pattern=r"^foo.*";
// -----------------------------------------------------
// Dollar Anchor
// let word_list=vec!["foo bar baz", "bar foo baz",
//              "baz foo bar", "bar baz foo", "foo baz bar", "baz bar foo"];
//     let pattern=r".*bar$";
// -----------------------------------------------------
// Curly Brace Repeater -1 
    //  let word_list=vec!["834", "519", "4874", "5", "89", "45687", "25", "645"];
    // let pattern=r"^[0-9]{3}$";
// -----------------------------------------------------
// Curly Brace Repeater -2
    // let word_list=vec!["lion", "tiger", "leopard", "fox",
    //          "kangaroo", "cat", "mouse", "cuckoo", "deer"];
    // let pattern=r"^[a-z]{4,6}$";
// -----------------------------------------------------
// Curly Brace with Paranthesis
// let word_list=vec!["haaaa","ha", "hahahahaha", "hahaha", "hahahaha", "haha",
//              "hahahahahaha", "hahahahahahahaha", "hahahahahahahahaha"];
//     let pattern=r"(ha){4,9}";
// -----------------------------------------------------
// Plus Symbol
// let word_list=vec!["fooaaaabar", "fooabar", "foobar",
//              "fooaabar", "fooxxxbar", "fooxar"];
//     let pattern=r"fooa+bar";
// -----------------------------------------------------
// Question Symbol
    // let word_list=vec!["https://website", "http://website",
    //          "httpss://website", "httpx://website", "httpxx://website"];
    // let pattern=r"https?://website";
// -----------------------------------------------------














