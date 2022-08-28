use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "fundamentals",
    about = "Simple CLI example from Thorsten Hans rust/structopt cli fundamentals",
    author = "Douglas E Alves")]

struct CLI {
    #[structopt(long, short, global = true, help = "Prints debug info")]
    debug: bool,
    input: String,
    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    #[structopt(name = "mod", about = "Use mod to modify strings")]
    Modify(ModifyOptions),
    #[structopt(name = "insp", about = "Use insp to inspect strings")]
    Inspect(InspectOptions),
}

#[derive(Debug, StructOpt)]
struct ModifyOptions {
    #[structopt(short, long, help = "Transforms a string to uppercase")]
    upper: bool,

    #[structopt(short, long, help = "Transforms a string to lowercase")]
    lower: bool,

    #[structopt(short, long, help = "Reverses a string")]
    reverse: bool,

    #[structopt(short = "pref", long, help = "Adds a prefix to the string")]
    prefix: Option<String>,

    #[structopt(short = "sub", long, help = "Adds a suffix to the string")]
    suffix: Option<String>,
}


#[derive(Debug, StructOpt)]
struct InspectOptions {
    #[structopt(short, long, help = "Counts all characters in a string")]
    length: bool,

    #[structopt(short, long, help = "Counts only numbers in the given string")]
    numbers: bool,
    
    #[structopt(short, long, help = "Counts only numbers in the given string")]
    spaces: bool,
}

fn inspect(input: &String, debug: bool, args: &InspectOptions) {
    println!("Inspect called for: {:#?}", input);
    if debug { println!("{:#?}", args); }

    if args.length {
        length(input, debug);
    }

    if args.numbers {
        count_numbers(input, debug)
    }

    if args.spaces {
        count_whitespace(input, debug);
    }
}

fn length(input: &String, _debug: bool) {
    let len: usize = input.chars().count();

    println!("Size of the value received is: {:#?}", len);
}

fn count_whitespace(input: &String, debug: bool) {
    let original_length: usize = input.chars().count();
    let trimmed_length: usize = input.trim().chars().count();

    if debug { 
        println!("Original text length: {:#?}", original_length);
        println!("Text length without the whitespaces: {:#?}", trimmed_length);
    }

    println!("Qty of white space in the text received is: {:#?}", trimmed_length - original_length);
}

fn count_numbers(input: &String, debug: bool) {
    let mut result = 0;

    for c in input.chars() {
        // Only count whitespace chars that are not preceded by another whitespace.
        if debug { 
            println!("Current char: {:#?}", c);
            println!("Is the current character a number? {:#?}", c.is_numeric());
        }

        if c.is_numeric() {
            result += 1
        }
    }

    println!("Qty of numbers in the text received is: {:#?}", result);
}

fn modify(input: &String, debug: bool, args: &ModifyOptions) {
    println!("Modify called for: {:#?}", input);
    if debug { println!("Params: {:#?}", args); }

    if args.upper { upper(input, debug)}
    if args.lower { lower(input, debug)}
    if args.reverse { reverse(input, debug)}
    if args.prefix.is_some() { 
        let prefix: &Option<String> = &args.prefix;
        add_prefix(input, prefix, debug)
    }
    if args.suffix.is_some() { 
        let suffix: &Option<String> = &args.suffix;
        add_suffix(input, suffix, debug)
    }
}

fn upper(input: &String, _debug: bool) {
    println!("Uppercase result: {:#?}", input.to_uppercase())
}

fn lower(input: &String, _debug: bool) {
    println!("Lowercase result: {:#?}", input.to_lowercase())
}

fn reverse(input: &String, _debug: bool) {
    let reversed_text: String = input.chars().rev().collect();
    println!("Reverse result: {:#?}", reversed_text)
}

fn add_prefix(input: &String, prefix: &Option<String>, _debug: bool) {
    println!("Prefix result: {:#?} {:#?}", prefix.as_ref().unwrap(), input)
}

fn add_suffix(input: &String, suffix: &Option<String>, _debug: bool) {
    println!("Suffix result: {:#?} {:#?}", input, suffix.as_ref().unwrap())
}

fn main() {
    let args = CLI::from_args();

    match args.cmd {
        SubCommand::Inspect(opt) => {
            inspect(&args.input, args.debug, &opt);
        }
        SubCommand::Modify(opt) => {
            modify(&args.input, args.debug, &opt);
        }
    }
}
