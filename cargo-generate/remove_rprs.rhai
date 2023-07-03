file::delete("README.md");
file::delete("debug_probes.md");
file::delete("CODE_OF_CONDUCT.md");
file::delete("Cargo.toml");
file::rename("cargo-generate/Cargo.toml", "Cargo.toml");
file::delete("cargo-generate")
