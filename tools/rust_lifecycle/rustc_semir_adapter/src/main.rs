#![cfg_attr(feature = "rustc-private", feature(rustc_private))]

#[cfg(feature = "rustc-private")]
extern crate rustc_driver;
#[cfg(feature = "rustc-private")]
extern crate rustc_hir;
#[cfg(feature = "rustc-private")]
extern crate rustc_interface;
#[cfg(feature = "rustc-private")]
extern crate rustc_middle;
#[cfg(feature = "rustc-private")]
extern crate rustc_span;

mod preflight;

#[cfg(feature = "rustc-private")]
mod hir_inventory;

#[cfg(feature = "rustc-private")]
use preflight::print_rustc_private_probe;

#[cfg(not(feature = "rustc-private"))]
fn print_rustc_private_probe() {
    eprintln!("rustc-private probe requires --features rustc-private");
    std::process::exit(2);
}

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("--preflight") if args.next().is_none() => preflight::print_preflight(),
        Some("--toolchain-preflight") if args.next().is_none() => {
            preflight::print_toolchain_preflight()
        }
        Some("--rustc-private-probe") if args.next().is_none() => print_rustc_private_probe(),
        #[cfg(feature = "rustc-private")]
        Some("--hir-item-provenance-inventory") => {
            let inputs: Vec<String> = args.collect();
            hir_inventory::run(&inputs)
        }
        _ => {
            eprintln!(
                "usage: rustc-semir-adapter \
                 (--preflight|--toolchain-preflight|--rustc-private-probe|\
                 --hir-item-provenance-inventory <rust-source>)"
            );
            std::process::exit(2);
        }
    }
}
