// Cria usuário, adiciona/remove/lista grupos e os apaga.
// Usa "Usuario".
// É usado pelo "menu_principal.rs".

use std::io;
use crate::modelos::usuario::Usuario;

fn ler_numero() -> u16 {

    loop {
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("\nErro na leitura\n");
        
        match entrada.trim().parse::<u16>() {
            Ok(numero) => return numero,
            Err(_) => println!("\nDigite um número válido!\n"),
        }
    }
}






fn ler_texto() -> String {
    let mut texto = String::new();
    io::stdin().read_line(&mut texto).expect("\nErro de letiura\n");
    texto.trim().to_string()
}






fn criar_usuario(usuarios: &mut Vec<Usuario>) {
    println!("Digite seu nome: ");
    let nome = ler_texto();
    println!("Digite o id desse usuário:");
    let uid = ler_numero();
    let novo = Usuario::new(nome, uid);
    usuarios.push(novo);
    println!("Usuário criado.");
}






fn listar_usuario(usuarios: &Vec<Usuario>) {
    if usuarios.len() == 0 {
        println!("Nenhum usuário cadastrado.");
        return;
    }

    for i in 0..usuarios.len() {
        println!("{} tem o UID {}", usuarios[i].nome, usuarios[i].uid);
    }
}


fn adicionar_grupo_usuario(usuarios: &mut Vec<Usuario>) {
    println!("\nQual o uid do usuário: \n");
    let uid = ler_numero();
    println!("\nDigite o gid do grupo: \n");
    let gid = ler_numero();

    let mut numero_chegada = 0;
    let mut existe = false;

    for i in 0..usuarios.len() {
        if (usuarios[i].uid) == uid {
            numero_chegada = i;
            existe = true;
            break;
        }
    }

    if existe {
            usuarios[numero_chegada].adiciona_grupo(gid);
            println!("\nUsuário adicionado no grupo {}. \n", gid)
        } else {
            println!("\nUsuário não encontrado.\n");
        }

}


fn listar_grupo_usuario(usuarios: &mut Vec<Usuario>) {
    println!("\nQual o uid do usuário: \n");
    let uid = ler_numero();

    let mut numero_chegada = 0;
    let mut existe = false;

    for i in 0..usuarios.len() {
        if (usuarios[i].uid) == uid {
            numero_chegada = i;
            existe = true;
            break;
        }
    }

    if existe {
            println!("\nEste usuário faz parte dos seguintes grupos: \n");
            usuarios[numero_chegada].listar_grupos();
        } else {
            println!("\nUsuário não encontrado.\n");
        }

}


pub fn menu_usuarios(usuarios: &mut Vec<Usuario>) {
    loop {
        println!("\n--- Usuários ---\n");
        println!("1. Criar usuário");
        println!("2. Listar usuários");
        println!("3. Adicionar grupo");
        println!("4. Listar grupos de um usuário");
        println!("0. Voltar\n");

        let opcao = ler_numero();

        match opcao {
            
            1 => criar_usuario(usuarios),
            2 => listar_usuario(usuarios),
            3 => adicionar_grupo_usuario(usuarios),
            4 => listar_grupo_usuario(usuarios),
            0 => break,
            _ => println!("\nA opção {} não existe.\n", opcao),
        }
    }
}