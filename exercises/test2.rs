// test2.rs
// This is a test for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `Strings`, some are `&strs`. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

// I AM DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); //reference not String
    string("red".to_string()); //reference not String, to_string returns String
    string(String::from("hi"));
    string("rust is fun!".to_owned()); //clones the reference
    string_slice("nice weather".into()); //cool! into, converts to either &str or String! both working
    string(format!("Interpolation {}", "Station")); //format to String
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim()); //trim of reference is still reference
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
