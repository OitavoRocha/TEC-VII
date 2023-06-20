//Faça um programa que crie um array com 10 elementos iniciados com valores gerados randomicamente.
//Como gerar numeros aleatorios?
//-----------------------------------------------------------------------------

extern crate rand;

use rand::Rng;

fn main() {
    let mut array: [i32; 10] = [0; 10];
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen();

    for _index in 0..10 {
        array[_index] = n;
    }

    for _index in 0..10 {
        print!("{} ", array[_index]);
    }
}
