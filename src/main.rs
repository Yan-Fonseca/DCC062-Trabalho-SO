use std::io;

/*
    Round-Robin (preemptivo)
*/

fn main() {
    println!("Digite o número total de processos: ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Falha na leitura");
    let n : usize = input_line.trim().parse().expect("Entrada não é um inteiro");
    
    //Detalhes dos processo
    let mut arr_time: Vec<i32> = vec![0; n];
    let mut burst_time: Vec<i32> = vec![0; n];
    let mut robin_time: Vec<i32> = vec![0; n]; //vetor auxiliar para saber o tempo restante.

    for i in 0..n {
        input_line.clear();
        println!("P[{}] Tempo de Chegada : ", i+1);
        io::stdin().read_line(&mut input_line).expect("Falha na leitura");
        let num: i32 = input_line.trim().parse().expect("Entrada inválida");
        arr_time[i] = num;

        input_line.clear();
        println!("P[{}] Tempo de Execução : ", i+1);
        io::stdin().read_line(&mut input_line).expect("Falha na leitura");
        let num: i32 = input_line.trim().parse().expect("Entrada inválida");
        burst_time[i] = num;

        robin_time[i] = burst_time[i];
    }
    
    println!("Digite o tamanho do quantum: ");
    input_line.clear();
    io::stdin().read_line(&mut input_line).expect("Falha na leitura");
    let quantum: i32 = input_line.trim().parse().expect("Entrada inválida");

    println!("\nProcess\t|Turnaround Time|Waiting Time\n\n");
    let mut remain = n; //número de processos
    let mut t = 0; //controla o tempo passado
    let mut i = 0; //controla os id's dos processos
    let mut flag = -1;

    let mut wt = 0; //tempo médio de espera
    let mut tat = 0; //tempo médio de turnaround

    while remain != 0{
        // caso para o tempo de tiver tempo restante e ser menor que o quantum (Ex: tr = 3 & quantum = 4)
        if robin_time[i] <= quantum && robin_time[i] > 0{
            t += robin_time[i];
            robin_time[i] = 0;
            flag = 1;
        }
        // caso para o tempo restante ser maior que o quantum (Ex: tr = 7 & quantum = 2)
        else if robin_time[i] > 0{
            robin_time[i] -= quantum;
            t += quantum;
        }

        //caso tenha caído no primeiro if...
        if robin_time[i] == 0 && flag == 1{
            remain -= 1;
            println!("P[{}]\t|\t{}\t|\t{}\n",i+1, t-arr_time[i],t-arr_time[i]-burst_time[i]);
            wt += t - arr_time[i] - burst_time[i];
            tat += t - arr_time[i];
            flag = 0;
        }
        if i == n - 1{
            i = 0; //reiniciar o ciclo para voltar para o primeiro
        }else if arr_time[i + 1] <= t{
            i += 1; //passar para o próximo por causa do tempo de chegada
        }else{
            i = 0;
        }
    }

    println!("Tempo médio de espera = {}", wt as f32 / n as f32);
    println!("Tempo médio de turnaround = {}",tat as f32 / n as f32);

}
