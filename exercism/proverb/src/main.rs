use proverb::build_proverb;

fn main() {
    let input = vec!["nail"];
    println!("{} \n", build_proverb(&input));
    //println!("{}",build_proverb(&input));
    let expected = vec![
        "For want of a nail the shoe was lost.",
        "And all for the want of a nail.",
    ]
    .join("\n");
    println!("{}", expected);
}
