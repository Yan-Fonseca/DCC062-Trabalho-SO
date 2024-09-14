use rand::{Rng, SeedableRng};   // Importa os traits necessários
use rand::rngs::StdRng;

use std::io; //crate para input & output
use prettytable::{Table, Row, Cell};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::Write;

#[derive(Clone, Copy)]
struct Process{
    process_id: i32,
    limit_time_execution: i32, // miliseconds
    begin_interval: i32,
    end_interval: i32,
    how_many_times_was_draw: i32, //quantas vezes foi sorteado
}

fn check_ticket_inside_interval(min: i32, max: i32, val: i32) -> bool{
    if min <= val && val <= max{
        return true;
    }
    false
}

fn print_scheduling_data(n: usize, total_time: i32, quantum: i32, sorted_index_by_num_tickets: &Vec<Process>, how_many_times_draw: bool, general_table: bool) {
    let mut table = Table::new();

    // Cabeçalho da tabela
    

    // Linhas da tabela
    if !how_many_times_draw{
        table.add_row(Row::new(vec![
            Cell::new("Id do Processo"),
            Cell::new("Começo do Intervalo"),
            Cell::new("Fim do Intervalo"),
            Cell::new("Número de Tickets"),
            Cell::new("Tempo Limite de Execução"),
        ]));

        for process in sorted_index_by_num_tickets {
            let row = Row::new(vec![
                Cell::new(&process.process_id.to_string()),
                Cell::new(&process.begin_interval.to_string()),
                Cell::new(&process.end_interval.to_string()),
                Cell::new(&(process.end_interval - process.begin_interval + 1).to_string()),
                Cell::new(&process.limit_time_execution.to_string()),
            ]);
            table.add_row(row);
        }
    } else {
        table.add_row(Row::new(vec![
            Cell::new("Id do Processo"),
            Cell::new("Começo do Intervalo"),
            Cell::new("Fim do Intervalo"),
            Cell::new("Número de Tickets"),
            Cell::new("Tempo Final Restante"),
            Cell::new("Quantas Vezes Foi Sorteado")
        ]));

        for process in sorted_index_by_num_tickets {
            let row = Row::new(vec![
                Cell::new(&process.process_id.to_string()),
                Cell::new(&process.begin_interval.to_string()),
                Cell::new(&process.end_interval.to_string()),
                Cell::new(&(process.end_interval - process.begin_interval + 1).to_string()),
                Cell::new(&process.limit_time_execution.to_string()),
                Cell::new(&process.how_many_times_was_draw.to_string())
            ]);
            table.add_row(row);
        }
    }
    

    if(general_table){
        let mut table2 = Table::new();
    
        // Linhas com os dados gerais
        table2.add_row(Row::new(vec![
            Cell::new("Número de Processos"),
            Cell::new(&n.to_string()),
        ]));
        table2.add_row(Row::new(vec![
            Cell::new("Tempo total (ms)"),
            Cell::new(&total_time.to_string()),
        ]));
        table2.add_row(Row::new(vec![
            Cell::new("Tamanho Quantum (ms)"),
            Cell::new(&quantum.to_string()),
        ]));
    
        // Imprime a tabela
        table2.printstd();

        println!("");
    }
   

    table.printstd();
}

pub fn lottery_scheduler_fn() -> () {

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Erro ao fazer a subtração do Tempo");
    
    // Usa os segundos do tempo atual como seed
    let seed: u64 = 1726342225;

    println!("Semente de Reprodutibilidade: {seed}");
    
    // Converta o valor da seed para um array de 32 bytes (necessário pelo StdRng)
    let seed_bytes = seed.to_le_bytes();  // Converte o u64 para um array de 8 bytes
    let mut rng = StdRng::from_seed([
        seed_bytes[0], seed_bytes[1], seed_bytes[2], seed_bytes[3],
        seed_bytes[4], seed_bytes[5], seed_bytes[6], seed_bytes[7],
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    ]);

    println!("Digite o número total de processos: ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Falha na leitura");
    let n : usize = input_line.trim().parse().expect("Entrada não é um inteiro");

    println!("Digite o tempo total de execução (s): ");
    input_line.clear();
    io::stdin().read_line(&mut input_line).expect("Falha na leitura");
    let mut total_time : i32 = input_line.trim().parse().expect("Entrada não é um inteiro");
    total_time *= 1000; //Conversão para milisegundos

    println!("Digite o tamanho do quantum (ms): ");
    input_line.clear();
    io::stdin().read_line(&mut input_line).expect("Falha na leitura");
    let quantum : i32 = input_line.trim().parse().expect("Entrada não é um inteiro");

    let mut v_process: Vec<Process> = Vec::new(); //vetor para armazenar os dados dos processos

    let mut total_tickets: i32 = 0; // Total de tickets distribuídos

    let max_tickets: i32 = n as i32 * 20; // Máximo de tickets que podem ser sorteados
    
    for i in 0..n {

        let num = rng.gen_range(5..max_tickets); //Limite máximo de n * 20 tickets.
        let mut begin_interval = 0;
        let mut end_interval = 0;

        if i == 0{
            begin_interval = 0;
            end_interval = num - 1;
        }else {
            begin_interval = v_process[i-1].end_interval + 1;
            end_interval = v_process[i-1].end_interval + num;
        }

        let process = Process{
            process_id: i as i32 + 1,
            begin_interval: begin_interval,
            end_interval: end_interval,
            limit_time_execution: rng.gen_range(800..8000),
            how_many_times_was_draw: 0
        };

        v_process.push(process);

        total_tickets += num;
    }

    // 4 processos: 0..9; 1 | 10..24; 2 | 5; 3 | 1; 4
    // 2 -> 1 -> 3 -> 4
    // 4 processos: 0:14; 2 | 15:24; 1 |...

    let mut sorted_index_by_num_tickets: Vec<Process> = Vec::new();
    
    for _i in 0..n{
        //Ordena os índices pelo número de tickets para tornar mais eficiente a checagem dos sorteios.
        let max_index = v_process.iter().enumerate().max_by_key(|(_idx, &val)| val.end_interval - val.begin_interval + 1).unwrap();
        sorted_index_by_num_tickets.push(*max_index.1);
        v_process.remove(max_index.0);
    }

    println!("");
    print_scheduling_data(n, total_time, quantum, &sorted_index_by_num_tickets, false, true);
    println!("");

    // panic!("Mensagem");

    while total_time > 0 {
        let random_value = rng.gen_range(0..(total_tickets-1)); //Sorteia um valor de ticket qualquer;
        println!("# Valor do Ticket Sorteado: {}", random_value);
        
        let mut cond: bool;
        for j in 0..n{

            cond = check_ticket_inside_interval(sorted_index_by_num_tickets[j].begin_interval, sorted_index_by_num_tickets[j].end_interval, random_value);
            
            if cond && sorted_index_by_num_tickets[j].limit_time_execution > 0{
                
                if sorted_index_by_num_tickets[j].limit_time_execution < quantum{
                    if total_time - quantum >= 0{
                        total_time -= sorted_index_by_num_tickets[j].limit_time_execution;
                        sorted_index_by_num_tickets[j].limit_time_execution = 0;
                    }else{
                        sorted_index_by_num_tickets[j].limit_time_execution -= total_time;
                        total_time = 0;
                    }
                }else {
                    
                    if total_time - quantum >= 0{
                        sorted_index_by_num_tickets[j].limit_time_execution -= quantum;
                        total_time -= quantum;
                    }else{
                        sorted_index_by_num_tickets[j].limit_time_execution -= total_time;
                        total_time = 0;
                    }
                }
                sorted_index_by_num_tickets[j].how_many_times_was_draw += 1;
                break;
            }else if cond && sorted_index_by_num_tickets[j].limit_time_execution <= 0{
                //Sorteou um ticket para quem não tem mais tempo.
                break;
            }
        }

        let mut num_zeros = 0;
        for i in 0..n{
            if(sorted_index_by_num_tickets[i].limit_time_execution == 0){
                num_zeros += 1;
            }
        }

        if num_zeros == n{
            break; //quebrar o while externo quando todos os processos já estiverem com tempo 0.
        }
    }

    sorted_index_by_num_tickets.sort_by_key(|f| f.process_id); //Ordena para pegar em ordem crescente

    println!("");
    print_scheduling_data(n, total_time, quantum, &sorted_index_by_num_tickets, true, false);

    println!("# Tempo Total Restante de Execução: {total_time}");

}