# 📝 Diário de Bordo & Desafios Técnicos

Este documento registra os bastidores do desenvolvimento do simulador Unix em Rust, detalhando os desafios arquiteturais e de compilação enfrentados durante o processo.

---

## 🎯 O Desafio da Submissão (Modularização vs. Arquivo Único)

Enquanto a estrutura ideal e limpa do projeto separa as responsabilidades em múltiplos arquivos e pastas (`src/modelos/` e `src/menus/`), a atividade exigia a submissão de todo o código compilável contido em **um único arquivo `main.rs`**.

Para resolver isso sem perder a organização lógica, estruturamos o código utilizando **módulos inline (`pub mod`)**. Isso permitiu manter a separação conceitual de escopos e submenus, garantindo que o compilador enxergasse a hierarquia correta em um único arquivo de entrada.

---

## ⚙️ Principais Erros do Compilador e Resoluções (*Troubleshooting*)

O compilador do Rust (via `rustc`) foi nosso principal aliado para garantir a segurança de tipos e o correto gerenciamento de referências mutáveis. Abaixo estão alguns dos principais erros enfrentados e como os solucionamos:

### 1. Erro de Escopo de Variáveis (`E0425`)
* **O problema:** Durante a integração do menu de arquivos no menu principal (`menu_principal`), o compilador apontava que o valor `arquivos` não era encontrado no escopo.
* **A causa:** Os arquivos no sistema não existiam como um vetor global solto, mas sim **encapsulados dentro de cada diretório** (`Vec<Diretorio>`).
* **A solução:** Ajustamos as assinaturas das funções do menu de arquivos para operarem sobre o vetor de diretórios (`&mut Vec<Diretorio>`), permitindo que a busca e manipulação de arquivos ocorresse dentro do diretório selecionado.

### 2. Incompatibilidade de Tipos nas Referências (`E0308`)
* **O problema:** Funções auxiliares de arquivos esperavam `&mut Vec<Arquivo>`, mas o menu principal repassava `&mut Vec<Diretorio>`.
* **A solução:** Alinhamos a assinatura das funções do menu de arquivos para aceitarem o contexto correto de diretórios, garantindo que o fluxo de referências mutáveis (`&mut`) do Rust respeitasse o empréstimo de dados sem violar as regras de segurança de memória.

---

## 💡 Aprendizados Principais
Trabalhar com Rust em um projeto de sistemas operacionais simulados reforçou a importância de:
* Compreender rigorosamente o **sistema de empréstimos e mutabilidade** (*ownership & borrowing*).
* Planejar a arquitetura de dados considerando a árvore de diretórios e arquivos de forma aninhada.
* Utilizar o compilador do Rust não como um obstáculo, mas como um valioso mentor de código limpo e seguro.
