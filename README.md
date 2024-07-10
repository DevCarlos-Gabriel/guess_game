# Jogo de Adivinhação de Números 🎯

Este projeto é uma implementação simples de um jogo de adivinhação em Rust, onde o programa escolhe um número aleatório entre 1 e 100, e o usuário deve adivinhar qual é esse número. O programa fornece dicas se o palpite do usuário é maior ou menor que o número secreto até que o usuário acerte.

# Funcionalidades ✨

- Geração de Número Aleatório: O programa escolhe um número aleatório entre 1 e 100;
- Interação com o Usuário: O usuário insere um palpite, e o programa informa se o palpite é maior, menor ou igual ao número secreto;
- Mensagens de Feedback: O programa fornece feedback ao usuário para ajudá-lo a adivinhar o número correto.

# Como usar 🚀
  
1. **Clone o repositório**:
    ```sh
    git clone https://github.com/DevCarlos-Gabriel/guess_game.git guess_game
    cd guess_game
    ```
    
2. **Compile o programa**:
    ```sh
    cargo build
    ```
    
3. **Execute o programa**:
    ```sh
    cargo run
    ```

## Exemplos de uso 📋

Ao iniciar o programa, você verá o seguinte prompt:

```sh
Esse programa escolheu um número aleatório, entre 1 a 100, e você tem que acertar qual ele escolheu.
Vamos lá! Digite um número inteiro!
```

## Jogando o Jogo 🎮

1. Digite um número inteiro entre 1 e 100 e pressione Enter;
2. O programa fornecerá uma dica dizendo se o número secreto é maior ou menor que o seu palpite;
3. Continue digitando palpites até adivinhar o número correto.

Exemplo de entrada:

```sh
50
```

Possíveis saídas:

```sh
Mais alto!
```

ou 

```sh
Mais baixo!
```

Quando você acertar o número, o programa exibirá:

```sh
Você ganhou!
```

# Estrutura do Código 📂

O código é estruturado da seguinte forma:

1. Geração do Número Secreto: O programa gera um número aleatório entre 1 e 100 usando a função `rand::thread_rng().gen_range(1..=100)`;
2. Entrada do Usuário: O programa lê o palpite do usuário a partir do `stdin`;
3. Comparação de Palpites: O programa compara o palpite do usuário com o número secreto e fornece dicas;
4. Loop de Jogo: O jogo continua em um loop até que o usuário adivinhe o número correto.

## Detalhes da Comparação de Palpites 🔍

A comparação dos palpites é feita usando a expressão `match guess.cmp(&secret_number)`. Aqui está um detalhamento do que ela faz:

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Mais alto!"),
    Ordering::Greater => println!("Mais baixo!"),
    Ordering::Equal => {
        println!("Você ganhou!");
        break;
    }
}
```

A linha `guess.cmp(&secret_number)` utiliza o método cmp para comparar dois valores:

- `guess` é o valor inserido pelo usuário;
- `secret_number` é o número aleatório gerado pelo programa;
- `cmp` é um método definido pela trait `Ord` em Rust, que compara dois valores e retorna um `Ordering`.

O método `cmp` retorna um dos três valores possíveis da enumeração `Ordering`:

1. `Ordering::Less`: Indica que `guess` é menor que `secret_number`.
2. `Ordering::Greater`: Indica que `guess` é maior que `secret_number`.
3. `Ordering::Equal`: Indica que `guess` é igual a `secret_number`.

Então o prgrama usa a estrutura `match` para lidar com cada possível resultado da comparação:

- Se `Ordering::Less` for retornado, significa que o palpite do usuário é menor que o número secreto, e o programa informa ao usuário que o número correto é mais alto;
- Se `Ordering::Greater` for retornado, significa que o palpite do usuário é maior que o número secreto, e o programa informa ao usuário que o número correto é mais baixo;
- Se `Ordering::Equal` for retornado, significa que o palpite do usuário é igual ao número secreto, e o programa informa ao usuário que ele acertou o número e encerra o jogo.

Essa comparação é crucial para fornecer feedback contínuo e interativo ao usuário, ajudando-o a adivinhar o número correto de maneira eficiente.

# Dependências 📦

Este projeto usa a crate `rand` para gerar números aleatórios. Adicione a seguinte dependência ao seu Cargo.toml:

```toml
[dependencies]
rand = "0.8.5"
```
# Considerações Finais 📝

Contribuições são bem-vindas! Sinta-se à vontade para abrir um PR ou relatar problemas.

# Licença 📄

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](https://github.com/DevCarlos-Gabriel/guess_game/blob/main/LICENSE) para mais detalhes.
