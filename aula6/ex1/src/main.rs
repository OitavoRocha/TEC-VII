//1) Faça uma função que receba um valor inteiro que represente a idade de uma pessoa e imprima a sua faixa etária. Bebê: Recém-nascido até 2 anos de idade. Criança: 2 anos até 12 anos de idade. Adolescente: 13 anos até 19 anos de idade. Jovem adulto: 20 anos até 29 anos de idade. Adulto jovem: 30 anos até 39 anos de idade. Adulto de meia-idade: 40 anos até 59 anos de idade. Idoso: 60 anos em diante. Lembre: 13..19 inclui o 13, mas não o 19. 13..=19, inclui tanto o 13 como o 19.
fn imprime_faixa_etaria( idade: u32) {
    match idade {
        0 | 1 => {
            println!("Recém-nascido!");
        },
        2..=12 => {
            println!("Criança!");
        },
        13..=19 => {
            println!("Adolescente!");
        },
        20..=29 => {
            println!("Jovem Adulto!");
        },
        30..=39 => {
            println!("Adulto Jovem!");
        },
        40..=59 => {
            println!("Adulto de meida idade!");
        },
        _ => {
            println!("Idoso!");
        },
    }
}

fn main() {
    let size = 7;
    let mut idade: u32 = 1;

    for _i in 0..size {
        imprime_faixa_etaria( idade );
        idade = idade + 10;
    }
}
