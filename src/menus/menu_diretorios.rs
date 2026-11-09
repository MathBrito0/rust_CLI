// Cria diretórios, lista conteúdo e apaga
// Usa "Diretorio" e "Permissao"
// É usado pelo "menu_principal.rs"

use std::io;
use crate::menus::menu_usuarios::{ler_numero, ler_texto};
use crate::modelos::diretorio::Diretorio;
use crate::modelos::permissao::Permissao;
use crate::modelos::arquivo::Arquivo;

fn criar_diretorio(diretorios: &mut Vec<Diretorio>){
    println!("Nome do diretório:");
    let nome_dir = ler_texto();

    println!("UID Dono:");
    let uid: u16 = ler_numero();

    let p1 = Permissao::new(true, true, true);    
    let p2 = Permissao::new(true, true, true);    
    let p3 = Permissao::new(true, false, true);  

    let novo_dir = Diretorio::new(nome_dir, (p1,p2,p3), uid);
    diretorios.push(novo_dir);
    println!("Diretório criado.");

}

fn listar_diretorios(diretorios: &Vec<Diretorio>){
    for i in 0..diretorios.len(){
        println!("{}", diretorios[i].nome);
    }

}

fn editar_diretorio(diretorios:&mut Vec<Diretorio>){
    println!("Nome do diretório:");
    let nome_dir = ler_texto();

    for dir in diretorios.iter_mut() {
        if dir.nome == nome_dir {
            println!("1.Adicionar arquivo");
            println!("2.Remover arquivo");
            println!("3.Listar conteúdo");

            let mut t = String::new();
            io::stdin().read_line(&mut t).expect("Erro ao ler");
            let opcao: u32 = t.trim().parse().expect("Digite um número");

            match opcao {
                1 => {
                    println!("Digite o nome do arquivo: ");
                    let nome = ler_texto();

                    println!("Digite o tamanho do arquivo: ");
                    let tamanho: u64 = ler_numero().into();

                    println!("Digite o ID do usuário: ");
                    let uid: u16 = ler_numero();

                    println!("Digite o ID do grupo: ");
                    let gid: u16 = ler_numero();

                    let arq = Arquivo::new(nome, tamanho, uid, gid);
                    dir.adiciona_arquivo(arq);
                    println!("Arquivo adicionado.");
                }
                2 => {
                    println!("Digite o nome do arquivo: ");
                    let nome = ler_texto();
                    dir.remove_arquivo(&nome);
                    println!("O arquivo {} foi removido.", nome);
                  
                }
                3 => dir.listar_conteudo(),
                _ => println!("Opção inválida."),
            }
            return;
        }
    }
    println!("Diretório não encontrado.");


}

fn remover_diretorio(diretorios: &mut Vec<Diretorio>) {
    println!("Nome do diretório:");
    let nome_dir = ler_texto();

    for i in 0..diretorios.len(){
        if diretorios[i].nome == nome_dir{
            diretorios.remove(i);
            break;
        }
    }
    

}


pub fn menu_diretorios(arquivos: &mut Vec<Arquivo>, diretorios: &mut Vec<Diretorio>) {
    loop {
        println!("\n=== DIRETORIOS ===\n");
        println!("1. Criar diretório");
        println!("2. Listar diretórios");
        println!("3. Editar diretório");
        println!("4. Remover diretório");
        println!("0. Voltar\n");

        let opcao = ler_numero();

        match opcao {
            1 => criar_diretorio(diretorios),
            2 => listar_diretorios(diretorios),
            3 => editar_diretorio(diretorios),
            4 => remover_diretorio(diretorios),
            0 => break,
            _ => println!("A opção {} não existe.", opcao),
        }
    }
}
