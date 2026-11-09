// Tem as opções de criação de arquivo,
// alteração de permissão,
// verificar status,
// e delete

// Usa "Arquivo", "Permissao" e "Diretorio" e é usado pelo "menu_principal.rs"

use crate::modelos::arquivo::Arquivo;
use crate::modelos::diretorio::Diretorio;
use crate::modelos::permissao::Permissao;
use crate::menus::menu_usuarios::{ler_numero, ler_texto};


fn criar_arquivo(arquivos: &mut Vec<Arquivo>){
    println!("Nome do arquivo:");
    let nome = ler_texto();

    println!("Tamanho do arquivo:");
    let tamanho: u64 = ler_numero().into();

    println!("UID do dono:");
    let uid: u16 = ler_numero();

    println!("GID do grupo:");
    let gid: u16 = ler_numero();

    let novo = Arquivo::new(nome, tamanho, uid, gid);
    arquivos.push(novo);
    println!("Arquivo criado.");

}

fn listar_arquivos(arquivos: &Vec<Arquivo>){
    for i in 0..arquivos.len(){
        println!("{}", arquivos[i].nome);
    }

}

fn ver_stat(arquivos: &Vec<Arquivo>){
    println!("Nome do arquivo:");
    let nome = ler_texto();

    for i in 0..arquivos.len() {
        if arquivos[i].nome == nome {
            arquivos[i].stat();
            return;
        }
    }
    println!("Arquivo não encontrado.");
}

fn ler_permissao(tipo: &str) -> Permissao {
    println!("Permissões de {} (sim/não):", tipo);

    println!("Permissão para leitura: ");
    let leitura = ler_texto() == "sim";

    println!("Permissão para escrita: ");
    let escrita = ler_texto() == "sim";

    println!("Permissão para execução: ");
    let execucao = ler_texto() == "sim";

    Permissao::new(leitura, escrita, execucao)
}

fn alterar_permissao(arquivos: &mut Vec<Arquivo>){
    println!("Nome do arquivo:");
    let nome = ler_texto();

    for i in 0..arquivos.len() {
        if arquivos[i].nome == nome {
            let dono = ler_permissao("dono");
            let grupo = ler_permissao("grupo");
            let outros = ler_permissao("outros");

            arquivos[i].alterar_permissao((dono, grupo, outros));
            println!("Permissões alteradas.");
            return;
        }
    }
    println!("Arquivo não encontrado.");
}

fn remover_arquivo(arquivos: &mut Vec<Arquivo>){
    println!("Nome do arquivo:");
    let nome_arq = ler_texto();

    for i in 0..arquivos.len(){
        if arquivos[i].nome == nome_arq{
            arquivos.remove(i);
            break;
        }
    }
    
}


pub fn menu_arquivos(arquivos: &mut Vec<Arquivo>) {
    loop {
        println!("\n=== ARQUIVOS ===\n");
        println!("1. Criar arquivo");
        println!("2. Listar arquivos");
        println!("3. Ver stat");
        println!("4. Alterar permissão");
        println!("5.Remover arquivo");
        println!("0. Voltar\n");

        let opcao = ler_numero();

        match opcao {
            1 => criar_arquivo(arquivos),
            2 => listar_arquivos(arquivos),
            3 => ver_stat(arquivos),
            4 => alterar_permissao(arquivos),
            5 => remover_arquivo(arquivos),
            0 => break,
            _ => println!("A opção {} não existe.", opcao),
        }
    }
}
