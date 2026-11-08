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
