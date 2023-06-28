// Na passagem de um Vec<> não é possível passar seus valores inteiros como mutáveis, pois o próprio Vec foi declarado mutável, na função swap
use rand::Rng;

fn gera_vet(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vet = Vec::with_capacity(size);
    for _ in 0..size {
        vet.push(rng.gen_range(0..100));
    }
    vet
}

fn swap( a: &mut i32, b: &mut i32 ) {
    let buffer: i32 = *a;

    *a = *b;
    *b = buffer;
}

fn bubble_sort( vet: &mut Vec<i32> ) {
    let n = vet.len();

    for _i in 0..n-1 {
        for j in 0.. n-1 {
            if vet[j] > vet[j+1] {
                swap(&mut vet[j],&mut vet[j+1] )
            }
        }
    }
}

fn main() {
    let size = 10;
    let mut vet = gera_vet(size);
    println!("Vetor original: {:?}", vet);
    bubble_sort(&mut vet); // Opa... isso não existe (ainda)
    println!("Vetor ordenado: {:?}", vet);
}

