// Essa é a struct "Arquivo" com new, alterar_permissao e stat;
// Guarga 3 "Permissao" em uma tupla.
// Usa "permissao.rs".
// É usado por "diretorio.rs" e "menu_arquivos.rs";

use crate::modelos::permissao::Permissao;

pub struct Arquivo {
    pub nome: String,
    pub tamanho: u64,
    pub permissoes:(Permissao, Permissao, Permissao),
    pub uid_dono: u16,
    pub gid_grupo: u16,

}

impl Arquivo {
    pub fn new(nome:String, tamanho: u64, uid: u16, gid:u16)-> Arquivo{
        let rw_dono = Permissao::new(true, true, false);
        let rw_grupo = Permissao::new(true, true, false);
        let r_outros = Permissao::new(true, false, false);
        Arquivo {
            nome,
            tamanho,
            permissoes:(rw_dono, rw_grupo, r_outros),
            uid_dono: uid,
            gid_grupo: gid,

        }
    }

    pub fn alterar_permissao(&mut self, permissoes:(Permissao, Permissao, Permissao)){
        self.permissoes = permissoes;
    }

    pub fn stat(&self) {
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        print!("Permissoes:({}{}{} / ",
        self.permissoes.0.octal(),
        self.permissoes.1.octal(),
        self.permissoes.2.octal());

        if self.permissoes.0.leitura { print!("r"); } else { print!("-"); }
        if self.permissoes.0.escrita {print!("w"); } else {print!("-");}
        if self.permissoes.0.execucao{print!("x");} else{print!("-");}

        if self.permissoes.1.leitura { print!("r"); } else { print!("-"); }
        if self.permissoes.1.escrita {print!("w"); } else {print!("-");}
        if self.permissoes.1.execucao{print!("x");} else{print!("-");}

        if self.permissoes.2.leitura { print!("r"); } else { print!("-"); }
        if self.permissoes.2.escrita {print!("w"); } else {print!("-");}
        if self.permissoes.2.execucao{print!("x");} else{print!("-");} 
        println!(")"); 

        println!("Uid:{}", self.uid_dono);
        println!("Gid:{}", self.gid_grupo);
    }

}
