fn variables() {
    // x is not mutanable
    let x = 1;
    // because we put "mut" when declaring the varaible we are
    // able to change it later on
    let mut y = 3;

    println!("The x variable is not able to change it is equal to {x}");
    println!("The y variable DOES have the ability to change it is equal to this {y}");

    // change the y variable
    y = 10;

    println!("Now the y varaible is {y}");
}

fn expressions() {
    // Here are the diffrent types of expressions
    println!("Below is an example of a literal expression");
    let string = "Hello";
    let character = '5';
    let int = 5;
    println!("{string} is a string");
    println!("{character} is a character");
    println!("{int} is a is a integer");

    // diffrent math expressions in rust
    println!("Below are examples of math expressions in rust");
    println!("Addition");
    let mut x = 5 + 5;
    println!("{x}");

    println!("Subtraction");
    x = 5 - 5;
    println!("{x}");

    println!("Multiplication");
    x = 5 * 5;
    println!("{x}");

    println!("Division");
    x = 5 / 5;
    println!("{x}");

    println!("Remainder or Mod");
    // Have to declare a new variable to use floats
    let y = 5.0 % 5.5;
    println!("{y}");
    
}

fn conditionals(x: i32) {
    // conditional with a passed in variable
    if x > 5 {
        println!("{x} is greater than 5");
    } else if x < 5 {
        println!("{x} is less than 5");
    } else {
        println!("{x} is equal to 5");
    }

}

fn loops() {
    // Start at 0 and keep adding and compariing until 10
    let mut x = 0;
    
    while x <= 10 {
        if x > 5 {
            println!("{x} is greater than 5");
        } else if x < 5 {
            println!("{x} is less than 5");
        } else {
            println!("{x} is equal to 5");
        }

        x = x + 1
    }
}

fn ref_function(s: &String) -> usize {
    // The variable is passed in then the size of the string is returned
    // since there the varribale is not borrowed it can not be changed
    s.len()
}

fn borrow_function(s: &mut String) {
    // borrowed variable s is then changed to include my last name
    s.push_str(", Miskin");
    println!("Then after being passed to the function and changed we get {s}");
}

fn main() {
    // Hello World print
    println!("Hello, World!");

    // Print the examples and what they do.
    println!("---------------------------------------------------------------------------");
    println!("Variables Exanple:");
    variables();

    println!("---------------------------------------------------------------------------");
    println!("Expresions Example:");
    expressions();

    println!("---------------------------------------------------------------------------");
    println!("Conditionals Example:");
    conditionals(5);
    
    println!("---------------------------------------------------------------------------");
    println!("Loops Example:");
    loops();

    println!("---------------------------------------------------------------------------");
    println!("Refrence Function Example:");
    
    // cretate a string
    let s1 = String::from("Dawson");
    // get the lenght of the string by passing refrence '&'
    let len = ref_function(&s1);
    println!("The length of '{}' is {}.", s1, len);

    println!("Borrow Function Example:");
    // have to create a mut string to change it when borrowed
    let mut s2 = String::from("Dawson");
    println!("The string that we start with is {s2}");
    // create the refrence so we can borrow the string and not change it
    let ref1 = borrow_function(&mut s2);
    println!("---------------------------------------------------------------------------");
}