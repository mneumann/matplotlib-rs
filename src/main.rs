extern crate matplotlib;

fn main() {
    let x = [123.0, 100.0];
    let y = [100.0, 200.0];

    matplotlib::scatterplot(&x, &y);
}
