use std::io;

fn main() {
    println!("*******EURO/USD CONVERTER********");
    convert();
}

fn convert(){
    let mut currency = String::new();

    println!("Enter type of currency (EUR or USD) -->");


    io::stdin()
        .read_line(&mut currency)
        .expect("Please enter EUR or USD");

    let currency = currency.trim();
    
    let mut amount = String::new();

    println!("Enter amount of currency -->");

    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");

    let amount: f32 = amount.trim().parse().expect("please enter an integer");
    let result;

    if currency == "EUR" {
        result = amount*1.0581;
        println!("You have ${} USD", result);
    } else if currency == "USD" {
        result = amount*0.94;
        println!("You have €{} EUR", result);
    } else {
        println!("Please make sure you entered EUR or USD correctly");
        convert()
    }
}