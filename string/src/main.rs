fn create_string() {
    /*
    Creating a New String
     */

    // create a empty String object just like a vector, or formally a Vec<T>
    let _s1 = String::new();

    // create a String object from a initial data
    let data = "initial contents";
    let _s2 = data.to_string();

    // create a String object from literal directly:
    let _s3 = "initial contents".to_string();

    // create a String object from a literal by using the function:
    // String::from actually does the same thing with .to_string()
    let _s4 = String::from("initial contents");

    // strings are UTF-8 encoded
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
}

fn update_string() {
    /*
    Updating a String
     */

    // push_str method adds a string slice to a String without taking ownership of the parameter
    let mut _ms1 = String::from("foo");
    _ms1.push_str("bar");
    let _s5 = "string slice contents";
    _ms1.push_str(_s5);
    println!("_ms1 becomes \"{_ms1}\"");
    println!("_s5 is still \"{_s5}\"");

    // push method adds a single character to the String
    let mut _ms2 = String::from("lo");
    _ms2.push('l');
    println!("_ms2 becomes \"{_ms2}\"");

    // Concatenate strings by using '+'
    let _s6 = String::from("Hello, ");
    let _s7 = String::from("world!");
    let _s8 = _s6 + &_s7; // note _s6 has been moved here and can no longer be used

    println!("_s7 is still \"{_s7}\"");
    println!("_s8 becomes \"{_s8}\"");

    // The format! macro makes complex concatenations more easier, because it uses the references.
    let _s9 = String::from("tic");
    let _s10 = String::from("tac");
    let _s11 = String::from("toe");

    let _s12 = format!("{_s9}-{_s10}-{_s11}");
    println!("_s12 is still \"{_s12}\"");
}

fn index_into_string() {
    /*
    Indexing into Strings
     */

    let _s1 = String::from("hello");
    // direct indexing is not allowed
    // let h = _s1[0];
}

fn slice_string() {
    /*
    Slicing Strings
     */

    // create a string slice from a String
    let _hello = "Здравствуйте";
    let _s = &_hello[0..4];
}

fn iterate_over_string() {
    /*
    Iterating Over Strings
     */

    // in chars
    for c in "Зд".chars() {
        println!("{c}");
    }

    // in bytes
    for c in "Зд".bytes() {
        println!("{c}");
    }
}

fn main() {
    create_string();
    update_string();
    index_into_string();
    slice_string();
    iterate_over_string();
}
