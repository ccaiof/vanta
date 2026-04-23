# Vanta

Vanta e uma linguagem em desenvolvimento escrita em Rust, organizada como um workspace com etapas separadas para lexing, parsing, analise semantica, lowering e geracao de codigo LLVM.

## Visao geral

O fluxo atual do projeto e:

1. Ler um arquivo `main.vt`.
2. Validar a declaracao de pacote (`pack ...`) no inicio do arquivo.
3. Fazer a analise lexica.
4. Fazer o parsing para a AST.
5. Rodar as checagens semanticas.
6. Fazer o lowering para IR.
7. Gerar LLVM IR.
8. Compilar o LLVM IR com `clang`.
9. Executar o binario gerado.

O exemplo principal esta em `examples/codegen-llvm`.

## Estrutura do workspace

- `crates/vanta-ast`: definicoes da AST.
- `crates/vanta-lexer`: analise lexica.
- `crates/vanta-parser`: parser da linguagem.
- `crates/vanta-diagnostics`: tipos e utilitarios de diagnostico.
- `crates/vanta-sema`: validacoes semanticas.
- `crates/vanta-lowering`: conversao da AST para IR.
- `crates/vanta-ir`: definicoes do IR.
- `crates/vanta-codegen-llvm`: geracao de LLVM IR.
- `crates/vanta-cli`: executavel que compila e roda o exemplo.

## Requisitos

- Rust toolchain com suporte ao edition 2024.
- `clang` instalado e disponivel no PATH.
- LLVM 22 compativel com a feature `llvm22-1` usada pelo crate `inkwell`.

## Dependencias

As dependencias Rust do projeto sao gerenciadas pelo Cargo e serao baixadas automaticamente ao executar o workspace. As principais dependencias externas usadas para executar o projeto sao:

- `inkwell` para geracao de LLVM IR.
- `clang` para compilar o LLVM IR gerado em binario executavel.

## Como rodar o exemplo

Entre na pasta do exemplo e execute:

```bash
cd examples/codegen-llvm
cargo run -p vanta-cli
```

O executavel vai ler `main.vt`, gerar `build/main.ll`, compilar o binario em `build/main` e executar o resultado.

## Como compilar

O projeto nao possui um comando separado de compilacao no momento. A compilacao acontece quando voce executa o CLI:

```bash
cd examples/codegen-llvm
cargo run -p vanta-cli
```

Esse comando faz toda a pipeline: lexer, parser, checagens semanticas, lowering, geracao de LLVM IR, chamada do `clang` e execucao do binario gerado.

Se quiser apenas inspecionar a saida gerada, depois de rodar o comando acima voce pode abrir o arquivo `build/main.ll`.

## Exemplo de entrada

O arquivo `examples/codegen-llvm/main.vt` atualmente contem um programa simples com uma classe `App` e um `main` que imprime uma mensagem na tela.

```vt
pack main

class App() {
    pub function main(): Void {
        print("Hello, Vanta!")
    }
}
```

Esse e um exemplo minimo da sintaxe atual da linguagem: declaracao de pacote com `pack`, uma classe com metodo `main` e uma chamada de funcao.

## Saida gerada

Durante a execucao, o projeto cria a pasta `build/` dentro do diretorio do exemplo para armazenar:

- `main.ll`: o LLVM IR gerado.
- `main`: o binario compilado.

## Status

Este projeto ainda esta em evolucao e a estrutura pode mudar conforme novas etapas da linguagem forem implementadas.
