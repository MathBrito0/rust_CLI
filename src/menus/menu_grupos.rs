// Cria grupos.
// Adiciona, remove e lista os membros.
// Usa "Grupo".
// É usado pelo "menu_principal.rs".


use crate::menus::menu_usuarios::{ler_numero, ler_texto, muda_vinculo};
use crate::modelos::grupo::Grupo;
use crate::modelos::usuario::Usuario;

fn criar_grupo(grupos: &mut Vec<Grupo>) {
    println!("\nDigite o nome do grupo: \n");
    let nome = ler_texto();
    println!("\nDigite o gid desse grupo: \n");
    let gid = ler_numero();
    let novo = Grupo::new(nome, gid);
    grupos.push(novo);
    println!("\nGrupo criado.\n");
}



fn listar_grupo(grupos: &Vec<Grupo>) {
    if grupos.len() == 0 {
        println!("Nenhum grupo registrado.");
        return;
    }

    for i in 0..grupos.len() {
        println!("{} tem o GID {}", grupos[i].nome, grupos[i].gid);
    }
}



fn adicionar_membro_grupo(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    println!("\nQual o GID do grupo: \n");
    let gid = ler_numero();
    println!("\nQual o UID do usuário: \n");
    let uid = ler_numero();

    muda_vinculo(usuarios, grupos, uid, gid, true);
}




fn remover_membro_grupo(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    println!("\nQual o GID do grupo: \n");
    let gid = ler_numero();
    println!("\nQual o UID do usuário: \n");
    let uid = ler_numero();

    muda_vinculo(usuarios, grupos, uid, gid, false);
}



fn listar_membros_grupo(grupos: &Vec<Grupo>) {
    println!("\nQual o GID do grupo: \n");
    let gid = ler_numero();

    let mut posicao = 0;
    let mut existe = false;

    for i in 0..grupos.len() {
        if grupos[i].gid == gid {
            posicao = i;
            existe = true;
            break;
        }
    }

    if existe {
        println!("\nNeste grupo, temos os seguintes usuários: \n");
        grupos[posicao].listar_membros();
    } else {
        println!("\nGrupo não encontrado.\n");
    }
}





pub fn menu_grupos(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    loop {
        println!("\n=== GRUPOS ===\n");
        println!("1. Criar grupo");
        println!("2. Listar grupos");
        println!("3. Adicionar membro");
        println!("4. Listar membros de um grupo");
        println!("5. Remover membros de um grupo");
        println!("0. Voltar\n");

        let opcao = ler_numero();

        match opcao {
            1 => criar_grupo(grupos),
            2 => listar_grupo(grupos),
            3 => adicionar_membro_grupo(grupos, usuarios),
            4 => listar_membros_grupo(grupos),
            5 => remover_membro_grupo(grupos, usuarios),
            0 => break,
            _ => println!("\nA opção {} não existe.\n", opcao),
        }
    }
}
