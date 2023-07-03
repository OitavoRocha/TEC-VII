enum Cor {
    Vermelho,
    Verde,
    Azul,
    Amarelo,
}

fn imprimir_mensagem_cor(cor: Cor) {
    match cor {
        Cor::Vermelho => {
            println!("A cor é vermelho");
        },
        Cor::Verde => {
            println!("A cor é verde");
        },
        Cor::Azul => {
            println!("A cor é azul");
        },
        Cor::Amarelo => {
            println!("A cor é amarelo");
        },
    }
}

fn main() {
    let cor1 = Cor::Vermelho;
    let cor2 = Cor::Azul;

    imprimir_mensagem_cor(cor1);
    imprimir_mensagem_cor(cor2);
}
