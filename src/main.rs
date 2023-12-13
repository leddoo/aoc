mod y22;
mod y23;

fn main() {
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(|info| {
        println!("PANIC! {:?}", info);
        std::process::exit(1)
    }));

    #[cfg(not(debug_assertions))]
    for i in 0..4_000_000_000u64 {
        core::hint::black_box(i);
    }

    y23::d01::main();
    y23::d07::main();
    y23::d08::main();
    y23::d09::main();
    y23::d10::main();
    y23::d11::main();
    //y23::d12::main();
    y23::d13::main();
}

