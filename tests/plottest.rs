
#[test]
fn start() {
    let a = vec![0,1,2,3,4,5,6,7,8,9];
    
    let mut skiped = false;
    let b: Vec<i32> = a.into_iter().filter(|x| {
        if *x <= 4 {
            return false;
        }   

        if !skiped {
            skiped = true;
            return false;
        }

        return true;
    }).collect();

    println!("{:?}", b);

    // for (i, x) in a.iter().enumerate() {
    //     if *x <= 4 {
    //         continue;
    //     }   

    //     if !skiped {
    //         skiped = true;
    //         continue;
    //     }

    //     a.remove(i);
    // }
}