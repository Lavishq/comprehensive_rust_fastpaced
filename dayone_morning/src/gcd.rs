
pub mod gcd {
    pub fn find_gcd(a: u32, b: u32) -> u32 {
        // this was from https://google.github.io/comprehensive-rust/control-flow-basics/functions.html
        // but there is a flaw if a < b then the result will be a but it should call gcd(b, a) in else if and there is more to fix i presume but a side note while learning
        if b > 0 {
            find_gcd(b, a % b) // 39
        } else {
            a
        }
    }

    // dbg eg
    pub fn factorial(n: u32) -> u32 {
        let mut product = 1;
        for i in 1..=n {
            // product *= dbg!(i);
            product *= i;
        }
        product
    }
    
    // todo eg
    pub fn fizzbuzz(n: u32) -> u32 {
        todo!()
    }
    
}
