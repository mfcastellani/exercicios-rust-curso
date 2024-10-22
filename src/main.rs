fn move_clone_strings(s1: String, s2: String) {
    let moved_string = s1;
    let cloned_string = s2.clone();

    // Exibindo os valores após o movimento e clonagem
    println!("String Movida : {}", moved_string);
    println!("String Clonada: {}", cloned_string);

    // println!("s1 original: {}", s1); // Isso causaria um erro de compilação
    // pois a string foi movida

    // A string original s2 ainda pode ser usada, pois foi clonada
    println!("S2 original   : {}", s2);
}

fn sum_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().map(|&num| num).sum()
}

fn adicionar_prefixo(s: &mut String) {
    let prefix = "Prefixo: ";
    s.insert_str(0, prefix);
}

fn multiply_itens(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    v1.iter()
        .zip(v2.iter()) 
        .map(|(&a, &b)| a * b) 
        .collect() 
}

fn contar_caracteres(s: &String) -> usize {
    s.chars().count()
}

fn my_own_split(s: &String) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn my_own_concat(s1: &String, s2: &String) -> String {
    format!("{} e {}", s1, s2)
}

fn adicionar_elemento(vetor: &mut Vec<i32>, numero: &i32) {
    vetor.push(*numero)
}

fn my_own_double(vetor: &mut Vec<i32>) {
    *vetor = vetor.iter().map(|&x| x * 2).collect();
}

fn main() {
    // ex 1
    println!("===================> 01");
    let string1 = String::from("Primeira String");
    let string2 = String::from("Segunda String");
    move_clone_strings(string1, string2);
    println!();

    // ex 2
    println!("===================> 02");
    let vetor = vec![1, 2, 3, 4, 5];
    let resultado = sum_numbers(&vetor);
    println!("Vetor original: {:?}", vetor);
    println!("Soma dos números: {}", resultado);
    println!();

    // ex 3 = 1
    println!("===================> 03");
    println!();

    // ex 4
    println!("===================> 04");
    let mut minha_string = String::from("Blockchain e Rust");
    adicionar_prefixo(&mut minha_string);
    println!("String após a modificação: {}", minha_string);
    println!();

    // ex 5
    println!("===================> 05");
    let vetor1 = vec![1, 2, 3, 4, 5];
    let vetor2 = vec![10, 20, 30, 40, 50];
    let resultado = multiply_itens(&vetor1, &vetor2);
    println!("Vetor resultante: {:?}", resultado);
    println!();

    // ex 6
    println!("===================> 06");
    let minha_string = String::from("Blockchain e Rust");
    let num_caracteres = contar_caracteres(&minha_string);
    println!("A string '{}' tem {} caracteres.", minha_string, num_caracteres);
    println!();

    // ex 7
    println!("===================> 07");
    let palavras = my_own_split(&minha_string);
    println!("Palavras na string: {:?}", palavras);
    println!();

    // ex 8
    println!("===================> 08");
    let string1 = String::from("Blockchain");
    let string2 = String::from("Rust");
    let resultado = my_own_concat(&string1, &string2);
    println!("Resultado da concatenação: {}", resultado);
    println!();

    // ex 9
    println!("===================> 09");
    let mut meu_vetor = vec![1, 2, 3];
    let numero = 4;
    adicionar_elemento(&mut meu_vetor, &numero);
    println!("Vetor após adicionar o número: {:?}", meu_vetor);
    println!();

    // ex 10
    println!("===================> 10");
    let mut meu_vetor = vec![1, 50, 34, 32, 21];
    println!("Antes : {:?}", meu_vetor);
    my_own_double(&mut meu_vetor);
    println!("Depois: {:?}", meu_vetor);
    println!();
}
