mod y22;

fn main() {
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(|info| {
        println!("PANIC! {:?}", info);
        std::process::exit(1)
    }));

    y22::d19::main();
}

