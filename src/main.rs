use std::env;
use std::process;

fn help(){
    println!("Usage: rgrep [OPTIONS]... PATTERN [FILE]...");
    println!("  -i,         ignore case distinctions in patterns and data");
    println!("  -v,         select non-matching lines");
    println!("  -n,         print line number with output lines");
    println!("  -r,         recursive file parsing");
    println!("  -d,         show debug information");
    process::exit(0);
}

fn unknown(){
    println!("Usage: rgrep [OPTIONS]... PATTERN [FILE]...");
    println!("Try 'rgrep -h' for more information.");
    process::exit(0);
}

fn flag_parsing(
    ignore_case: &mut bool,
    recursive: &mut bool,
    numbered: &mut bool,
    debug: &mut bool,
    invert_match: &mut bool,
    ) {
    let args: Vec<String> = env::args().collect();
        for (index, arg) in args.iter().enumerate() {
            if index > 0 {
                if arg.starts_with('-') {
                    for c in arg.chars().skip(1) {
                        match c {
                        'i' => *ignore_case = true,
                        'v' => *invert_match = true,
                        'n' => *numbered = true,
                        'd' => *debug = true,
                        'r' => *recursive = true,
                        'h' => help(),
                        _ => unknown(),
                    }
                }
            }
        }
    }
}


fn main() {
    let mut ignore_case = false;
    let mut recursive = false;
    let mut numbered = false;
    let mut debug = false;
    let mut invert_match = false;
    flag_parsing(
        &mut ignore_case,
        &mut recursive,
        &mut numbered,
        &mut debug,
        &mut invert_match,
    );
    if debug {
        println!("Flags: ignore_case = {}, recursive = {}, numbered = {}, invert_match =  {} ", ignore_case, recursive, numbered, invert_match);
    }
}
