# matplotlib-rs
Uses rust-cpython to provide a convenient interface to Python's matplotlib from Rust

## Example

```rust
extern crate matplotlib;

use matplotlib::{Env, Plot};

fn main() {
    let env = Env::new();
    let plot = Plot::new(&env);

    // set interactive mode on
    plot.interactive();

    // display window
    plot.show();

    let mut x = Vec::new();
    let mut y = Vec::new();

    for i in 1..100 {
        x.push(i as f32);
        y.push(100.0 - i as f32);

        // clear figure
        plot.clf();

        // set plot title
        plot.title(&format!("Iteration: {}", i));

        // set x/y axis labels
        plot.xlabel("x");
        plot.ylabel("y");

        // show grid
        plot.grid(true);

        // draw the actual data
        plot.scatter(&x, &y);

        // update drawing
        plot.draw();

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
```
