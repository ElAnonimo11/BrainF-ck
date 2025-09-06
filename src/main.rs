

fn main() {
    // We collect the data
    let v_args: Vec<String> = std::env::args().collect();

    // If not correct size, we exit
    if v_args.len() != 2 {
        panic!("Wrong number of arguments");
    }


}
