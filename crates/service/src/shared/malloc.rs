#[cfg(feature = "mimalloc")]
pub mod mimalloc {
    pub use mimalloc::*;

    #[global_allocator]
    static GLOBAL: MiMalloc = MiMalloc;
}
