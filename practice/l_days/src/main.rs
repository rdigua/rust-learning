
fn main() {
	let fmt = "%Y%m%d";
	let now=chrono::Local::now().date();
	let dft: chrono::format::DelayedFormat<chrono::format::strftime::StrftimeItems>=now.format(fmt);
	println!("Hello, world! {}",dft.to_string());
	println!("Hello, world! {}",get_str_from_date());

    println!("Hello, world! {}",chrono::Local::now().date());
	println!("Hello, world! {}",chrono::Datelike::month(&chrono::Local::now().date()));
	println!("Hello, world! {}",chrono::Datelike::day(&chrono::Local::now().date()));
		println!("Hello, world! {}",chrono::Datelike::year(&chrono::Local::now().date()));
		println!("Hello, world! {}",chrono::Datelike::weekday(&chrono::Local::now().date()));
		println!("Datelike:{:?}",get_day_str());
		println!("Datelike:{:?}",get_day_week_str());

}

fn get_str_from_date()->String{
		let fmt = "%Y%m%d";
	let now=chrono::Local::now().date();
	let dft: chrono::format::DelayedFormat<chrono::format::strftime::StrftimeItems>=now.format(fmt);
	dft.to_string()
}

fn get_day_week_str()->Option<String>{
	//https://github.com/rust-lang/rust-wiki-backup
	let mut s=String::new();
	let n=chrono::Local::now().date();
	let d=(chrono::Datelike::year(&n).to_string(),chrono::Datelike::month(&n).to_string(),chrono::Datelike::day(&n).to_string(),chrono::Datelike::weekday(&n).to_string());
	s.push_str(&d.0);
	s.push_str(&d.1);
	s.push_str(&d.2);
	s.push_str(&d.3);
	Some(s)
	
}

fn get_day_str()->Option<String>{
	//https://github.com/rust-lang/rust-wiki-backup
	let mut s=String::new();
	let n=chrono::Local::now().date();
	let d=(chrono::Datelike::year(&n).to_string(),chrono::Datelike::month(&n).to_string(),chrono::Datelike::day(&n).to_string());
	s.push_str(&d.0);
	s.push_str(&d.1);
	s.push_str(&d.2);
	Some(s)
	
}
