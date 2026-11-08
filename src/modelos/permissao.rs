// É a struct "Permissao" (r -- w -- e) com new e octal.
// Não usa nenhum outro arquivo.
// É usado pelo "arquivo.rs", pelo "diretorio.rs" e
// pelos menus que criam ou alteram as permissões de usuário.
pub struct Permissao {
    pub leitura: bool,
    pub escrita: bool,
    pub execucao: bool,
}

impl Permissao{
    pub fn new (leitura: bool, escrita: bool, execucao:bool) -> Permissao{
        Permissao {
            leitura,
            escrita,
            execucao
        }
    }

    pub fn octal(&self)-> u8 {
        let mut total = 0;
        if self.leitura {total += 4;}
        if self.escrita{total += 2;}
        if self.execucao{total += 1;}
        total
    }
}
