fn compare_and_swap( ant: &mut i32, novo: i32 , comp: i32 ) {
    if *ant != comp {
        *ant = novo;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        println!("Informe dois inteiro, base e altura");
        return;
    }

    let mut ant: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Argumento não é inteiro válido!");
            return;
        }
    };

    let novo: i32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Argumento não é inteiro válido!");
            return;
        }
    };

    let comp: i32 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Argumento não é inteiro válido!");
            return;
        }
    };
    
    println!("valor atual: {}", ant);

    compare_and_swap( &mut ant, novo, comp );

    println!("valor atual: {}", ant);
}
