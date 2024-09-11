# Rust Scheduling

Projeto desenvolvido para a cadeira de Engenharia de Software no IFCE.

Implementa um sistema crud simples de armazenamento de contatos com grupos,
utilizando a linguagem de programação rust, com uma arquitetura MVC, e
implementando o pattern Observer.

Para executar:

1. clone o repositório
2. instale o compilador Rust
3. certifique-se que está na raíz do projeto
4. execute o código com o comando `cargo run`

### Explicação

As classes dentro da pasta models lidam c
com o armazenamento e manipulação de contatos e listas de contato. A classe Agenda manipula a estrutura de dados principal do programa (uma lista de contatos), enquanto a classe Contato representa uma entrada nessa ED.

A classe em View simplesmente se responsabiliza por mostrar o model na saída padrão, tendo sua função update, da interface Observer, adicionada aos subscribers de Agenda.

O controller lida com o parseamento da entrada do usuário, e tem também a responsabilidade de atualizar o Model conforme necessário.
