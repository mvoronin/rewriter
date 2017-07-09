use std::env;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;


fn main() {
    let mut owrp_ifilename:Option<String> = None;
    let mut owrp_ofilename:Option<String> = None;

    let args: Vec<String> = env::args().collect();

    let mut i = 1;

    while i+1 < args.len() {
        let arg_key = &args[i];
        let arg_value = args[i+1].clone();

        match arg_key.as_ref() {
            "-i" => { owrp_ifilename = Some(arg_value); }
            "-o" => { owrp_ofilename = Some(arg_value); }
            _ => { panic!("Unrecognized argument!"); }
        }

        i += 2;
    }


    let input_buffer = owrp_ifilename
        .map(|filename| {
            File::open(&filename)
                .map_err(|why| panic!("Couldn't open input file \"{}\": {}", filename, why.description()))
                .unwrap()
        })
        .map(|file| Box::new(BufReader::new(file)) as Box<BufRead>)
        .unwrap_or_else(|| Box::new(BufReader::new(io::stdin())) as Box<BufRead>);

    let mut output_buffer = owrp_ofilename
        .map(|filename| {
            OpenOptions::new().create(true).append(true)
                .open(&filename)
                .map_err(|why| panic!("Couldn't open output file \"{}\": {}", filename, why.description()))
                .unwrap()
        })
        .map(|file| Box::new(BufWriter::new(file)) as Box<Write>)
        .unwrap_or_else(|| Box::new(BufWriter::new(io::stdout())) as Box<Write>);

    for line in input_buffer.lines() {
        let line: String = match line {
            Err(why) => panic!("Couldn't read from file! Reason: {}", why),
            Ok(line) => line
        };

        if let Err(e) = write!(output_buffer, "{}\r\n", line) {
            panic!("Can't write to file! Reason: {}", e);
        }
    }
}
