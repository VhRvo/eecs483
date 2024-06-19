type AST = i64;

fn main() {
    use std::fs;

    // get the program arguments as a Vec<String>
    let args = std::env::args().collect::<Vec<_>>();
    // read arg[1] into a String
    let input = fs::read_to_string(&args[1]).unwrap();
    let number = parse(&input).unwrap();
    println!("{}", compile(number));
}

fn parse(str: &str) -> Result<AST, String> {
    // .trim() removes leading and trailing whitespace
    i64::from_str_radix(str.trim(), 10).map_err(|err| err.to_string())
}

fn compile(number: AST) -> String {
    // Add _ to the front of the label for Mac OS X
    format!(
        "\
    section .text
    global _start_here
_start_here:
    mov rax, {}
    ret \n",
        number
    )
}
