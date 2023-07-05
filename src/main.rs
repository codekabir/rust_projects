extern crate flate2;           //emport the extern crate to fetch the module from it

use flate2::write::GzEncoder;  //this mudule have some functionality enconging the data 
use flate2::Compression;       //this mudule have functionality of compress the file 
use std::env::args;           //these all are used for use the cli opretions  
use std::fs::File;                
use std::io::copy;            //this module is will have functionality of capying data from the input    
use std::io::BufReader;        //this will help to read the the data from the buffer
use std::time::Instant;        //this will help to record the time


fn main(){    

    //this line will check the number of argument provide from the command
    if args().len() !=3{
        eprintln!("usage: 'input_file' 'compressed_File' ");
        return;
    }


//this will teke the source file from input
    let mut input =BufReader::new(File::open(args().nth(1).unwrap()).unwrap()); 
    let output =File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder =GzEncoder::new(output,Compression::default());

    // this will be record the time of compression
    let  start =Instant::now();

    
    // this will copy the data into encoder from the input 
    copy(&mut input, &mut encoder).unwrap();

    // this will hold  the output after encoding
    let output = encoder.finish().unwrap();
    println!(
        "input_file  length : {:?}",input.get_ref().metadata().unwrap().len()
    );
     

    //  print the length compressed_file
    println!(
        "compressed_File length : {:?}",output.metadata().unwrap().len()
    );

    // will print compressed file time
    println!("compression time: {:?}",start.elapsed());

}
