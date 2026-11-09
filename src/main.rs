use crate::menus::menu_principal::menu_principal;
use crate::modelos::grupo::Grupo;
use crate::modelos::usuario::Usuario;

mod menus;
mod modelos;

fn main() {
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();

    // MIDORI --    let mut diretorios: Vec<Diretorio> = Vec::new();

    menu_principal(&mut usuarios, &mut grupos);
}

/*=========================Código Completo============================*/

/*
    ====================================================================
    PROJETO: rust_CLI

    // Tabalho de autoria da dupla:
    // Helen Midori e Matheus Brito
    
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
        // Cole aqui todo o conteúdo de src/modelos/arquivo.rs
    }

    // Origem: src/modelos/diretorio.rs
    pub mod diretorio {
        // Colar aqui todo o conteúdo de src/modelos/diretorio.rs
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
        // Cole aqui todo o conteúdo de src/modelos/permissao.rs
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
        // Cole aqui todo o conteúdo de src/menus/menu_arquivos.rs
    }


    // Origem: src/menus/menu_diretorios.rs
    pub mod menu_diretorios {
        // Cole aqui todo o conteúdo de src/menus/menu_diretorios.rs
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
        use crate::modelos::grupo::Grupo;
        use crate::modelos::usuario::Usuario;

        pub fn menu_principal(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
            loop {
                println!("=== MENU PRINCIPAL ===");
                println!("1. Menu dos usuários.");
                println!("2. Menu dos grupos.");
                println!("0. Finalizar o programa.");

                let opcao = ler_numero();

                match opcao {
                    1 => menu_usuarios(usuarios, grupos),
                    2 => menu_grupos(grupos, usuarios),

                    3 => println!("pasta 'arquivos' em desenvolvimento."), // MIDORI -- troca esse print pela função 'menu_arquivos(diretorios)'
                    4 => println!("pasta 'diretórios' em desenvolvimento."), // MIDORI -- troca esse print pela função 'menu_diretorios(diretorios)'

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


fn main() {
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();

    // MIDORI --    let mut diretorios: Vec<Diretorio> = Vec::new();

    menu_principal(&mut usuarios, &mut grupos);
}
