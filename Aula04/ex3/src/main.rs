//Implemente um programa que inicialize preencha a diagonal superior de uma matriz quadrada com o valor 1, a diagonal com o valor 0 e a diagonal inferior com o valor -1. Imprima o resultado.
fn main() {
    //let args: Vec<String> = std::env::args().collect();

    let mut m:[[i32; 5]; 5] = [[0;5];5];

    for i in 0..5 {
        for j in 0..5 {
            if i > j {
                m[i][j] = -1;
            } else if i < j {
                m[i][j] = 1;
            }
        }
    }

    for i in 0..5  {
        for j in 0..5 {
            print!("{} ", m[i][j]);
        }
        println!(" ");
    }
}
