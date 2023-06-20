// Faça um programa que receba um valor por parâmetro e imprima outro triangulo na tela com o caracter *.
// --------------------------------------------------------------------------
fn reverse_triangle( height:i32 ) {
    let mut n: i32 = height - 1;
    let mut m: i32 = 1;
    for _index in 0..height {
        for _index2 in 0..n {
            print!(" ");
        }
        for _index2 in 0..m {
            print!("*");
        }
        println!("");
        n = n - 1;
        m = m + 1;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Informe um valor inteiro.");
        return;
    }

    let height: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("O argumento não é válido");
            return;
        }
    };

    reverse_triangle(height);
}
