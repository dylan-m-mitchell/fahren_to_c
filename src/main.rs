use std::io;

fn main() {

    loop{
        let mut init_temp = String::new();
        

        println!("Input temperature.");

        io::stdin()
            .read_line(&mut init_temp)
            .expect("Failed to read line");


        let init_temp: f32 = match init_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Fahrenheit or Celsius?");
        println!("Type F/C:");
        'conversion: loop{
            let mut conv = String::new();
            
            io::stdin()
                .read_line(&mut conv)
                .expect("Failed to read line");

            let conv: String = conv.trim().parse().expect("Please type a letter!");
            let conv = conv.to_lowercase();
            
        
            if conv == "f" {
                let final_value = fahrenheit_celsius(init_temp);
                println!("Temp in Celsius: {:.2}", final_value);
                break 'conversion;
            } 
            else if conv == "c" {
                let final_value = celsius_fahrenheit(init_temp);
                println!("Temp in Fahrenheit: {:.2}", final_value);
                break 'conversion;
            } else {
                println!("Please type F or C!");
                
            }
        }
        println!("Press any key to try again or 'q' to quit.");
        let mut leave = String::new();
        io::stdin()
            .read_line(&mut leave)
            .expect("Letters please.");
        let leave: String = leave.trim().parse().expect("Press any key!");
        if leave.to_lowercase() == "q" {
            break;
        } else {
            continue;
        }
    }
}

fn fahrenheit_celsius(value: f32) -> f32 {
    let temp = (value - 32.0) * (5.0/9.0);
    return temp;
}

fn celsius_fahrenheit(value: f32) -> f32 {
    let temp = value * (9.0/5.0) + 32.0;
    return temp
}
