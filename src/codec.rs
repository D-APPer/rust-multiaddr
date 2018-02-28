use integer_encoding::{VarInt, VarIntWriter};

use parse;
use protocol;
use varint;


pub fn encode(addr: &str) -> Vec<usize> {
	let split: Vec<Vec<&str>> = parse::split_str(addr);	
	let mut vec: Vec<usize> = Vec::new();

	for addr_pair in split {
		let proto: Vec<usize> = protocol::proto_str(addr_pair[0], addr_pair[1]).unwrap();
		proto.iter().map(|x| Varint::encode(x.to_owned(), &mut vec));
	}
		
	return vec;
}
