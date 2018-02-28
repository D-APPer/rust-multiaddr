extern crate integer_encoding;

mod codec;
mod protocol;
mod parse;
mod varint;


#[derive(Clone, PartialEq)]
pub struct MultiAddr { 
	addr: String,
	bytes: Vec<usize>,
}



impl MultiAddr {
	// Constructors.
	pub fn new(addr: String) -> MultiAddr {
		MultiAddr {  
				bytes: codec::encode(&addr),
				addr: addr
			}
	}
	
	pub fn new_bytes(bytes: Vec<u8>) {
		
	}

	// MULTI_ADDR1, MULTI_ADDR2  ->  /MULTI_ADDR1/MULTI_ADDR2
	pub fn encapsulate(self, addr: &str) -> MultiAddr {
		let encap: String = format!("{0}/{1}", self.addr.trim_right_matches('/'), 
					addr.trim_left_matches('/'));
		return MultiAddr::new(encap);
	}

	// /MULTI_ADDR/UNWANTED_ADDR/MORE_UNWANTED_ADDR  ->  /MULTI_ADDR
	pub fn decapsulate(self, slice: &str) -> MultiAddr {
		let index: Option<usize> = self.addr.find(slice);
		if index != None {
			let slice: String = (self.addr)
						.split_at(index.unwrap())
						.0.to_owned();
			return MultiAddr::new(slice);
		}
		return self;
	}
	
}

