fn main(){
    // things learned:
    /*
    - Rust does macros with !
    - the ; is back after every line
    - 
    
    */

    let x = 5* 6 /* comment again */ + 1;
    println!("Hi there, x is = {}", x);
    format!("this is a string");
    print!("this is a print to console");
    eprint!("this is an error");

    #[allow(dead_code)]  // this disabled the warning for deadcode
    struct Structure(i32);

}