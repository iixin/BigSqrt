use num::{bigint::{BigInt, ToBigInt}, Zero, One, ToPrimitive};
use indicatif::ProgressIterator;

trait Digit {
    fn is_string_digit(&self) -> bool;
    fn str_to_bigint(&self) -> Option<BigInt>;
}

impl Digit for String {
    fn is_string_digit(&self) -> bool {
        if !self[..].is_ascii() {
            return false;
        }
        for item in self.as_bytes().iter() {
            if !item.is_ascii_digit() {
                return false;
            }
        }
        return true;
    }

    fn str_to_bigint(&self) -> Option<BigInt> {
        // if !self.is_string_digit() {
        //     return None;
        // }

        let mut result = BigInt::zero();
        let ten: BigInt = BigInt::one() * 10;
        let len = self.len().to_u32().unwrap();
        for (i, &item) in self.as_bytes().iter().enumerate() {
            let x = (item - b'0').to_i32().unwrap();
            let x = x.to_bigint().unwrap();

            result = result + x * (&ten.pow(len - 1 - i.to_u32().unwrap()));
        }
        Some(result)
    }
}

trait BigSqrt {
    fn big_sqrt(&self,len:u32) -> Option<String>;
}

impl BigSqrt for BigInt {
    fn big_sqrt(&self,len:u32) -> Option<String> {
        if self < &BigInt::zero() {
            return None;
        }
        let mut result = self.sqrt();

        let index_of_point =result.to_string().len();
        let ten: BigInt = BigInt::one() * 10;
        for j in (1..len+1).progress() {
            let mut tirl = 0;

            for i in 0..10 {
                let mm1 = &result*&ten + i; //temp
                let mm=&mm1*&mm1;
                let mx=&mm/&ten.pow(2*j);
                if &mx >= self {
                    tirl = i - 1;

                    break;
                }
                tirl = 9;
            }
            result = &result*&ten +tirl;
        }
        let mut ans = result.to_string();
        ans.insert(index_of_point, '.');
        Some(ans)
    }
    
}

fn main() {
    let x="2".to_string().str_to_bigint().unwrap(); // 2
    println!("{}", x.big_sqrt(10000).unwrap());               // sqrt(2) 小数点后1000位
}