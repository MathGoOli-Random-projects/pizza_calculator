## disclaimer
- o foco do artigo é programação e não pizzaria.
- Poderia ter utilizado qualquer linguagem tipo Python ou JavaScript, até pra deixar mais "fácil", mas como eu tava lendo o [livro de Rust][https://doc.rust-lang.org/book/] esses dias decidi utilizar esta linguagem para fazer o script nela para gerar um executável. Não precisei usar as principais "core features" da linguagem tipo borrowing(tirando no `read_line(&mut entry)"`ou lifetimes.
- Essa receita sempre funcionou pra mim. se for fazer, faça a receita por sua conta e risco :v

Fim de semana pede uma pizza né!? Porem a unica receita que eu tenho é para 500 gramas de farinha(que dá umas 3 pizzas). 500 gramas de farinha é metade do saco tem que ter um jeito da gente fazer com menos. 

- farinha: 500g
 - fermento 10g
 - oleo: 70g
 - açucar: 80g
 - sal: 5g
 - agua: 250g
 
Uma maneira simples de resolver este problema é dividindo tudo por 2. Mas como podemos fazer para uma quantidade em função da quantidade de farinha?  Usando a famigerada regra de 3.

farinha 500g -> F
Açucar 80g -> A
A = F * 80/500

Precisamos repetir isso para todos os ingredientes.


 
```Rust
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

// calcula os valores.
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
```

## exemplos do console

```cmd
###################################################
############### Pizza calculator ##################
###################################################
for favor inserir a quantidade de farinha em gramas:
250
sua receita vai precisar de:
 - farinha: 250g
 - fermento 5g
 - oleo: 35g
 - açucar: 40g
 - sal: 2.5g
 - agua: 125g
Sovar na batedeira por 7 minutos

###################################################
############### Pizza calculator ##################
###################################################
for favor inserir a quantidade de farinha em gramas:
200
sua receita vai precisar de:
 - farinha: 200g
 - fermento 4g
 - oleo: 28.000000000000004g
 - açucar: 32g
 - sal: 2g
 - agua: 100g
Sovar na batedeira por 7 minutos
```

O ``oleo: 28.000000000000004g``ocorre porque os dados não foram truncados.
## sobre a receita na pratica
- Se você quiser fazer a receita vai precisar de uma balança.
- Utilizar fermento biológico.
- Menos que 200g de farinha fica ruim de pesar os ingredientes porque a balança não é tão precisa para pesar 2g de sal(é +/- meia colher de chá de sal).
- Se não tiver uma batedeira vai precisar sovar a massa na mão.
- a massa precisa descansar coberta de azeite numa vasilha fechada de insulfilme com por pelo menos meia hora mas eu aconselho deixar mais 24 horas na geladeira.
- se quiser, pode trocar o oleo por azeite. 

builds:
[windowns][https://github.com/MathGoOli-Random-projects/pizza_calculator/blob/master/builds/pizza_calculator.exe]
