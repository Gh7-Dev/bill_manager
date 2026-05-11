use std::io;
//creates a struct to store the bills the names and the amount
struct Bill {
    name : String,
    amount : f64,
}

//prints a prompt and returns whatever the user typed
fn get_input (prompt: &str) -> String {
    println!("{}", prompt);
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()  

}

//Borrow Bill Mutably to allow for update of names and amount to the list
fn add_bill(bills: &mut Vec<Bill>){
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
    bills.push(Bill {name, amount});
    println!("Bill added successfully.");
}

//Borrow bills immutably and oly rwead the content
fn view_bills(bills: &Vec<Bill>) {
    if bills.is_empty(){
        println!("No bills found");
        return;
    }
    println!("\n== Your Bills ==");
    for bill in bills{
        println!("{}: KES {:.2}", bill.name, bill.amount );
    }
} 

fn main(){
    let mut bills : Vec<Bill> = Vec::new();

    loop{
        println!("\n== Bill Manager ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("Q. Quit");

        let choice = get_input("Enter choice : ");
        match choice.as_str() {
            "1" => add_bill(&mut bills),
            "2" => view_bills(&bills),
            "Q" =>  {
                println!("Goodbye");
                break;
            }
            _=> println!("Invalid option. Pleae try again."),
        }
    }

}

