use rand::{rngs::StdRng, Rng};
use std::io; //crate para input & output

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

pub fn lottery_scheduler_fn() -> () {

    let mut rng = rand::thread_rng();

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

        let num = rng.gen_range(0..max_tickets); //Limite máximo de n * 20 tickets.
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
            limit_time_execution: rng.gen_range(50..800),
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
    
    println!("Dados Pré-Execução do Escalonamento");
    println!("Num Processos: {n}");
    println!("Tempo total (s): {total_time}");
    println!("Tamanho quantum (ms): {quantum}");
    for i in 0..n{
        println!("ProcessId: {} | Min: {} | Max: {} | NumTickets: {} | Limit_Time_Execution: {}", sorted_index_by_num_tickets[i].process_id, sorted_index_by_num_tickets[i].begin_interval, sorted_index_by_num_tickets[i].end_interval, sorted_index_by_num_tickets[i].end_interval - sorted_index_by_num_tickets[i].begin_interval + 1, sorted_index_by_num_tickets[i].limit_time_execution);
    }
    
    while total_time > 0 {
        let random_value = rng.gen_range(0..(total_tickets-1)); //Sorteia um valor de ticket qualquer;
        println!("Ticket Sorteado: {}", random_value);
        
        let mut cond: bool;
        for j in 0..n{

            cond = check_ticket_inside_interval(sorted_index_by_num_tickets[j].begin_interval, sorted_index_by_num_tickets[j].end_interval, random_value);
            
            if cond && sorted_index_by_num_tickets[j].limit_time_execution > 0{
                
                if sorted_index_by_num_tickets[j].limit_time_execution < quantum{
                    println!("Id Do Processo Sortudo: {}", sorted_index_by_num_tickets[j].process_id);
                    total_time -= sorted_index_by_num_tickets[j].limit_time_execution;
                    sorted_index_by_num_tickets[j].limit_time_execution = 0;
                }else {
                    println!("Id Do Processo Sortudo: {}", sorted_index_by_num_tickets[j].process_id);
                    sorted_index_by_num_tickets[j].limit_time_execution -= quantum;
                    total_time -= quantum;
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

    for i in 0..n{
        println!("ProcessId: {} | Min: {} | Max: {} | NumTickets: {} | Limit_Time_Execution: {} | Quantas vezes foi sorteado: {}", sorted_index_by_num_tickets[i].process_id, sorted_index_by_num_tickets[i].begin_interval, sorted_index_by_num_tickets[i].end_interval, sorted_index_by_num_tickets[i].end_interval - sorted_index_by_num_tickets[i].begin_interval + 1, sorted_index_by_num_tickets[i].limit_time_execution, sorted_index_by_num_tickets[i].how_many_times_was_draw);
    }

    
}