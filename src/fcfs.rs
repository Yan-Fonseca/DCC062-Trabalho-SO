use std::io; //crate para input & output

pub fn fcfs(){
    println!("Digite o número total de processos (Máximo de 20): ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Falha na leitura");
    let n : usize = input_line.trim().parse().expect("Entrada não é um inteiro");

    //Vetor para armazenar os tempos de execução dos processos (burst times).
    let mut bt: Vec<i32> = vec![0; n];

    for i in 0..n {
        let mut input_line = String::new();
        println!("P[{}]: ", i+1);
        io::stdin().read_line(&mut input_line).expect("Falha na leitura");
        let num: i32 = input_line.trim().parse().expect("Entrada inválida");
        bt[i] = num;
    }

    //Vetor para armazenar os tempos de espera dos processos.
    let mut wt = vec![0; n];
    wt[0] = 0; //tempo de espera do 1º é 0

    //Cálculo do tempo de espera dos demais
    for i in 1..n{
        wt[i] = 0;
        for j in 0..i{
            wt[i] += bt[j]; //soma dos tempos anteriores...
        }
    }

    //Cálculo do tempo de turnaround
    let mut tat = vec![0; n];

    let mut avwt = 0; //variável para calculo do tempo médio de espera
    let mut avtat = 0; //variável para calculo do tmepo médio de turnaround

    println!("Process\t\tTempo De Execução\tTempo de Espera\t\tTempo de TurnAround");

    for i in 0..n{
        tat[i] = bt[i] + wt[i]; //tempo de execução + tempo de espera para entrar
        avwt += wt[i];
        avtat += tat[i];
        println!("P[{}]\t\t{}\t\t\t{}\t\t\t{}", i+1, bt[i], wt[i], tat[i]);
    }

    let avwt_new: f32 = avwt as f32 / n as f32;
    let avtat_new: f32 = avtat as f32 / n as f32;

    println!("Tempo médio de espera dos processos: {}", avwt_new);
    println!("Tempo médio de TurnAround: {}", avtat_new);
}