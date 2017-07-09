extern crate clap;

use std::error::Error;
use std::fs::File;
use std::process;
use std::io;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;

use clap::{Arg, App};


fn myread<RT: BufRead, WT:Write>(input: RT, mut output: WT) {
    for o_line in input.lines() {
        let str_line = match o_line {
            Err(why) => {
                println!("Couldn't read from STDIN! Reason: {}", why);
                process::exit(0x0f00);
            },
            Ok(line) => line
        };

        let mut line: String = str_line.to_string();
        line.push_str("\r\n");

        match output.write(line.as_bytes()) {
            Err(why) => {
                println!("Couldn't write to output! Reason: {}", why);
                process::exit(0x0f00);
            },
            Ok(_) => {}
        }
    }
}


fn main() {
    let matches = App::new("rewriter")
        .version("1.0")
        .about("PG SQL dump obfuscator!")
        .author("Michael Voronin")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("Sets the input file")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Sets the output file")
            .takes_value(true))
        .get_matches();

    let path_config = matches.value_of("config").unwrap();
    println!("{}", path_config);

    let mut owrp_ifile:Option<File> = None;
    let mut owrp_ofile:Option<File> = None;

    let owrp_ifilename = matches.value_of("input");
    let owrp_ofilename = matches.value_of("output");


    if let Some(filename) = owrp_ifilename {
        match File::open(&filename) {
            Err(why) => {
                println!("Couldn't open the file \"{}\". Reason: {}", filename, why.description());
                process::exit(0x0f00);
            },
            Ok(file) => { owrp_ifile = Some(file) },
        };
    }

    if let Some(filename) = owrp_ofilename {
        match File::create(&filename) {
            Err(why) => {
                println!("Couldn't create the file \"{}\". Reason: {}", filename, why.description());
                process::exit(0x0f00);
            },
            Ok(file) => { owrp_ofile = Some(file) },
        }
    }

    match owrp_ifile {
        Some(ifile) => {
            let ifilebuf = BufReader::new(&ifile);

            match owrp_ofile {
                Some(ofile) => {
                    myread(ifilebuf, ofile);
                },
                None => {
                    let stdout = io::stdout();
                    let stdout_handle = stdout.lock();
                    myread(ifilebuf, stdout_handle);
                }
            }


        },
        None => {
            let stdin = io::stdin();
            let stdin_handle = stdin.lock();

            match owrp_ofile {
                Some(ofile) => {
                    myread(stdin_handle, ofile);
                },
                None => {
                    let stdout = io::stdout();
                    let stdout_handle = stdout.lock();

                    myread(stdin_handle, stdout_handle);
                }
            }
        }
    }
}
