#[cfg(not(feature = "lite"))]
mod cli;

// Avoid musl's default allocator due to lackluster performance
// https://nickb.dev/blog/default-musl-allocator-considered-harmful-to-performance
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "lite")]
mod fast_cli;

#[cfg(not(feature = "lite"))]
pub fn main() -> Result<(), anyhow::Error> {
    cli::Cli::run()
}

#[cfg(feature = "lite")]
pub fn main() -> Result<(), anyhow::Error> {
    fast_cli::run()
}
