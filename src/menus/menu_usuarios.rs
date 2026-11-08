// Cria usuário, adiciona/remove/lista grupos e os apaga.
// Usa "Usuario".
// É usado pelo "menu_principal.rs".

use std::io;
use crate::modelos::usuario::Usuario;
use crate::modelos::grupo::Grupo;

pub fn ler_numero() -> u16 {

    loop {
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("\nErro na leitura\n");
        
        match entrada.trim().parse::<u16>() {
            Ok(numero) => return numero,
            Err(_) => println!("\nDigite um número válido!\n"),
        }
    }
}






pub fn ler_texto() -> String {
    let mut texto = String::new();
    io::stdin().read_line(&mut texto).expect("\nErro de letiura\n");
    texto.trim().to_string()
}




pub fn muda_vinculo(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>, uid: u16, gid: u16, conectar: bool) {

    let mut posicao_usuario = None;
    for i in 0..usuarios.len() {
        if usuarios[i].uid == uid {
            posicao_usuario = Some(i);
            break;
        }
    }

    let mut posicao_grupo = None;
    for i in 0..grupos.len() {
        if grupos[i].gid == gid {
            posicao_grupo = Some(i);
            break;
        }
    }


    match (posicao_usuario, posicao_grupo) {
        (Some(id_usuario), Some(id_grupo)) => {
            
            if conectar {

                usuarios[id_usuario].adiciona_grupo(gid);
                grupos[id_grupo].adiciona_membro(uid);

                println!("\n Conexão criada com sucesso! Usuário {} foi adicionado ao Grupo {}.\n", uid, gid);

        } else {

            usuarios[id_usuario].remove_grupo(gid);
            grupos[id_grupo].remove_membro(uid);

            println!("\nConexão desfeita! Usuário {} não pertence mais ao Grupo {}.\n", uid, gid);

        }
    }

        (None, Some(_)) => println!("\nUsuário não encontrado.\n"),
        (Some(_), None) => println!("\nGrupo não encontrado.\n"),
        (None, None) => println!("\nNão foi encontrado nem grupo e nem usuário.\n")
    }
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


fn adicionar_grupo_usuario(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    println!("\nQual o uid do usuário: \n");
    let uid = ler_numero();
    println!("\nDigite o gid do grupo: \n");
    let gid = ler_numero();

    muda_vinculo(usuarios, grupos, uid, gid, true);

}



fn remover_grupo_usuario(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    println!("\nQual o uid do usuário: \n");
    let uid = ler_numero();
    println!("\nDigite o gid do grupo: \n");
    let gid = ler_numero();

    muda_vinculo(usuarios, grupos, uid, gid, false);

}



fn listar_grupo_usuario(usuarios: &Vec<Usuario>) {
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


pub fn menu_usuarios(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    loop {
        println!("\n--- Usuários ---\n");
        println!("1. Criar usuário");
        println!("2. Listar usuários");
        println!("3. Adicionar grupo");
        println!("4. Listar grupos de um usuário");
        println!("5. remover grupo de um usuário");
        println!("0. Voltar\n");

        let opcao = ler_numero();

        match opcao {
            
            1 => criar_usuario(usuarios),
            2 => listar_usuario(usuarios),
            3 => adicionar_grupo_usuario(usuarios, grupos),
            4 => listar_grupo_usuario(usuarios),
            5 => remover_grupo_usuario(usuarios, grupos),
            0 => break,
            _ => println!("\nA opção {} não existe.\n", opcao),
        }
    }
}