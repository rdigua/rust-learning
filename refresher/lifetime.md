# Lifetime

## Begin

### start from a simple function


```
fn main() {
	let mut s:String="Hello world!".to_string();
	println!("{}",get_first_word(&mut s));
}
/*
//It is Ok!
fn get_first_word(s:String)->String{
	
	let ss:String=s.chars().take_while(|x| x.is_ascii_alphabetic()).clone().collect();
	ss
}
*/

/*
// This is error.
// &ss cannot use fnnciton out.
fn fn get_first_word(s:String)->&str{
	let ss:String=s.chars().take_while(|x| x.is_ascii_alphabetic()).clone().collect();
	&ss
}
*/
// so, changed main string to mut and using follow function.
fn get_first_word(s:&mut String)->&str{
	let key=s.find(' ');
	match key{
		Some(i)=> return &s[..i],
		None => return &s[..],
	}	
	&s
}

```

### 


## more

[Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)

[Chinese][Rust生命周期常见误区](https://github.com/whfuyn/rust-blog/blob/master/posts/Rust%E7%94%9F%E5%91%BD%E5%91%A8%E6%9C%9F%E7%9A%84%E5%B8%B8%E8%A7%81%E8%AF%AF%E8%A7%A3.md)
