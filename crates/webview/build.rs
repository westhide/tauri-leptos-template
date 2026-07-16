use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        client: { any(feature = "csr", feature = "hydrate") },
        server: { feature = "ssr" },
    }
}
