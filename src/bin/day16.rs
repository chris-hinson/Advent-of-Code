//Christopher Hinson
//notes:

use std::fmt;
use std::fs;

//every packet can have EITHER children, OR data
#[derive(Debug)]
struct Packet {
    version: i8,
    packet_type: i8,
    children: Option<Vec<Box<Packet>>>,
    data: Option<i128>,
}

impl Packet {
    fn new(version: i8, packet_type: i8) -> Packet {
        Packet {
            version,
            packet_type,
            children: None,
            data: None,
        }
    }

    fn add_children(&mut self, children: Option<Vec<Box<Packet>>>) {
        self.children = children;
    }

    fn add_data(&mut self, data: Option<i128>) {
        self.data = data
    }
}
impl fmt::Display for Packet {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out_string = format!("v:{} t:{}", self.version, self.packet_type);
        if self.children.is_some() {
            for child in self.children.as_ref().unwrap() {
                out_string.push_str(&format!("\n\t{}", child));
            }
        } else {
            out_string.push_str(&format!("->{}\n", self.data.unwrap()));
        }
        write!(f, "{out_string}")
    }
}

fn get_bits(content: &mut Vec<char>, len: u32) -> usize {
    //drain the bits from the vec
    let drain = &content.drain(0..((len) as usize)).collect::<String>();
    //println!("\t\t\t\t\t\t\t\t{drain}");
    //return our actual bitstring
    return usize::from_str_radix(drain, 2).unwrap();
}

//pass in the input string, receive head node of tree
fn parse(input: &mut Vec<char>) -> Packet {
    println!(
        "bitstring now looks like {}",
        input.iter().collect::<String>()
    );

    //we need to keep track of how many bits we're reading so that we can discord padding
    //let mut bits_consumed = 0;
    //we ALWAYS get the version and type
    let version = get_bits(input, 3);
    let packet_type = get_bits(input, 3);
    println!("version:{}\ntype:{}", version, packet_type);
    let mut packet = Packet::new(version as i8, packet_type as i8);

    //depending on the type, we will either have a terminal value, or need to recurse and parse more values
    if packet_type != 4 {
        println!("\toperator node");
        let contents_bit = get_bits(input, 1);

        let mut children: Vec<Box<Packet>> = Vec::new();

        //0 bit means 15 bit number for length of contents
        if contents_bit == 0 {
            println!("\toperator mode 0");
            let len = get_bits(input, 15);
            println!("\t\tnext {} bits are children of this node", len);
            let sub_packets = &mut input.drain(0..len).collect::<Vec<char>>();

            //only recurse if there's meaningful data to read lul
            while sub_packets.len() != 0
                && sub_packets.clone().iter_mut().find(|ele| **ele == '1') != None
            {
                children.push(Box::new(parse(&mut *sub_packets)));
            }
        } else
        //1 bit means next 11 bits are number OF sub packets, so we will recurse this many times
        {
            println!("\toperator mode 1");
            let num_packets = get_bits(input, 11);
            println!("\t\tthis node has {} children", num_packets);
            for _i in 0..num_packets {
                //only recurse if theres meaningful data upon which to do so
                if input.len() != 0 && input.clone().iter_mut().find(|ele| **ele == '1') != None {
                    children.push(Box::new(parse(input)));
                }
            }
        }

        packet.add_children(Some(children));
    //lets parse a literal
    } else {
        println!("\tparsing literal");
        let mut val_string = String::new();
        while get_bits(input, 1) != 0 {
            let subpacket = &input.drain(0..4).collect::<String>();
            println!("\t\thit subpacket:{}", subpacket);
            val_string.push_str(subpacket);
        }
        //at this point we already read the 0 bit from the last packet, so just read the data
        let final_subpacket = &input.drain(0..4).collect::<String>();
        println!("\t\tfinal subpacket:{}", final_subpacket);
        val_string.push_str(final_subpacket);

        //this is our full literal, so add it as data
        let final_val = Some(i128::from_str_radix(&val_string, 2).unwrap());
        println!("\tfinal lit val:{:?}", final_val);
        packet.add_data(final_val);
    }

    return packet;
}

fn main() {
    let filename = "./inputs/input16.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("{contents}");

    //ok lets turn our hex string into a bin string
    let mut bitstring = contents
        .chars()
        .map(|hex_char| match hex_char {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        })
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>();

    println!("{}\n", bitstring.iter().collect::<String>());
    let head = parse(&mut bitstring);

    //print_tree(&head);
    let mut sum: i128 = 0;
    sum_tree(&head, &mut sum);

    println!("{sum}");

    println!("{}", head);
}

fn sum_tree(node: &Packet, sum: &mut i128) {
    *sum += node.version as i128;
    //if we have hit a leaf, add its data to the running total
    if node.children.is_none() {
        return;
    }
    //otherwise recurse on each child in order
    for child in node.children.as_ref().unwrap() {
        sum_tree(&child, sum);
    }
}
