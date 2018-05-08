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

fn parse_address(input: &String) -> Vec<u32> {
    let address: Vec<&str> = input.trim_right_matches('\n').split('.').collect();

    address.iter().map(|octect| octect.parse::<u32>().unwrap()).collect()
}

fn get_net_address(address: &Vec<u32>, mask: &Vec<u32>) -> Vec<u32> {
    address.iter().enumerate().map(|(i, octect)| octect & mask[i]).collect()
}

fn get_wildcard_mask(mask: &Vec<u32>) -> Vec<u32> {
    mask.iter().map(|octect| 255 - octect).collect()
}

fn get_broadcast_address(net_address: &Vec<u32>, mask: &Vec<u32>) -> Vec<u32> {
    mask.iter().enumerate().map(|(i, octect)| {
        if octect < &255 {
            return net_address[i] - octect + 255;
        }

        net_address[i]
    }).collect()
}

fn get_first_ip(net_address: &Vec<u32>) -> Vec<u32> {
    let mut first_ip = net_address.to_vec();
    first_ip[3] += 1;
    first_ip
}

fn get_last_ip(broadcast_address: &Vec<u32>) -> Vec<u32> {
    broadcast_address.iter().enumerate().map(|(i, octect)| {
        if i == 3 {
            return octect - 1;
        }

        *octect
    }).collect()
}

fn format_binary_address(address: &Vec<u32>) -> String {
    let binary_address: Vec<String> = address.iter()
        .map(|octect| format!("{:08b}", octect))
        .collect();

    binary_address.join(".")
}

fn format_address(address: &Vec<u32>) -> String {
    let string_address: Vec<String> = address.iter()
        .map(|octect| format!("{}", octect))
        .collect();

    string_address.join(".")
}

fn main() {
    let input_address = get_user_input("IP address");
    let input_mask = get_user_input("Netmask");

    let address = parse_address(&input_address);
    let mask = parse_address(&input_mask);

    let net_address = get_net_address(&address, &mask);
    let wildcard_mask = get_wildcard_mask(&mask);
    let broadcast_address = get_broadcast_address(&net_address, &mask);
    let first_ip = get_first_ip(&net_address);
    let last_ip = get_last_ip(&broadcast_address);

    println!("The IP entered in binary: {}", format_binary_address(&address));
    println!("The mask entered in binary: {}", format_binary_address(&mask));
    println!("Net address: {}", format_address(&net_address));
    println!("Wildcard address: {}", format_address(&wildcard_mask));
    println!("First available IP address: {}", format_address(&first_ip));
    println!("Last available IP address: {}", format_address(&last_ip));
    println!("Broadcast address: {}", format_address(&broadcast_address));
    println!("Broadcast address: {}", format_address(&last_ip));
}
