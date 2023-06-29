//6) Faça uma função que crie retângulos com tamanhos aleatórios (delimite o tamanho dos lados entre 2 e 20).
//7) Implemente um programa que manipule um vetor de retângulos, ordenando-os por tamanho.
use rand::Rng;

struct Retangulo {
    base: i32,
    height: i32,
    area: i32
}

impl Retangulo {
    fn find_area( &mut self ) {
        self.area = self.base * self.height;
    }

    fn create( b: i32, h: i32, a: i32 ) -> Retangulo {
        Retangulo { 
            base: b, 
            height: h, 
            area: a, 
        }
    }
}

fn gera_rets( size:usize ) -> Vec<Retangulo> {
    let mut rng = rand::thread_rng();
    let mut vet = Vec::with_capacity(size);
    for i in 0..size {
        vet.push( Retangulo::create( rng.gen_range(2..20), rng.gen_range(2..20), 0 ) );
        vet[i].find_area();
    }
    vet
}

fn main() {
    let size = 10;
    let mut vet = gera_rets(size);
    for i in 0..size{
        println!("Valores do Retangulo{} -> base: {}  altura: {}  area: {}", i, vet[i].base, vet[i].height, vet[i].area);
    }
}
