// iterators4.rs

// Try it! I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
	/*
	let mut n:u64=num;
    let mut r:u64=n;
    //if n==1 {return 1};
    loop {
        n-=1;
        if n!=0{
			r=r*n;
		  }
        println!("{}",r);
          if n==0{
            return r; // return 1
          }
    } 
	
	r
	*/
	
	let mut x = 1;
    (1..=num).for_each(|i| x *= i);
    x

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
