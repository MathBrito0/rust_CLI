// Tem as opções de criação de arquivo,
// alteração de permissão,
// verificar status,
// e delete

// Usa "Arquivo", "Permissao" e "Diretorio" e é usado pelo "menu_principal.rs"

use crate::modelos::arquivo::{self, Arquivo};
use crate::modelos::diretorio::Diretorio;
use crate::modelos::permissao::Permissao;
use crate::menus::menu_usuarios::{ler_numero, ler_texto};