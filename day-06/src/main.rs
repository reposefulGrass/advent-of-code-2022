
mod elfpacket;

use elfpacket::ElfPacket;

fn main() {
    let input = include_str!("../input/input.txt");

    part_a(input);
    part_b(input);
}

fn part_a(input: &str) {
    let ep = ElfPacket::new(input);
    println!(
        "[Part A] The start-of-packet market of the packet is at index {}.", 
        ep.find_start_of_packet_marker().unwrap()
    );
}

fn part_b(input: &str) {
    let ep = ElfPacket::new(input);
    println!(
        "[Part B] The start-of-message market of the packet is at index {}.", 
        ep.find_start_of_message_marker().unwrap()
    );
}
