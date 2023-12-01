mod y22;
mod y23;

fn main() {
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(|info| {
        println!("PANIC! {:?}", info);
        std::process::exit(1)
    }));


    y23::d01::main();
}

