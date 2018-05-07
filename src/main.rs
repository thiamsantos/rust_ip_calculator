use std::io;

fn get_user_input(input_name: &str) -> String {
    let mut input_message = String::from("Please input your ");
    input_message.push_str(input_name);

    let mut failed_message = String::from("Failed to read ");
    failed_message.push_str(input_name);

    let mut input = String::new();

    println!("{}", &input_message);

    io::stdin().read_line(&mut input)
        .expect(&failed_message);

    input
}

fn parse_address(input: &String) -> Vec<i32> {
    let address: Vec<&str> = input.trim_right_matches('\n').split('.').collect();

    address.iter().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn main() {
    let input_address = get_user_input("IP address");
    let address = parse_address(&input_address);

    let input_mask = get_user_input("Netmask");
    let mask = parse_address(&input_mask);

    println!("You IP: {:?}", address);
    println!("You Mask: {:?}", mask);
}
