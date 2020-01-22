fn main(){
    let rect = (30, 50);

    println!(
        "the area of the rectanble is {}.", area(rect)
    );
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
