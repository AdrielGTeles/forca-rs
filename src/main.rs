use std::io;


fn main() {

    loop {
        //limpar_tela();
        //variavel global para as opções do menu
        const OPTIONS: &[&str] = & [
            "SOLO",
            "VS",
            "LISTA DE PLAYERS",
            "SAIR"
        ];

        //apresentação do menu
        println!("JOGO DA FORCA");
        for (index, option) in OPTIONS.iter().enumerate() {
            println!("{} - {}", index + 1, option);
        }
        
        //leitura da escolha do usuário
        let mut choice = String::new();
        println!("Opção:");
        io::stdin()
            .read_line(&mut choice)
            .expect("Falha ao ler a entrada");

        //conversão da escolha para número
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, insira um número válido.");
                continue;
            }
        };
        
        //processamento da escolha do usuário
        match choice {
            1 => {
                println!("Modo SOLO selecionado.");
                // Lógica para o modo SOLO
            },
            2 => {
                println!("Modo VS selecionado.");
                // Lógica para o modo VS
            },
            3 => {
                println!("Lista de Players:");
                // Lógica para exibir a lista de players
            },
            4 => {
                println!("Saindo do jogo. Até mais!");
                break;
            },
            _ => {
                println!("Opção inválida. Por favor, tente novamente.");
                //continua o loop
            },
        }
    }
}


fn _limpar_tela() {
    print!("\x1B[2J\x1B[1;1H");
}