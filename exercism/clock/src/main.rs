use clock::Clock;
fn main() {
    println!("-30 % 24 is {}.", -30 % 24);
    println!("-50 % 24 is {}.\n", -50 % 24);
    println!("1500 % 60 is {}.\n", 1500 % 60);
    println!("-28 % 24 is {}.\n", -28 % 24);

    let c1 = Clock::new(-25, 00);
    println!("{}", c1);
    println!("(-25, 00) It must be '23:00'.\n");
    let c1 = Clock::new(-91, 00);
    println!("{}", c1);
    println!("(-91, 00) It must be '05:00'.\n");

    println!(
        "(10, 3) add -30minutes is {}",
        Clock::new(10, 3).add_minutes(-30)
    );
    println!("It must be '09:33'.\n");

    println!("(-25, 00) is {}", Clock::new(-25, 00));
    println!("It must be '23:00'.\n");

    /*
        fn test_negative_hour_roll_over() {
        assert_eq!(Clock::new(-25, 00).to_string(), "23:00");
    }
        */
}
