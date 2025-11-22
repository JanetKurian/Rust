
use std::env::{args,Args};

fn main() {
    //println!("Hello, world!");
    let mut args:Args=args(); // to get rid of waarning: Args not used
    let first=args.nth(1).unwrap();// unwrap on a none value
    //let operator =args.nth(2).unwrap();
     let operator =args.nth(0).unwrap().chars().next().unwrap();
   let second=args.nth(0).unwrap();
    println!("{:?} {} {}",first,operator,second); //nth-> iterator will iterate
 /*let first=args.nth(1).unwrap();// unwrap on a none value
    let operator =args.nth( let first=args.nth(1).unwrap();// unwrap on a none value
    let operator =args.nth(2).unwrap();
    let second=args.nth(3).unwrap(0); // to reinintilaise the nth position in place of .next()
    let second=args.nth(3).unwrap(0);*/
let first_number=first.parse::<f32>().unwrap();
let second_number=second.parse::<f32>().unwrap();
let result= operate(operator, first_number, second_number);
let out= output(operator,first_number,second_number,result);
println!("{:?}",result); //nth-> iterator will iterate

}
fn operate(operator:char,first_number:f32,second_number:f32)-> f32{
    if operator =='+'{
        return first_number+second_number; 
    }else if operator=='-'{
return first_number-second_number; 
    }else if operator=='/'{
return first_number-second_number; 
    }else if operator=='*'{
return first_number-second_number; 
    }else {
        0f32 
    }

}
fn output(operator:char,first_number:f32,second_number:f32,result:f32)->String{
format!("{} {} {} = {}", first_number,operator,second_number,result)
}
