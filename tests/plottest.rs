
#[test]
fn start() {
    let a = [0,1,2,3,4,5,6,7,8,9];
    for x in a.iter().skip(1) {
        println!("{}", x);
    }
}