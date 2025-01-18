use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};

fn main() {
    let mut i = BigInt::one();

    loop {
        let result = i.pow(i.to_u32().unwrap_or(u32::MAX));
        print!("{}", result.to_string());
        i += 1;
    }
}
