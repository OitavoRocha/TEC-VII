fn rectangle( base:i32, height:i32 ) {
    for _height in 0..height {
        for _base in 0..base {
            print!("*");
        }
        println!("");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Informe dois inteiro, base e altura");
        return;
    }

    let base: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Argumento não é inteiro válido!");
            return;
        }
    };

    let height: i32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Argumento não é inteiro válido!");
            return;
        }
    };

    rectangle(base, height);
}
