// fn longest<'a> (x:&'a str, y:&'a str)->&'a str{
//  if x.len()<y.len(){
//     x
//  }else{
//     y
//  }
// }

// #[derive(Debug)]
// struct Car<'a>{
//  name: &'a str,
// }
// impl <'a> Car<'a>{
//     fn print(){
//         println!("{:?}",Car{name:"Ford"});
//     }
// }
// fn main() {
//    // println!("Hello, world!");
//    let z:&str=longest("hi","world");
//    println!("{:?}",z);
//    let c=Car{name:"Renault"};
//    println!("Car:: {:?}",c.name);
//    Car::print();
// }

// use std::{fs::{File, OpenOptions}, io::{self, BufRead, BufReader, Read}};

// use io::Write;

// fn write_to_file()->io::Result<()>{
//     let mut file=File::create("io.txt")?;
//     file.write_all(b"Hello")?;

//     Ok(())
// }

// fn append_to_file() -> io::Result<()> {
//     let mut file = OpenOptions::new()
//         .append(true)
//         .open("io.txt")?;
//     file.write_all(b"\nAppending this line.")?;
//     Ok(())
// }
// fn read_from_file()-> io::Result<String>{
// let  file=File::open("io.txt")?;
// let mut reader=BufReader::new(file);
// let mut contents=String::new();
// let _=reader.read_to_string(&mut contents);
// Ok(contents)

// }


// fn read_lines_from_file() -> io::Result<()> {
//     let file = File::open("io.txt")?;
//     let reader = BufReader::new(file);

//     for line in reader.lines() {
//         let line = line?;
//         println!("{}", line);
//     }
//     Ok(())
// }


// fn main(){
// if let  Err(e) = write_to_file(){
// eprintln!("{}",e);
// } 
// match  read_from_file() {
//     Ok(contents)=>println!(" File:: {}",contents),
//     Err(e)=>println!("{}",e),
// }
// if let  Err(e) = append_to_file(){
//     eprintln!("{}",e);
//     } 
   
//     if let  Err(e) = read_lines_from_file(){
//         eprintln!("{}",e);
//         } 

// }

#[derive(Debug)]
struct Car<'a>{
   name: &'a str,
}


fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len()> y.len(){
        x
    }else{
        y
    }
}



fn main() {
   // println!("Hello World");
   let mut x: i32=5;
   let y=&mut x;
   //Car::name("Renault");
   //println!("{:?}",c);
   let z:&str=longest("hi","world");
   println!("{:?}",z);
    
    
    
}

#[cfg(test)]
mod tests{
    use crate::longest;

    #[test]
    fn test_code(){
        let x:&str="big";
        let y:&str="tree";
        longest(x,y);
       assert_eq!("tree",longest(x,y)); 
    }
    }
