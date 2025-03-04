use std::io;

pub fn sum(){
    println!("Introduce el primer numero: ");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("no");
    // Convierte String a Int
    let num1: i32 = num1.trim().parse().expect("Please enter a valid number");
    println!("Introduce el segundo numero: ");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("no");
    // Convierte String a Int
    let num2: i32 = num2.trim().parse().expect("Please enter a valid number");
    let sum = num1+num2;
    println!("La suma de {num1} + {num2} es: {sum}");
}

pub fn resta(){
    println!("Introduce el primer numero: ");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("no");
    // Convierte String a Int
    let num1: i32 = num1.trim().parse().expect("Please enter a valid number");
    println!("Introduce el segundo numero: ");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("no");
    // Convierte String a Int
    let num2: i32 = num2.trim().parse().expect("Please enter a valid number");
    let rest = num1-num2;
    println!("La resta de {num1} - {num2} es: {rest}");
}

pub fn multi(){
    println!("Introduce el primer numero: ");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("no");
    // Convierte String a Int
    let num1: i32 = num1.trim().parse().expect("Please enter a valid number");
    println!("Introduce el segundo numero: ");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("no");
    // Convierte String a Int
    let num2: i32 = num2.trim().parse().expect("Please enter a valid number");
    let mult = num1*num2;
    println!("La multiplicacion de {num1} * {num2} es: {mult}");
}

pub fn divi(){
    println!("Introduce el primer numero: ");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("no");
    // Convierte String a Int
    let num1: i32 = num1.trim().parse().expect("Please enter a valid number");
    println!("Introduce el segundo numero: ");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("no");
    // Convierte String a Int
    let num2: i32 = num2.trim().parse().expect("Please enter a valid number");
    let div = num1/num2;
    println!("La division de {num1} / {num2} es: {div}");
    println!("Quieres ver el resto?");
    let mut opcion = String::new();
    io::stdin().read_line(&mut opcion).expect("Error al leer la línea");
    let opcion = opcion.trim();
    if opcion == "Si" || opcion == "si" {
        let resto = num1 % num2;
        println!("El resto es {}", resto);
    } else {
        println!("Ok :3");
    }
}

// Bot fn
fn bot(){
    println!("Bot Bob(っ◕‿◕)っ");
    loop{
        println!("1.Sumar");
        println!("2.Restar");
        println!("3.Multiplicar");
        println!("4.Dividir");
        println!("5.Salir");
        println!("Selecciona una opcion: ");
        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .expect("no");
        // Convierte String a Int
        let option: i32 = opcion.trim().parse().expect("Please enter a valid number");
        //Break Loop
        if option == 5{
            println!("Adios :3");
            break;
        }
        if option == 1{
            sum();
        } else if option == 2  {
            resta();
        } else if option == 3 {
            multi();
        } else if option == 4 {
            divi();
        } // Fin if
    } // Fin Loop
} // Fin fn

fn main() {
    bot();
}
