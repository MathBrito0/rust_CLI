// Tem as opções de criação de arquivo,
// alteração de permissão,
// verificar status,
// e delete

// Usa "Arquivo", "Permissao" e "Diretorio" e é usado pelo "menu_principal.rs"

use crate::modelos::arquivo::{self, Arquivo};
use crate::modelos::diretorio::Diretorio;
use crate::modelos::permissao::Permissao;
use crate::menus::menu_usuarios::{ler_numero, ler_texto};


pub fn criar_arquivo(diretorios: &mut Vec<Diretorio>) {
    let posicao = match escolher_diretorio(diretorios) {
        Some(p) => p,
        None => return,
    };

    println!("Nome do arquivo:");
    let nome = ler_texto();
    println!("Tamanho em bytes:");
    let tamanho = ler_numero() as u64;
    println!("UID do dono: ");
    let uid = ler_numero();
    println!("GID do grupo: ");
    let gid = ler_numero();


    let novo_arquivo = Arquivo::new(nome, tamanho, uid, gid);
    diretorios[posicao].adiciona_arquivo(novo_arquivo);
    println!("\nArquivo criado.\n");
}


pub fn alterar_permissao() {

}


pub fn verificar() {

}


pub fn menu_arquivos(diretorios: &mut Vec<Diretorio>) {
    loop {
        println!("\n=== GRUPOS ===\n");
        println!("1. Criar arquivo");
        println!("2. Alterar permissões");
        println!("3. Acessar um arquivo");
        println!("4. Listar arquivos de um diretório");
        println!("5. Apagar um arquivo");
        println!("0. Voltar\n");

        let opcao = ler_numero();

        match opcao {
            1 => criar_arquivo(arquivos),
            2 => alterar_permissao(),
            3 => adicionar_membro_grupo(grupos, usuarios),
            4 => listar_conteudo(diretorios),
            5 => remover_arquivo(diretorios),
            0 => break,
            _ => println!("\nA opção {} não existe.\n", opcao),
        }
    }
}