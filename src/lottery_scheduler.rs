use rand::Rng;
use std::io; //crate para input & output

fn check_ticket_inside_interval(min: i32, max: i32, val: i32) -> bool{
    if min <= val && val <= max{
        return true;
    }
    false
}

pub fn lottery_scheduler_fn() -> () {

    println!("Digite o número total de processos: ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Falha na leitura");
    let n : usize = input_line.trim().parse().expect("Entrada não é um inteiro");

    //(min, max)
    let mut v_tickets: Vec<(i32, i32, usize)> = Vec::new(); //vetor para armazenar os tickets de cada processo


    let mut total_tickets: i32 = 0;
    for i in 0..n {
        input_line.clear();
        println!("P[{}]: ", i+1);
        io::stdin().read_line(&mut input_line).expect("Falha na leitura");
        let num: i32 = input_line.trim().parse().expect("Entrada inválida");
        
        if i == 0{
            v_tickets.push((0, num-1, i));
        }else {
            v_tickets.push((v_tickets[i-1].1 + 1, v_tickets[i-1].1 + num, i));
        }

        total_tickets += num;
    }

    // 4 processos: 0..9; 1 | 10..24; 2 | 5; 3 | 1; 4
    // 2 -> 1 -> 3 -> 4
    // 4 processos: 0:14; 2 | 15:24; 1 |...

    let mut sorted_index_by_num_tickets: Vec<(i32, i32, usize)> = Vec::new();
    let mut how_many_time_was_draw: Vec<i32> = vec![0; n];

    for _i in 0..n{
        //Ordena os índices pelo número de tickets para tornar mais eficiente a checagem dos sorteios.
        let max_index = v_tickets.iter().enumerate().max_by_key(|(_idx, &val)| val.1 - val.0 + 1).unwrap();
        sorted_index_by_num_tickets.push(*max_index.1);
        v_tickets.remove(max_index.0);
    }

    println!("Impressão dos Intervalos Ordenados pelo tamanho");
    for i in 0..n{
        println!("ProcessId: {} | Min: {} | Max: {} | NumTickets: {}", sorted_index_by_num_tickets[i].2, sorted_index_by_num_tickets[i].0, sorted_index_by_num_tickets[i].1, sorted_index_by_num_tickets[i].1 - sorted_index_by_num_tickets[i].0 + 1);
    }

    input_line.clear();
    println!("Digite a quantidade de sorteios: ");
    io::stdin().read_line(&mut input_line).expect("Falha na leitura");
    let num_sorteios: i32 = input_line.trim().parse().expect("Entrada inválida");

    let mut i: i32 = 0;
    let mut rng = rand::thread_rng();
    while i < num_sorteios {
        let random_value = rng.gen_range(0..(total_tickets-1));
        println!("Valor Aleatório Sorteado: {}", random_value);
        
        let mut cond: bool;
        for j in 0..n{
            cond = check_ticket_inside_interval(sorted_index_by_num_tickets[j].0, sorted_index_by_num_tickets[j].1, random_value);
            if cond{
                how_many_time_was_draw[sorted_index_by_num_tickets[j].2] += 1;
                break;
            }
        }

        i += 1;
    }

    for i in 0..n{
        println!("ProcessID: {} | Total Vezes Sorteado: {}", i, how_many_time_was_draw[i]);
    }
}