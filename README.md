# Gerador de senhas em Rust

O programa pede o comprimento da senha e imprime uma combinação aleatória de letras maiúsculas, letras minúsculas e números.

Com Rust e Cargo instalados, execute na pasta do projeto:

```sh
cargo run --quiet
```

Informe um número inteiro maior que zero e pressione Enter. Se o valor for inválido, o programa pedirá o comprimento novamente.

## Exemplos de uso

Ao informar um comprimento de 8 caracteres, o console pode exibir:

```text
Quantos caracteres deve ter a senha? 8
a7B2kP9m
```

Ao informar zero, o programa exibe a mensagem «Digite um número inteiro maior que zero.» e faz a pergunta novamente.
