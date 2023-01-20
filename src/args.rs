fn get_nth_arg(n: usize) -> String{
    std::env::args().nth(n).unwrap()
}
//creating a stuct tuple for the Arguments Variable
//Derives the debug trait for the struct in order to allow app to run
#[derive(Debug)]
pub struct Args {
    pub image1: String,
    pub image2: String,
    pub output: String
}

//implementing a new function to the Args Struct that will build out the
//struct the way we need it
impl Args {
    pub fn new() -> Self{
        //use the function to grab the three variables of the struct to fill in when creating
        Args {
        image1: get_nth_arg(1),
        image2: get_nth_arg(2),
        output: get_nth_arg(3)
        }
    }
}