use std::io;

fn main() {
    loop {
        println!("Enter any number kid!");
        let mut input_text = String::new();
        io::stdin()
        .read_line(&mut input_text)
        .expect("Something is wrong...");
        
        let input: i32 = input_text.trim().parse().expect("Well yeah, not a number yk");
        println!("Enter a operator!");

        let mut options = String::new();
        io::stdin()
        .read_line(&mut options)
        .expect("Yeah, no");
        let typing = options.trim();

         println!("Enter your second number!");
        let mut input_text2 = String::new();

        io::stdin()
        .read_line(&mut input_text2)
        .expect("Nuh uh");
        let input2: i32 = input_text2.trim().parse().expect("Yeah kid");
        println!("=");
        let result = match typing {
            "x" => input * input2,
            "/" => input / input2,
            "+" => input + input2,
            "-" => input - input2,
            _ => 0,
        };


        println!("So the full calculation is: {} {} {} = {} ", input, typing, input2, result);

        break;

    }
}
