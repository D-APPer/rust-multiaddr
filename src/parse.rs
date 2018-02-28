use protocol;


pub fn split_str(string: &str) -> Vec<Vec<&str>> {
	let split: Vec<&str> = (string)
				.split('/')
				.filter(|x| !x.is_empty())
				.collect::<Vec<&str>>();
	
	let length: usize = split.len();
	let mut index: usize = 0;
	let mut result: Vec<_> = Vec::new();
	
	while length > index {
		let table_slice = (protocol::PROTO_TABLE)
						.iter()
						.find(|x| x.0 == split[index])
						.unwrap();
		if (table_slice.1).1 == "0" {
			result.push(vec![split[index]]);
			index += 1;
			continue;
		}
		
		index += 1;
		result.push(vec![table_slice.0, split[index]]);

		index += 1;
	}

	return result;
}
