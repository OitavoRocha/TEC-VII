//2) Crie uma enumeração chamada Shape que represente diferentes formas geométricas, como círculo, quadrado, triângulo e retângulo. Cada variante deve ter campos associados para armazenar informações relevantes sobre a forma geométrica, como raio, lado ou base/altura. Em seguida, escreva uma função que recebe uma forma geométrica como argumento e calcula e imprime sua área
enum Shape {
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
    Triangle(f32, f32)
}

use Shape::*; // indica que o programa deve resolver um problema usados na enumeraçao, tirando a necessidade de indicar para cada declaração de variavel que elas são do tipo shape

fn calculate_area( s: Shape ) {
    match s {
        Shape::Circle(radius) => {
            let area = radius * radius * std::f32::consts::PI;
            println!("A area do circulo é {}", area);
        },
        Shape::Square(side) => {
            let area = side * side;
            println!("A area do quadrado é {}", area);
        },
        Shape::Rectangle(width, height) => {
            let area = width * height;
            println!("A area do retangulo é {}", area);
        },
        Shape::Triangle(base, height) => {
            let area = (base * height)/2.0;
            println!("A area do triangulo é {}", area);
        },
    }
}

fn main() {
    let shape1 = Circle(5.0);
    let shape2 = Square(5.0);
    let shape3 = Rectangle(2.0, 3.0);
    let shape4 = Triangle(3.0, 3.0);

    calculate_area(shape1);
    calculate_area(shape2);
    calculate_area(shape3);
    calculate_area(shape4);
}
