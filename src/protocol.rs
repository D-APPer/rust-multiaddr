// ["PROTO_NAME", "PROTO_CODE", "PROTO_SIZE"]
pub const PROTO_TABLE: [(&str, (usize, &str)); 24]  = [
	("dns", (53, "V")),				// 0
	("dns4", (54, "V")),				// 1
	("dns6", (55, "V")),				// 2
	("dccp", (33, "16")),				// 3
	("http", (480, "0")),				// 4
	("https", (443, "0")),				// 5
	("ipfs", (421, "V")),				// 6
	("ip4", (4, "32")),				// 7
	("ip6", (41, "126")),				// 8
	("onion", (444, "96")),				// 9
	("p2p", (420, "V")),				// 10
	("p2p-circui", (290, "0")),			// 11
	("p2p-webrtc-direct", (276, "0")),		// 12
	("p2p-webrtc-star", (275, "0")),		// 13
	("p2p-websocket-star", (479, "0")),		// 14
	("quic", (460, "0")),				// 15
	("sclp", (132, "16")),				// 16
	("tcp", (6, "16")),				// 17
	("udp", (17, "16")),				// 18
	("unix", (400, "V")),				// 19
	("utp", (302, "0")),				// 20
	("udt", (301, "0")),				// 21
	("ws", (477, "0")),				// 22
	("wss", (478, "0"))				// 23
];


pub fn proto_str(proto_name: &str, proto_value: &str) -> Option<Vec<usize>> {
	let proto_code: usize = ((PROTO_TABLE)
					.iter()
					.find(|x| x.0 == proto_name)
					.unwrap()
					.1).0;
	
	match proto_name {

		"ip4" => {
				let mut vec: Vec<usize> = vec![proto_code];
				let mut parsed = (proto_value)
							.split('.')
							.filter_map(|x| x.parse::<usize>().ok())
							.collect::<Vec<usize>>();
				vec.append(&mut parsed);
				return Some(vec);
			},

		"ip6" => {
				let parsed = proto_value.split("::");
				None
			},
		
		"tcp" | "udp" | "dccp" | "sctp" => {
							let parsed: usize = (proto_value)
										.parse::<usize>()
										.unwrap();
							return Some(vec![proto_code, parsed]);
						},

		"ipfs" => {
				None
			},

		"onion" => None,
		
		"utp" | "udt" | "http" | "https" | "ws" | "wss" => None,

		&_ => None,

	}
}
