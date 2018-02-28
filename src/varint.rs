const MSB: usize = 0x80;
const NO_MSB: usize = 0x7F;

fn count_bytes(int: usize) -> usize {
	let mut counter: usize = 0;
	let mut int: usize = int;	
	
	while int > 0 {
		counter += 1;
		int >>= 7;
	}

	return counter;
}


pub fn encode(int: usize, dst: &mut Vec<usize>) -> bool {
	if int <= NO_MSB {
		dst.push(int);
		return true;
	}
	
	let mut int = int.to_owned();
	let mut i: usize = 0;	

	while int > 0 {
		dst[i] = MSB | (int as usize & NO_MSB) as usize;
		i += 1;
		int >>= 7;
	}
	
	dst[i - 1] = NO_MSB & dst[i - 1];

	return true;
}


pub fn decode(bit_vector: Vec<usize>) -> usize {
	let mut result: usize = 0;
	let mut shift: usize = 0;

	for byte in bit_vector.iter() {
		let msb_dropped = byte & NO_MSB;
		result |= (msb_dropped as usize) << shift;
		shift += 7;

		if byte & MSB == 0 || shift > (10 * 7) {
			break;
		}
	}

	return result;		
}
