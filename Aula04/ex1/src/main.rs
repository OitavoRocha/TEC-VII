//Implemente uma função "swap", que receba dois valores inteiros e os troque.

fn swap( a: &mut i32, b: &mut i32 ) {
    let buffer: i32 = *a;

    *a = *b;
    *b = buffer;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Informe dois inteiro, base e altura");
        return;
    }

    let mut a: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Argumento não é inteiro válido!");
            return;
        }
    };

    let mut b: i32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Argumento não é inteiro válido!");
            return;
        }
    };

    swap( &mut a, &mut b );

    println!("a: {} b: {}", a , b);
}
