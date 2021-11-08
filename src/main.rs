// u8   = (0 => 255)
// u16  = (0 => 65,535)
// u32  = (0 => 4,294,967,295)
// u64  = (0 => 18,446,744,073,709,551,615)
// u128 = (0 => 340,282,366,920,938,463,463,374,607,431,768,211,455)

// i8   (-128 => 127)
// i16  (-32,768 => 32,767)
// i32  (-2,147,483,648 => 2,147,483,647)
// i64  (-9,223,372,036,854,775,808 => 9,223,372,036,854,775,807)
// i128 (-170,141,183,460,469,231,731,687,303,715,884,105,728
//          => 170,141,183,460,469,231,731,687,303,715,884,105,727)

mod calculator;
use calculator::*;

use std::io::{stdin, stdout, Write};

fn main() {
    // title
    println!("welcome to the calculator");
    println!("----------");

    // initialize variables
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    loop {
        // get user input
        println!("what is the first number? ");
        read(&mut num1);
        println!("what is the second number? ");
        read(&mut num2);
        println!("what operation would you like to do? [*+-/]:  ");
        read(&mut operator);

        // clean up input
        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        // check for valid operator
        let operators = String::from("+-*/");
        if !operators.contains(operator) {
            println!("unknown operator");
            return;
        }

        // calculate results based on operator
        let calc = Calculator::new(num1, num2);
        let result = match operator {
            '+' => calc.add(),
            '-' => calc.subtract(),
            '*' => calc.multiply(),
            '/' => calc.divide(),
            _ => panic!("error in operator"),
        };

        println!("the results of {} {} {} = {}", num1, operator, num2, result);
    }
}

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}
