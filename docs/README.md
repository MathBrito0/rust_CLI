# 📝 Diário de Bordo e Decisões Arquiteturais

[⬅️ Voltar para o README Principal](../README.md)

Este documento registra a jornada técnica de desenvolvimento do simulador Unix em Rust, detalhando as decisões de arquitetura, a adaptação exigida para a entrega acadêmica e como solucionamos os principais bloqueios do compilador.

---


## 🎯 O Desafio: Arquitetura Modular vs. Arquivo Único (`main.rs`)

Durante o desenvolvimento, organizamos o código dividindo cada responsabilidade em arquivos isolados dentro de `src/modelos/` e `src/menus/`. Essa abordagem facilita o trabalho em equipe via Git, evita conflitos de *merge* e mantém o código limpo.

Contudo, o ambiente de avaliação da atividade exigia que todo o projeto fosse submetido em **um único arquivo `main.rs`**.


### Como resolvemos sem perder a organização:
Em vez de simplesmente colar todas as funções soltas no arquivo principal, utilizamos **módulos inline (`pub mod`)** dentro do `main.rs`:

```rust
pub mod modelos {
    pub mod usuario { /* ... */ }
    pub mod grupo { /* ... */ }
    // ...
}

pub mod menus {
    pub mod menu_principal { /* ... */ }
    // ...
}
```

Isso preservou exatamente os mesmos caminhos de importação (`crate::modelos::...` e `crate::menus::...`) da versão modularizada, garantindo coesão e legibilidade mesmo em um arquivo extenso.

---



## 🛠️ Troubleshooting: Erros do Compilador e Soluções

O compilador do Rust (`rustc`) possui regras rígidas de escopo e propriedade de memória. Abaixo estão os principais erros reais que enfrentamos durante a integração dos módulos e como os corrigimos:



### 1. Variável Fora de Escopo (`error[E0425]`)
* **Sintoma:** `cannot find value 'arquivos' in this scope` ao tentar adicionar um novo arquivo ou acionar o menu de arquivos pelo menu principal.
* **Causa Raiz:** Havia uma divergência arquitetural entre duas partes do código: uma parte tratava os arquivos como elementos armazenados dentro de `Diretorio` (`Vec<Diretorio>`), enquanto outra esperava um vetor global independente (`Vec<Arquivo>`).
* **Solução:** Padronizamos o fluxo de dados desde a função `main()`, garantindo que os vetores corretos fossem inicializados, importados no escopo raiz (`use crate::modelos::arquivo::Arquivo;`) e repassados explicitamente como argumentos para as funções internas.



### 2. Incompatibilidade de Tipos em Referências Mutáveis (`error[E0308]`)
* **Sintoma:** `expected '&mut Vec<Diretorio>', found '&mut Vec<Arquivo>'`.
* **Causa Raiz:** Ao conectar `menu_principal` com `menu_arquivos` e suas funções auxiliares (`criar_arquivo`, `alterar_permissao`), as assinaturas das funções esperavam tipos diferentes de coleções mutáveis.
* **Solução:** Revisamos todas as assinaturas de funções para alinhar os tipos esperados com os argumentos enviados, respeitando as regras de *borrowing* (`&mut`) do Rust para evitar múltiplas referências mutáveis simultâneas.



### 3. Visibilidade de Funções Utilitárias
* **Sintoma:** Funções auxiliares de entrada de dados (`ler_numero` e `ler_texto`) definidas em `menu_usuarios` não eram acessíveis pelos outros menus.
* **Solução:** Promovemos essas funções com o modificador `pub fn` e as importamos nos demais submódulos (`use crate::menus::menu_usuarios::{ler_numero, ler_texto};`), evitando duplicação de código de leitura do `std::io`.

---



## 💡 Principais Aprendizados

1. **Planejamento de Estado Compartilhado:** Definir logo no início como as estruturas se relacionam (se arquivos vivem dentro de diretórios ou em listas globais) evita refatorações complexas na integração.
2. **Leitura de Diagnósticos do Rust:** As mensagens de erro do `rustc` (como `E0308` e `E0425`) indicam exatamente a linha e o tipo esperado, funcionando como um guia preciso para corrigir falhas de arquitetura.
3. **Documentação de Processo:** Registrar os erros e decisões ajuda tanto na apresentação do trabalho quanto na fixação dos conceitos de *ownership* e módulos em Rust.
