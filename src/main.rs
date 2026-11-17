/*=========================Código Completo============================*/

/*
    ====================================================================
    PROJETO: rust_CLI

    ESTRUTURA ORIGINAL DO PROJETO (Modularizado):
    src/
    ├── main.rs
    ├── modelos/
    │   ├── arquivo.rs
    │   ├── diretorio.rs
    │   ├── grupo.rs
    │   ├── permissao.rs
    │   ├── usuario.rs
    │   └── mod.rs
    └── menus/
        ├── menu_arquivos.rs
        ├── menu_diretorios.rs
        ├── menu_grupos.rs
        ├── menu_principal.rs
        ├── menu_usuarios.rs
        └── mod.rs


    Tinhamos feito a estrutura com módulos para facilitar a divisão entre
    a dupla e para ficar melhor organizado.

    Como só podia ser enviado um arquivo .rs, precisamos inserir o código
    de cada uma das pastas dentro da main.rs.
    ====================================================================
*/

// ====================================================================
// MÓDULO: MODELOS (Origem: pasta src/modelos/)
// ====================================================================

pub mod modelos {

    // Origem: src/modelos/usuario.rs
    pub mod arquivo {
        // Essa é a struct "Arquivo" com new, alterar_permissao e stat;
        // Gauga 3 "Permissao" em uma tupla.
        // Usa "permissao.rs".
        // É usado por "diretorio.rs" e "menu_arquivos.rs";

        use crate::modelos::permissao::Permissao;

        pub struct Arquivo {
            pub nome: String,
            pub tamanho: u64,
            pub permissoes: (Permissao, Permissao, Permissao),
            pub uid_dono: u16,
            pub gid_grupo: u16,
        }

        impl Arquivo {
            pub fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo {
                let rw_dono = Permissao::new(true, true, false);
                let rw_grupo = Permissao::new(true, true, false);
                let r_outros = Permissao::new(true, false, false);
                Arquivo {
                    nome,
                    tamanho,
                    permissoes: (rw_dono, rw_grupo, r_outros),
                    uid_dono: uid,
                    gid_grupo: gid,
                }
            }

            pub fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
                self.permissoes = permissoes;
            }

            pub fn stat(&self) {
                println!("Arquivo: {}", self.nome);
                println!("Tamanho: {} bytes", self.tamanho);
                print!(
                    "Permissoes:({}{}{} / ",
                    self.permissoes.0.octal(),
                    self.permissoes.1.octal(),
                    self.permissoes.2.octal()
                );

                if self.permissoes.0.leitura {
                    print!("r");
                } else {
                    print!("-");
                }
                if self.permissoes.0.escrita {
                    print!("w");
                } else {
                    print!("-");
                }
                if self.permissoes.0.execucao {
                    print!("x");
                } else {
                    print!("-");
                }

                if self.permissoes.1.leitura {
                    print!("r");
                } else {
                    print!("-");
                }
                if self.permissoes.1.escrita {
                    print!("w");
                } else {
                    print!("-");
                }
                if self.permissoes.1.execucao {
                    print!("x");
                } else {
                    print!("-");
                }

                if self.permissoes.2.leitura {
                    print!("r");
                } else {
                    print!("-");
                }
                if self.permissoes.2.escrita {
                    print!("w");
                } else {
                    print!("-");
                }
                if self.permissoes.2.execucao {
                    print!("x");
                } else {
                    print!("-");
                }
                println!(")");

                println!("Uid:{}", self.uid_dono);
                println!("Gid:{}", self.gid_grupo);
            }
        }
    }

    // Origem: src/modelos/diretorio.rs
    pub mod diretorio {
        // Struct "Diretorio" que guarda um vetor (Vec<Arquivo>).
        // Usa os métodos new, adiciona_arquivo, remove_arquivo, listar_conteudo.
        // Usa "arquivo.rs" e "permissao.rs".
        // É usado pelo "menu_diretorios.rs", "menu_arquivos.rs" e "main.rs".
        use crate::modelos::arquivo::Arquivo;
        use crate::modelos::permissao::Permissao;

        pub struct Diretorio {
            pub nome: String,
            pub arquivos: Vec<Arquivo>,
            pub permissoes: (Permissao, Permissao, Permissao),
            pub uid_dono: u16,
        }

        impl Diretorio {
            pub fn new(
                nome: String,
                permissoes: (Permissao, Permissao, Permissao),
                uid_dono: u16,
            ) -> Diretorio {
                Diretorio {
                    nome,
                    arquivos: Vec::new(),
                    permissoes,
                    uid_dono,
                }
            }

            pub fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
                self.arquivos.push(arquivo);
            }

            pub fn remove_arquivo(&mut self, nome_arquivo: &str) {
                for n in 0..self.arquivos.len() {
                    if self.arquivos[n].nome == nome_arquivo {
                        self.arquivos.remove(n);
                        break;
                    }
                }
            }

            pub fn listar_conteudo(&self) {
                for n in &self.arquivos {
                    println!("{}", n.nome);
                }
            }
        }
    }

    // Origem: src/modelos/grupo.rs
    pub mod grupo {
        // struct Grupo com lista de UIDs;
        // Usa os métodos new, adiciona_membro, remove_membro, listar_membros.
        // É usado pelo "menu_grupos.rs" e "main.rs".

        pub struct Grupo {
            pub nome: String,
            pub gid: u16,
            pub membros_uids: Vec<u16>,
        }

        impl Grupo {
            pub fn new(nome: String, gid: u16) -> Self {
                Grupo {
                    nome: nome,
                    gid: gid,
                    membros_uids: Vec::new(),
                }
            }

            pub fn adiciona_membro(&mut self, gid: u16) {
                self.membros_uids.push(gid);
            }

            pub fn listar_membros(&self) {
                if self.membros_uids.len() == 0 {
                    println!("Nenhum membro cadastrado.");
                } else {
                    for n in &self.membros_uids {
                        println!("{}", n);
                    }
                }
            }

            pub fn remove_membro(&mut self, uid: u16) {
                for n in 0..self.membros_uids.len() {
                    if self.membros_uids[n] == uid {
                        self.membros_uids.remove(n);
                        break;
                    }
                }
            }
        }
    }

    // Origem: src/modelos/permissao.rs
    pub mod permissao {
        // É a struct "Permissao" (r -- w -- e) com new e octal.
        // Não usa nenhum outro arquivo.
        // É usado pelo "arquivo.rs", pelo "diretorio.rs" e
        // pelos menus que criam ou alteram as permissões de usuário.
        pub struct Permissao {
            pub leitura: bool,
            pub escrita: bool,
            pub execucao: bool,
        }

        impl Permissao {
            pub fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
                Permissao {
                    leitura,
                    escrita,
                    execucao,
                }
            }

            pub fn octal(&self) -> u8 {
                let mut total = 0;
                if self.leitura {
                    total += 4;
                }
                if self.escrita {
                    total += 2;
                }
                if self.execucao {
                    total += 1;
                }
                total
            }
        }
    }

    // Origem: src/modelos/usuario.rs
    pub mod usuario {
        // struct Usuario com lista de id dos grupos;
        // Usa os métodos new, "adiciona_grupo", "remove_grupo", "listar_grupos".
        // É usado pelo "menu_usuarios.rs" e pelo "main.rs".

        pub struct Usuario {
            pub nome: String,
            pub uid: u16,
            pub grupos_ids: Vec<u16>,
        }

        impl Usuario {
            pub fn new(nome: String, uid: u16) -> Self {
                Usuario {
                    nome: nome,
                    uid: uid,
                    grupos_ids: Vec::new(),
                }
            }

            pub fn adiciona_grupo(&mut self, uid: u16) {
                self.grupos_ids.push(uid);
            }

            pub fn listar_grupos(&self) {
                if self.grupos_ids.len() == 0 {
                    println!("Nenhum grupo encontrado.");
                } else {
                    for n in &self.grupos_ids {
                        println!("{}", n);
                    }
                }
            }

            pub fn remove_grupo(&mut self, gid: u16) {
                for n in 0..self.grupos_ids.len() {
                    if self.grupos_ids[n] == gid {
                        self.grupos_ids.remove(n);
                        break;
                    }
                }
            }
        }
    }
}

// ====================================================================
// MÓDULO: MENUS (Origem: pasta src/menus/)
// ====================================================================

pub mod menus {

    // Origem: src/menus/menu_arquivos.rs
    pub mod menu_arquivos {
        // Tem as opções de criação de arquivo,
        // alteração de permissão,
        // verificar status,
        // e delete

        // Usa "Arquivo", "Permissao" e "Diretorio" e é usado pelo "menu_principal.rs"

        use crate::menus::menu_usuarios::{ler_numero, ler_texto};
        use crate::modelos::arquivo::{self, Arquivo};
        use crate::modelos::diretorio::Diretorio;
        use crate::modelos::permissao::Permissao;
    }

    // Origem: src/menus/menu_diretorios.rs
    pub mod menu_diretorios {
        // Cria diretórios, lista conteúdo e apaga
        // Usa "Diretorio" e "Permissao"
        // É usado pelo "menu_principal.rs"

        use crate::menus::menu_usuarios::{ler_numero, ler_texto};
        use crate::modelos::arquivo::Arquivo;
        use crate::modelos::diretorio::Diretorio;
        use crate::modelos::permissao::Permissao;
        use std::io;

        fn criar_diretorio(diretorios: &mut Vec<Diretorio>) {
            println!("Nome do diretório:");
            let nome_dir = ler_texto();

            println!("UID Dono:");
            let uid: u16 = ler_numero();

            let p1 = Permissao::new(true, true, true);
            let p2 = Permissao::new(true, true, true);
            let p3 = Permissao::new(true, false, true);

            let novo_dir = Diretorio::new(nome_dir, (p1, p2, p3), uid);
            diretorios.push(novo_dir);
            println!("Diretório criado.");
        }

        fn listar_diretorios(diretorios: &Vec<Diretorio>) {
            for i in 0..diretorios.len() {
                println!("{}", diretorios[i].nome);
            }
        }

        fn editar_diretorio(diretorios: &mut Vec<Diretorio>) {
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

            for i in 0..diretorios.len() {
                if diretorios[i].nome == nome_dir {
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
    }

    // Origem: src/menus/menu_grupos.rs
    pub mod menu_grupos {
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
    }

    // Origem: src/menus/menu_principal.rs
    pub mod menu_principal {
        // Menu inicial que leva aos submenus. Usa os 4 outros menus e é usado pelo "main.rs"

        use crate::menus::menu_grupos::menu_grupos;
        use crate::menus::menu_usuarios::{ler_numero, menu_usuarios};
        use crate::menus::menu_diretorios::{menu_diretorios};
        use crate::modelos::grupo::Grupo;
        use crate::modelos::diretorio::Diretorio;
        use crate::modelos::usuario::Usuario;

        pub fn menu_principal(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>, diretorios: &mut Vec<Diretorio>) {
            loop {
                println!("=== MENU PRINCIPAL ===");
                println!("1. Menu dos usuários.");
                println!("2. Menu dos grupos.");
                println!("3. Menu dos diretórios.");
                println!("0. Finalizar o programa.");

                let opcao = ler_numero();

                match opcao {
                    1 => menu_usuarios(usuarios, grupos),
                    2 => menu_grupos(grupos, usuarios),
                    3 => menu_diretorios(diretorios),

                    0 => break,
                    _ => println!("Essa opção é inválida."),
                }
            }
        }
    }

    // Origem: src/menus/menu_usuarios.rs
    pub mod menu_usuarios {
        // Cria usuário, adiciona/remove/lista grupos e os apaga.
        // Usa "Usuario".
        // É usado pelo "menu_principal.rs".

        use crate::modelos::grupo::Grupo;
        use crate::modelos::usuario::Usuario;
        use std::io;

        pub fn ler_numero() -> u16 {
            loop {
                let mut entrada = String::new();
                io::stdin()
                    .read_line(&mut entrada)
                    .expect("\nErro na leitura\n");

                match entrada.trim().parse::<u16>() {
                    Ok(numero) => return numero,
                    Err(_) => println!("\nDigite um número válido!\n"),
                }
            }
        }

        pub fn ler_texto() -> String {
            let mut texto = String::new();
            io::stdin()
                .read_line(&mut texto)
                .expect("\nErro de letiura\n");
            texto.trim().to_string()
        }

        pub fn muda_vinculo(
            usuarios: &mut Vec<Usuario>,
            grupos: &mut Vec<Grupo>,
            uid: u16,
            gid: u16,
            conectar: bool,
        ) {
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

                        println!(
                            "\n Conexão criada com sucesso! Usuário {} foi adicionado ao Grupo {}.\n",
                            uid, gid
                        );
                    } else {
                        usuarios[id_usuario].remove_grupo(gid);
                        grupos[id_grupo].remove_membro(uid);

                        println!(
                            "\nConexão desfeita! Usuário {} não pertence mais ao Grupo {}.\n",
                            uid, gid
                        );
                    }
                }

                (None, Some(_)) => println!("\nUsuário não encontrado.\n"),
                (Some(_), None) => println!("\nGrupo não encontrado.\n"),
                (None, None) => println!("\nNão foi encontrado nem grupo e nem usuário.\n"),
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
    }
}

// ====================================================================
// FUNÇÃO PRINCIPAL (Origem: src/main.rs)
// ====================================================================

use crate::menus::menu_principal::menu_principal;
use crate::modelos::grupo::Grupo;
use crate::modelos::usuario::Usuario;

fn main() {
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();

    let mut diretorios: Vec<Diretorio> = Vec::new();

    menu_principal(&mut usuarios, &mut grupos);
}
