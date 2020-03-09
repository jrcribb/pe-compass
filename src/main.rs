#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate scroll_derive;
extern crate scroll;
extern crate serde_json;
extern crate serde;
extern crate clap;
extern crate sha2;
extern crate md5;

/// Imports: Rust STD Lib

/// Imports: 3rd Party Crates
use serde_json::*;

/// Imports: My Modules & Utils
mod modules;
use modules::pe_parser::*;

mod utils;
use utils::filesystem::file_handler::FileHandler;
use utils::args::argument_parser::ArgumentsParser;

fn main() -> Result<()>
{
    //let _sample = "pe-samples/ExplorerSuite.exe";
    //let _sample = "pe-samples/010EditorWin32Installer10.0.1.exe";
    //let _sample = "pe-samples/sqlite3x64.dll";
    //let _sample = "pe-samples/sqlite3x86.dll";
    //let _sample = "pe-samples/7z1900.exe";
    //let _sample = "pe-samples/putty.exe";
    //let _sample = "pe-samples/rdpclip.exe";
    //let _sample = "pe-samples/print.exe";
    //let _sample = "pe-samples/TestClient.exe";
    //let _sample = "pe-samples/vcredist_x64.exe";
    //let _sample = "pe-samples/TestClientx64.exe";
    let _args = ArgumentsParser::new();
    if _args.inputs.is_present("file") {
        let _sample = _args.inputs.value_of("file").unwrap();
        let _pe = PeParser::new(_sample);
        let _file = _pe.inspect_file();
        let _content = serde_json::to_string_pretty(&_file)?;
        if _args.inputs.is_present("output") {
            let _ov = _args.inputs.value_of("output").unwrap();
            let mut _outfile = FileHandler::open(_ov, "crw");
            _outfile.write(&_content).expect("Could Not Write Content to Desired Output File");
        }
        println!("{}", _content);
    }
    /*
    let _pe = PeParser::new(_sample);
    let _file = _pe.inspect_file();
    println!("{}", serde_json::to_string_pretty(&_file)?);
    */
    Ok(())
}