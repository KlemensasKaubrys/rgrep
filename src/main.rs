use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};
use walkdir::WalkDir;
use std::path::Path;
use std::time::Instant;

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

fn file_parsing(
    ignore_case: &bool,
    recursive: &bool,
    numbered: &bool,
    invert_match: &bool,
    h: &String,
    n: &String,
) -> io::Result<()> {
    if *recursive {
        for entry in WalkDir::new(h)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                process_file(entry.path(), ignore_case, numbered, invert_match, n)?;
            }
        }
    } else {
        process_file(Path::new(h), ignore_case, numbered, invert_match, n)?;
    }
    Ok(())
}

fn process_file(
    path: &Path,
    ignore_case: &bool,
    numbered: &bool,
    invert_match: &bool,
    n: &String,
) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut line_number: u64 = 0;

    for line in reader.lines() {
        line_number += 1;
        let line = line?;

        let line_to_check = if *ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };

        let search_string = if *ignore_case {
            n.to_lowercase()
        } else {
            n.clone()
        };

        let matches = if *invert_match {
            !line_to_check.contains(&search_string)
        } else {
            line_to_check.contains(&search_string)
        };

        if matches {
            if *numbered {
                print!("{}: ", line_number);
            }
            println!("{}", line);
        }
    }

    Ok(())
}

fn flag_parsing(
    ignore_case: &mut bool,
    recursive: &mut bool,
    numbered: &mut bool,
    debug: &mut bool,
    invert_match: &mut bool,
    needle_exists: &mut bool,
    h: &mut String,
    n: &mut String,
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
            } else if *needle_exists {
                h.push_str(arg);

            } else {
                n.push_str(arg);
                *needle_exists = true;
            }
        }
    }
}


fn main() {
    let start = Instant::now();
    let mut ignore_case = false;
    let mut recursive = false;
    let mut numbered = false;
    let mut debug = false;
    let mut invert_match = false;
    let mut needle_exists = false;
    let mut haystack = String::new();
    let mut needle = String::new();
    flag_parsing(
        &mut ignore_case,
        &mut recursive,
        &mut numbered,
        &mut debug,
        &mut invert_match,
        &mut needle_exists,
        &mut haystack,
        &mut needle,
    );
    if debug {println!("-----RESULT-----");}
    file_parsing(
        &ignore_case,
        &recursive,
        &numbered,
        &invert_match,
        &haystack,
        &needle,
    );
    let duration = start.elapsed();
    if debug {
        println!("-----DEBUG-----");
        println!("Flags: ignore_case = {}, recursive = {}, numbered = {}, invert_match =  {} ", &ignore_case, &recursive, &numbered, &invert_match);
        println!("Pattern: {}", &needle);
        println!("File/Directory: {}", &haystack);
        println!("Time elapsed: {:?}", duration);
    }
}
