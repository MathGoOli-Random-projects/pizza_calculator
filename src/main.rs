use std::io::{self};
fn main() {
    println!("###################################################");
    println!("############### Pizza calculator ##################");
    println!("###################################################");
    
    // constantes colocando todos os ingredientes em função da farinha
    const FERMENTO: f64 = 10.0 / 500.0;
    const OLEO: f64 = 70.0 / 500.0;
    const ACUCAR: f64 = 80.0 / 500.0;
    const SAL: f64 = 5.0 / 500.0;
    const AGUA: f64 = 250.0 / 500.0;

    // entrada de dados quantidade de farinha
    let mut entry = String::new();
    println!("for favor inserir a quantidade de farinha em gramas:");
    io::stdin().read_line(&mut entry).expect("failed to read a line");
    
    // converte a string da farinha para float.
    let farinha: f64 = entry.trim().parse().unwrap();
    let fermento: f64 = FERMENTO * farinha;
    let oleo: f64 = OLEO * farinha;
    let acucar: f64 = ACUCAR * farinha;
    let sal: f64 = SAL * farinha;
    let agua: f64 = AGUA * farinha;

    // saida de dados
    println!("sua receita vai precisar de:");
    println!(" - farinha: {farinha}g\n - fermento {fermento}g\n - oleo: {oleo}g\n - açucar: {acucar}g\n - sal: {sal}g\n - agua: {agua}g");
    println!("Sovar na batedeira por 7 minutos");

    // esta linha é somente para segurar o console aberto.
    io::stdin().read_line(&mut entry).expect("failed to read a line");
}
