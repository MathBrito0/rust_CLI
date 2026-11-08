// Struct "Diretorio" que guarda um vetor (Vec<Arquivo>).
// Usa os métodos new, adiciona_arquivo, remove_arquivo, listar_conteudo.
// Usa "arquivo.rs" e "permissao.rs".
// É usado pelo "menu_diretorios.rs", "menu_arquivos.rs" e "main.rs".
use crate::modelos::arquivo::Arquivo;
use crate::modelos::permissao::Permissao;

pub struct Diretorio {
    pub nome:String,
    pub arquivos: Vec<Arquivo>,
    pub permissoes:(Permissao,Permissao, Permissao),
    pub uid_dono: u16 

}

impl Diretorio {
    pub fn new(nome:String, permissoes:(Permissao,Permissao,Permissao), uid_dono:u16) -> Diretorio{
        Diretorio {
            nome,
            arquivos: Vec::new(),
            permissoes,
            uid_dono
        }
    }

    pub fn adiciona_arquivo(&mut self, arquivo: Arquivo){
    self.arquivos.push(arquivo);
    }

    pub fn remove_arquivo(&mut self, nome_arquivo:&str){
        for n in 0..self.arquivos.len(){
            if self.arquivos[n].nome == nome_arquivo{
            self.arquivos.remove(n);
            break;
        }
    }

    }

    pub fn listar_conteudo(&self){
        for n in &self.arquivos{
        println!("{}", n.nome);
    }
    }    
    
    }
