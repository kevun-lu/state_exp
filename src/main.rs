fn main() {
    let x = 6;

    let y = {
        let x = 5;
        x + 1
    };

    println!("The expression y is {y}");
}
