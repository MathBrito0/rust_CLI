// Cria usuário, adiciona/remove/lista grupos e os apaga.
// Usa "Usuario".
// É usado pelo "menu_principal.rs".

use std::io;

pub fn menu_usuarios() {
    loop {
        println!("--- Usuários ---\n");
        println!("1. Criar usuário");
        println!("2. Listar usuários");
        println!("0. Voltar\n");

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("\nErro na leitura!\n");

        match entrada.trim().parse::<u16>() {
            Ok(opcao) => {
                match opcao {
                    1 => println!("Vamos criar seu usuário"),
                    2 => println!("Vamos listar os usuários existentes"),
                    0 => break,
                    _ => println!("\n{} não é uma opção válida.\n", opcao),
                }
            }
            Err(_) => println!("\nEntrada inválida. Digite apenas números!\n"),
        }
    }
}