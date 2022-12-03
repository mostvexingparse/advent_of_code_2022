use std::io::stdin;

fn main() {
    let result: usize = day_03::get_shared_item_sum(&mut stdin().lock());

    println!("{}", result);
}
