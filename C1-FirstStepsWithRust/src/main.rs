use std::io::stdin;  // Namespace prefix

// Customize greeting with each name
struct Visitor {  // struct is a type that groups data together
                  // Built-in structs include: String, StdIn
    name: String,  // member fields can be of almost any variable type
    greeting: String, // member field syntax: field: type

}

/* impl == 'implement' ::> - methods, aka, 'associated fns' */
impl Visitor { // create struct Vistor instance: Visitor::new()
    fn new(name: &str, greeting: &str) -> Self {  // return type(Self)
        Self {                             // `Self` refers: struct type itself
            name: name.to_lowercase(),  // .to_lowercase() converts &str->String
            greeting: greeting.to_string(),  // .to_string() converts &str->String
        }
    }
    // Implicit return syntax: lack of `;`
    fn greet_visitor(&self) { // Member Function, akak Method
        println!("{}", self.greeting);  // vs `self` refers: instance of struct
    }
}



fn treehouse_guestlist(name: String) {
    /* ARRAYS */
    // type(str): unmutable, hard-coded
    // type(String): mutable, stores(location, len, capacity)
    // let alt_visitor_list : [&str;3] = ["bert", "stever", "fred"];
    // let visitor_list = ["bert", "steve", "fred"];
    let visitor_list = [ // Requires `impl` fns for Visitor struct
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    let mut allow_them_in = false;
    /* LOOPS */
    /* for visitor in &visitor_list {
        if visitor == &name.to_lowercase() {
            allow_them_in = true;
        }
    } */
    /* iterators */
    let known_visitor = visitor_list  // assign result of iter fns chain to var
        .iter()  // create an type(iter): contains all data(visitor_list)
        .find(|visitor| visitor.name == name);  // run a closure: rets `true`
                                                // `find()` rets matching val
                                                // `;` finishes statement
                                            // Closures:
                                            // - a fns defined in place
                                            // Inline closure == def a fns:
                                            // `|visitor| visitor.name == name`


    if allow_them_in {
        println!("Welcome to the Treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }
}

/*
fn treehouse_guestlist_struct(name:String) {

}
*/

fn what_is_your_name() -> String { // -> String denotes fns returns a String
    println!("Hello, what's your name?");
    let mut your_name = String::new(); // type = String
    stdin() // Function chaining
        .read_line(&mut your_name)  // std::io::stdin::read_line
                                    //                  |-> an stdin method
                                    // &mut == "Borrow"
                                    //      aka, pass-by-reference
                                    //      aka, pointer
        .expect("Failed to read line");  // "Unwrap" a Result object &&
                                         //         terminate prgm w/
                                         //         specificed mgs IF ERROR
                                         //   aka, Error Handling
    your_name  // Return variable is last line in fns
        .trim()     // trim special chars: \r, \n
                    // WARNING: .trim() converts String -> str
        // .to_lowercase()
        .to_string() // Convert str -> String
}

fn main() {
    let name = what_is_your_name();
    println!("Hello, {}", name);
    println!("Hello, {:?}", name);  // Debugging placeholder [<<] \r, \n, "VAR"
    treehouse_guestlist(name);
}
