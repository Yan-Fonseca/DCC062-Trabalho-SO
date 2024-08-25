pub mod process;
pub mod lottery_scheduler;

use crate::process::Process;
use std::io;
use crate::lottery_scheduler::sort_process_vector;

/*
Função responsável de gerar os processos que serão consumidos durante o Algoritmo de Escalonamento.

    (Vec<process::Process>) return: Vetor contendo os processos criados.
*/
fn proccess_generate() -> Vec<process::Process> {
    println!("Digite o número total de processos: ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Falha na leitura");
    let n : usize = input_line.trim().parse().expect("Entrada não é um inteiro");

    let mut v_process : Vec<Process> = Vec::new();

    for i in 0..n{
        input_line.clear();
        println!("P[{}] Tempo de Chegada : ", i+1);
        io::stdin().read_line(&mut input_line).expect("Falha na leitura");
        let arr_time: i32 = input_line.trim().parse().expect("Entrada inválida");

        input_line.clear();
        println!("P[{}] Prioridade : ", i+1);
        io::stdin().read_line(&mut input_line).expect("Falha na leitura");
        let priority: i64 = input_line.trim().parse().expect("Entrada inválida");

        let process = Process{
            pid: i as i32,
            arrival_time: arr_time,
            priority
        };

        v_process.push(process);
    }

    v_process
}

fn display_single_process(process: process::Process) -> (){
    println!("Process ID\tArrival Time\tPriority");

    println!("{}\t\t{}\t\t{}", process.pid, process.arrival_time, process.priority);
}

fn display_all_process(v_process: Vec<process::Process>) -> (){
    println!("Process ID\tArrival Time\tPriority");

    for i in 0..v_process.len() {
        let process = v_process[i];
        println!("{}\t\t{}\t\t{}", process.pid, process.arrival_time, process.priority);
    }
}

fn main() {
    let mut v_process = proccess_generate();
    sort_process_vector(&mut v_process);
    display_all_process(v_process);
}
