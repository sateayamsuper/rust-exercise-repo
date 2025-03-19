let names = vec![
	"Albert".to_owned(),
	"Sara".to_owned(),
];

let mut friends = Friends { names };

struct Friends { // struct -> container that held informations
	names: Vec<String>, // Vec -> vector (vector is a list of things)
}

impl IntoIterator for Friends { // implement IntoIterator for Friends
	type Item = String;
	type IntoIter = 
	std::vec::IntoIter<Self::Item>; // self -> currenct object that are being worked on

	fn into_iter(self) -> Self::IntoIter {
		self.names.into_iter()
	}
}

for f in friends {
	println!("{:?}, f");
}