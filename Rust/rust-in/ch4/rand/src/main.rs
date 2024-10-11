// 모듈 사용을 위한 메인 프로그램
mod random;

use crate::random::{linear, xorshift};
fn main() {
    let mut seed = 1u32;
    let r1 = linear::rand(&mut seed);
    let r2 = xorshift::rand(&mut seed);
    println!("{}, {}", r1, r2);
}
