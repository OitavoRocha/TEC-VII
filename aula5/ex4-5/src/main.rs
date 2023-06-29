//4) Refaça o exemplo do ponto usando uma estrutura para Retangulo.
//5) Inclua na estrutura Retangulo um campo área e um método para calcular a área.
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

fn main() {
    let mut ret = Retangulo::create(10, 20,-1 );
    ret.find_area();
    println!("Valores do Retangulo -> base: {}  altura: {}  area: {}", ret.base, ret.height, ret.area);
}
