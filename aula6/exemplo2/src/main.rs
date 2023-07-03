enum Opcao {
    Valor(i32),
    Nenhum
}

fn main() {
    let opcao1 = Opcao::Valor(2);
    let opcao2 = Opcao::Nenhum;

    match opcao1 {
        Opcao::Valor(x) if x > 10000 => {
            println!("Opção com valor: {}, que é bem grande!", x);
        },
         Opcao::Valor(x) if x < 10 => {
            println!("Opção com valor: {}, que é bem pequeno!", x);
        },
        Opcao::Valor(x)  => {
            println!("Opção com valor: {}, que não é pequeno nem grande!", x);
        },
         Opcao::Nenhum => {
            println!("Opção vazia.");
        }
    }

    match opcao2 {
        Opcao::Valor(x) => {
            println!("Opção com valor: {}", x);
        },
        Opcao::Nenhum => {
            println!("Opção vazia.");
        }
    }
}
