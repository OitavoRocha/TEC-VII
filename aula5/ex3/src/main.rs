use rand::Rng;
use std::time::Instant;

fn gera_vet(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vet = Vec::with_capacity(size);
    for _ in 0..size {
        vet.push(rng.gen_range(0..100));
    }
    vet
}

fn bubble_sort( vet: &mut Vec<i32> ) {
    let n = vet.len();

    for _i in 0..n-1 {
        for j in 0.. n-1 {
            if vet[j] > vet[j+1] {
                vet.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let size = 10000;
    let mut vet = gera_vet(size);

    let inicio = Instant::now();
    bubble_sort(&mut vet); // Opa... isso não existe (ainda)
    let term = Instant::now() - inicio;

    println!("Tempo da operação: {:?}", term);
}
