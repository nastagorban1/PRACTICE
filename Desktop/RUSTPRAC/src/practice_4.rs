#[test]
fn test0() {
    const SIZE: u8 = 9;
    let half = SIZE / 2;

    fn refl (a: u8) -> u8 {
        SIZE - 1 - a
    }  //відзеркал

    for y in 0..SIZE {
        for x in 0..SIZE {
            let q1 = x + y < half;   //верхня ліва частина
            let q2 = refl(x) + y < half; //верх права
            let q3 = x + refl(y) < half;  //нижня ліва
            let q4 = refl(x) + refl(y) < half;
            let c = if q1 || q2 || q3 || q4 { ' ' } else { '*' };
            print!("{c}");
        }
        println!();
    }
}

