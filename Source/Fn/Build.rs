use dashmap::DashMap;

pub async fn Fn(Entry:&str) -> Result<DashMap<u64, (String, String)>, Box<dyn std::error::Error>> {
	let Build = DashMap::new();

	Ok(Build)
}

pub mod Group {
	pub fn Fn(output: Vec<(String, String)>) {
		println!("Processed {} files", output.len());
	}
}
