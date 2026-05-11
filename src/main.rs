use std::io;
use std::collections::HashMap;

//creates a struct to store the bills the names and the amount

//prints a prompt and returns whatever the user typed
fn get_input (prompt: &str) -> String {
    println!("{}", prompt);
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()  

}

//Borrow Bill Mutably to allow for update of names and amount to the list
fn add_bill(bills: &mut HashMap<String, f64>){
    let name = get_input("Enter bill name");
    let amount_input = get_input ("Enter amount");

    //conversion of string amount to actual f64 number
    let amount:f64 = match amount_input.parse(){
        Ok(value) => value,
        Err(_) =>{
            println!("Invalid amount! Please Enter a number");
            return;
        }
    };
    //create a bill and push into the list
    bills.insert(name.clone(), amount);
    println!("Bill '{}' added successfully.", name);
}

//Borrow bills immutably and oly rwead the content
fn view_bills(bills: &HashMap<String, f64>) {
    if bills.is_empty(){
        println!("No bills found");
        return;
    }
    println!("\n== Your Bills ==");
    for (name, amount) in bills{
        println!("{}: KES {:.2}", name, amount );
    }
} 

fn remove_bill(bills: &mut HashMap<String, f64>){
    view_bills(bills);

    let name = get_input("Enter bill name to remove");
    match bills.remove(&name){
        Some(_) => println!("Bill '{}' removed succesfully", name),
        None  => println!("Bill '{}' not found", name),
    }
}

fn edit_bill(bills: &mut HashMap<String, f64>){
    view_bills(bills);
    let name  = get_input("Enter bill name to edit:");
    if !bills.contains_key(&name){
        println!("Bill'{}' not found.", name);
        return;
    }
    let amount_input = get_input("Enter new amount (or press 'b' to go back):");
    // if b
    if amount_input.trim() == "b"{
        println!("Going back.");
        return;
    }
    let amount: f64  = match amount_input.parse(){
        Ok(value) => value,
        Err(_) =>{
            println!("Invalid amount. Please enter a number.");
            return;
        }
    };
    bills.insert(name.clone(), amount);
    println!("Bill '{}' updatednto KES {:.2}.", name, amount);
}

fn main(){
    let mut bills : HashMap<String,  f64> =HashMap::new();

    loop{
        println!("\n== Bill Manager ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Edit bills");
        println!("q. Quit");

        let choice = get_input("Enter choice : ");
        match choice.as_str() {
            "1" => add_bill(&mut bills),
            "2" => view_bills(&bills),
            "3" => remove_bill(&mut bills),
            "4" => edit_bill(&mut bills),
            "q" =>  {
                println!("Goodbye");
                break;
            }
            _=> println!("Invalid option. Pleae try again."),
        }
    }

}

