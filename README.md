# Trabalho Prático - Sistemas Operacionais - DCC062 (2024.1)

## Escalonamento de Processos

### Professor: Eduardo Pagani

### Membros: Hugo Carvalho, Lucas Silva Santana, Ricardo Ervilha Silva e Yan Messias de Azevedo Fonseca.

Dado um simulador de carga de trabalho para o subsistema de gerenciamento de processos de um sistema operacional hipotético, implementamos o algoritmo Lottery Scheduling como um dos algoritmos suportados pelo simulador. O usuário informa número de processos, tempo total de execução e tamanho do quantum para a simulação ocorrer.

Instalação do Rust no ambiente Linux:

```bash
sudo apt-get update

sudo apt install build-essential

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Instalação do Rust

source $HOME/.cargo/env # Adicionar o Rust as variáveis de ambiente

rustc --version # Verificar se instalou corretamente 
```

Configuração e execução do projeto:

```bash
git clone https://github.com/Yan-Fonseca/DCC062-Trabalho-SO.git

cd ./DCC062-Trabalho-SO/

cargo run ./src/main.rs # Compilar e executar o código
```


