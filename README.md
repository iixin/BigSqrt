# BigSqrt

计算任意整数开根号

# Quick Start
```Rust
//创建一个BigInt类型的待开方数
let x="2".to_string().str_to_bigint().unwrap(); // 2
//开平方，指定小数点后位数
println!("{}", x.big_sqrt(100).unwrap());       // sqrt(2) 小数点后1000位
```
