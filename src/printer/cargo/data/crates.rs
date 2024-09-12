#[derive(Copy, Clone, Default)]
pub struct Crate {
    pub name: &'static str,
    pub version: (u16, u16, u16),
}

pub static CRATES: [Crate; 512] = [
    Crate { name: "syn", version: (2, 0, 77) },
    Crate { name: "proc-macro2", version: (1, 0, 86) },
    Crate { name: "bitflags", version: (2, 6, 0) },
    Crate { name: "quote", version: (1, 0, 37) },
    Crate { name: "libc", version: (0, 2, 158) },
    Crate { name: "hashbrown", version: (0, 14, 5) },
    Crate { name: "cfg-if", version: (1, 0, 0) },
    Crate { name: "rand_core", version: (0, 6, 4) },
    Crate { name: "rand", version: (0, 8, 5) },
    Crate { name: "base64", version: (0, 22, 1) },
    Crate { name: "serde", version: (1, 0, 210) },
    Crate { name: "regex-syntax", version: (0, 8, 4) },
    Crate { name: "itoa", version: (1, 0, 11) },
    Crate { name: "serde_derive", version: (1, 0, 210) },
    Crate { name: "autocfg", version: (1, 3, 0) },
    Crate { name: "getrandom", version: (0, 2, 15) },
    Crate { name: "memchr", version: (2, 7, 4) },
    Crate { name: "serde_json", version: (1, 0, 128) },
    Crate { name: "log", version: (0, 4, 22) },
    Crate { name: "indexmap", version: (2, 5, 0) },
    Crate { name: "regex", version: (1, 10, 6) },
    Crate { name: "rand_chacha", version: (0, 3, 1) },
    Crate { name: "aho-corasick", version: (1, 1, 3) },
    Crate { name: "once_cell", version: (1, 19, 0) },
    Crate { name: "parking_lot_core", version: (0, 9, 10) },
    Crate { name: "ryu", version: (1, 0, 18) },
    Crate { name: "cc", version: (1, 1, 18) },
    Crate { name: "lazy_static", version: (1, 5, 0) },
    Crate { name: "smallvec", version: (1, 13, 2) },
    Crate { name: "parking_lot", version: (0, 12, 3) },
    Crate { name: "time", version: (0, 3, 36) },
    Crate { name: "clap", version: (4, 5, 17) },
    Crate { name: "num-traits", version: (0, 2, 19) },
    Crate { name: "itertools", version: (0, 13, 0) },
    Crate { name: "heck", version: (0, 5, 0) },
    Crate { name: "strsim", version: (0, 11, 1) },
    Crate { name: "version_check", version: (0, 9, 5) },
    Crate { name: "semver", version: (1, 0, 23) },
    Crate { name: "unicode-ident", version: (1, 0, 13) },
    Crate { name: "lock_api", version: (0, 4, 12) },
    Crate { name: "digest", version: (0, 10, 7) },
    Crate { name: "scopeguard", version: (1, 2, 0) },
    Crate { name: "either", version: (1, 13, 0) },
    Crate { name: "thiserror", version: (1, 0, 63) },
    Crate { name: "thiserror-impl", version: (1, 0, 63) },
    Crate { name: "idna", version: (1, 0, 2) },
    Crate { name: "bytes", version: (1, 7, 1) },
    Crate { name: "block-buffer", version: (0, 10, 4) },
    Crate { name: "crossbeam-utils", version: (0, 8, 20) },
    Crate { name: "regex-automata", version: (0, 4, 7) },
    Crate { name: "socket2", version: (0, 5, 7) },
    Crate { name: "memoffset", version: (0, 9, 1) },
    Crate { name: "pin-project-lite", version: (0, 2, 14) },
    Crate { name: "num_cpus", version: (1, 16, 0) },
    Crate { name: "percent-encoding", version: (2, 3, 1) },
    Crate { name: "generic-array", version: (1, 1, 0) },
    Crate { name: "mio", version: (1, 0, 2) },
    Crate { name: "ppv-lite86", version: (0, 2, 20) },
    Crate { name: "tokio", version: (1, 40, 0) },
    Crate { name: "anyhow", version: (1, 0, 88) },
    Crate { name: "byteorder", version: (1, 5, 0) },
    Crate { name: "miniz_oxide", version: (0, 8, 0) },
    Crate { name: "url", version: (2, 5, 2) },
    Crate { name: "unicode-xid", version: (0, 2, 5) },
    Crate { name: "slab", version: (0, 4, 9) },
    Crate { name: "toml", version: (0, 8, 19) },
    Crate { name: "futures-core", version: (0, 3, 30) },
    Crate { name: "unicode-normalization", version: (0, 1, 23) },
    Crate { name: "num-integer", version: (0, 1, 46) },
    Crate { name: "chrono", version: (0, 4, 38) },
    Crate { name: "futures-util", version: (0, 3, 30) },
    Crate { name: "futures-task", version: (0, 3, 30) },
    Crate { name: "ahash", version: (0, 8, 11) },
    Crate { name: "unicode-bidi", version: (0, 3, 15) },
    Crate { name: "sha2", version: (0, 10, 8) },
    Crate { name: "fnv", version: (1, 0, 7) },
    Crate { name: "futures-sink", version: (0, 3, 30) },
    Crate { name: "typenum", version: (1, 17, 0) },
    Crate { name: "futures", version: (0, 3, 30) },
    Crate { name: "futures-channel", version: (0, 3, 30) },
    Crate { name: "rustix", version: (0, 38, 37) },
    Crate { name: "fastrand", version: (2, 1, 1) },
    Crate { name: "hyper", version: (1, 4, 1) },
    Crate { name: "http", version: (1, 1, 0) },
    Crate { name: "tokio-util", version: (0, 7, 12) },
    Crate { name: "tracing", version: (0, 1, 40) },
    Crate { name: "tracing-core", version: (0, 1, 32) },
    Crate { name: "tinyvec", version: (1, 8, 0) },
    Crate { name: "pkg-config", version: (0, 3, 30) },
    Crate { name: "spin", version: (0, 9, 8) },
    Crate { name: "futures-io", version: (0, 3, 30) },
    Crate { name: "rustc_version", version: (0, 4, 1) },
    Crate { name: "pin-utils", version: (0, 1, 0) },
    Crate { name: "tempfile", version: (3, 12, 0) },
    Crate { name: "unicode-width", version: (0, 1, 13) },
    Crate { name: "uuid", version: (1, 10, 0) },
    Crate { name: "textwrap", version: (0, 16, 1) },
    Crate { name: "termcolor", version: (1, 4, 1) },
    Crate { name: "nom", version: (7, 1, 3) },
    Crate { name: "linux-raw-sys", version: (0, 6, 5) },
    Crate { name: "nix", version: (0, 29, 0) },
    Crate { name: "form_urlencoded", version: (1, 2, 1) },
    Crate { name: "windows_x86_64_msvc", version: (0, 52, 6) },
    Crate { name: "tinyvec_macros", version: (0, 1, 1) },
    Crate { name: "h2", version: (0, 4, 6) },
    Crate { name: "crossbeam-epoch", version: (0, 9, 18) },
    Crate { name: "env_logger", version: (0, 11, 5) },
    Crate { name: "http-body", version: (1, 0, 1) },
    Crate { name: "crossbeam-channel", version: (0, 5, 13) },
    Crate { name: "futures-macro", version: (0, 3, 30) },
    Crate { name: "adler", version: (1, 0, 2) },
    Crate { name: "windows-sys", version: (0, 59, 0) },
    Crate { name: "atty", version: (0, 2, 14) },
    Crate { name: "thread_local", version: (1, 1, 8) },
    Crate { name: "crc32fast", version: (1, 4, 2) },
    Crate { name: "rustls", version: (0, 23, 13) },
    Crate { name: "cpufeatures", version: (0, 2, 14) },
    Crate { name: "httparse", version: (1, 9, 4) },
    Crate { name: "futures-executor", version: (0, 3, 30) },
    Crate { name: "windows_x86_64_gnu", version: (0, 52, 6) },
    Crate { name: "windows_i686_msvc", version: (0, 52, 6) },
    Crate { name: "windows_aarch64_msvc", version: (0, 52, 6) },
    Crate { name: "windows_i686_gnu", version: (0, 52, 6) },
    Crate { name: "crossbeam-deque", version: (0, 8, 5) },
    Crate { name: "flate2", version: (1, 0, 33) },
    Crate { name: "tokio-macros", version: (2, 4, 0) },
    Crate { name: "async-trait", version: (0, 1, 82) },
    Crate { name: "hex", version: (0, 4, 3) },
    Crate { name: "clap_lex", version: (0, 7, 2) },
    Crate { name: "pin-project", version: (1, 1, 5) },
    Crate { name: "pin-project-internal", version: (1, 1, 5) },
    Crate { name: "redox_syscall", version: (0, 5, 4) },
    Crate { name: "tracing-attributes", version: (0, 1, 27) },
    Crate { name: "mime", version: (0, 3, 17) },
    Crate { name: "winapi", version: (0, 3, 9) },
    Crate { name: "object", version: (0, 36, 4) },
    Crate { name: "want", version: (0, 3, 1) },
    Crate { name: "try-lock", version: (0, 2, 5) },
    Crate { name: "wasi", version: (0, 13, 2) },
    Crate { name: "arrayvec", version: (0, 7, 6) },
    Crate { name: "rustc-demangle", version: (0, 1, 24) },
    Crate { name: "httpdate", version: (1, 0, 3) },
    Crate { name: "untrusted", version: (0, 9, 0) },
    Crate { name: "ring", version: (0, 17, 8) },
    Crate { name: "walkdir", version: (2, 5, 0) },
    Crate { name: "gimli", version: (0, 31, 0) },
    Crate { name: "humantime", version: (2, 1, 0) },
    Crate { name: "backtrace", version: (0, 3, 74) },
    Crate { name: "time-macros", version: (0, 2, 18) },
    Crate { name: "unicode-segmentation", version: (1, 11, 0) },
    Crate { name: "tower-service", version: (0, 3, 3) },
    Crate { name: "signal-hook-registry", version: (1, 4, 2) },
    Crate { name: "subtle", version: (2, 6, 1) },
    Crate { name: "glob", version: (0, 3, 1) },
    Crate { name: "num-bigint", version: (0, 4, 6) },
    Crate { name: "windows-targets", version: (0, 52, 6) },
    Crate { name: "hmac", version: (0, 12, 1) },
    Crate { name: "same-file", version: (1, 0, 6) },
    Crate { name: "hermit-abi", version: (0, 4, 0) },
    Crate { name: "crypto-common", version: (0, 1, 6) },
    Crate { name: "instant", version: (0, 1, 13) },
    Crate { name: "addr2line", version: (0, 24, 1) },
    Crate { name: "rayon", version: (1, 10, 0) },
    Crate { name: "encoding_rs", version: (0, 8, 34) },
    Crate { name: "windows_aarch64_gnullvm", version: (0, 52, 6) },
    Crate { name: "windows_x86_64_gnullvm", version: (0, 52, 6) },
    Crate { name: "serde_urlencoded", version: (0, 7, 1) },
    Crate { name: "rayon-core", version: (1, 12, 1) },
    Crate { name: "paste", version: (1, 0, 15) },
    Crate { name: "reqwest", version: (0, 12, 7) },
    Crate { name: "clap_derive", version: (4, 5, 13) },
    Crate { name: "tokio-rustls", version: (0, 26, 0) },
    Crate { name: "proc-macro-error", version: (1, 0, 4) },
    Crate { name: "openssl-sys", version: (0, 9, 103) },
    Crate { name: "darling_core", version: (0, 20, 10) },
    Crate { name: "darling_macro", version: (0, 20, 10) },
    Crate { name: "darling", version: (0, 20, 10) },
    Crate { name: "proc-macro-error-attr", version: (1, 0, 4) },
    Crate { name: "opaque-debug", version: (0, 3, 1) },
    Crate { name: "iana-time-zone", version: (0, 1, 60) },
    Crate { name: "errno", version: (0, 3, 9) },
    Crate { name: "which", version: (6, 0, 3) },
    Crate { name: "libloading", version: (0, 8, 5) },
    Crate { name: "static_assertions", version: (1, 1, 0) },
    Crate { name: "openssl-probe", version: (0, 1, 5) },
    Crate { name: "ipnet", version: (2, 10, 0) },
    Crate { name: "ansi_term", version: (0, 12, 1) },
    Crate { name: "toml_edit", version: (0, 22, 20) },
    Crate { name: "rustversion", version: (1, 0, 17) },
    Crate { name: "minimal-lexical", version: (0, 2, 1) },
    Crate { name: "tracing-subscriber", version: (0, 3, 18) },
    Crate { name: "strum_macros", version: (0, 26, 4) },
    Crate { name: "webpki-roots", version: (0, 26, 5) },
    Crate { name: "sct", version: (0, 7, 1) },
    Crate { name: "rustls-pemfile", version: (2, 1, 3) },
    Crate { name: "openssl", version: (0, 10, 66) },
    Crate { name: "zeroize", version: (1, 8, 1) },
    Crate { name: "strum", version: (0, 26, 3) },
    Crate { name: "foreign-types", version: (0, 5, 0) },
    Crate { name: "foreign-types-shared", version: (0, 3, 1) },
    Crate { name: "tracing-log", version: (0, 2, 0) },
    Crate { name: "bumpalo", version: (3, 16, 0) },
    Crate { name: "winapi-x86_64-pc-windows-gnu", version: (0, 4, 0) },
    Crate { name: "sharded-slab", version: (0, 1, 7) },
    Crate { name: "winapi-i686-pc-windows-gnu", version: (0, 4, 0) },
    Crate { name: "jobserver", version: (0, 1, 32) },
    Crate { name: "phf_shared", version: (0, 11, 2) },
    Crate { name: "rustc-hash", version: (2, 0, 0) },
    Crate { name: "bstr", version: (1, 10, 0) },
    Crate { name: "proc-macro-crate", version: (3, 2, 0) },
    Crate { name: "matches", version: (0, 1, 10) },
    Crate { name: "wasm-bindgen", version: (0, 2, 93) },
    Crate { name: "wasm-bindgen-backend", version: (0, 2, 93) },
    Crate { name: "wasm-bindgen-macro", version: (0, 2, 93) },
    Crate { name: "wasm-bindgen-macro-support", version: (0, 2, 93) },
    Crate { name: "wasm-bindgen-shared", version: (0, 2, 93) },
    Crate { name: "petgraph", version: (0, 6, 5) },
    Crate { name: "fixedbitset", version: (0, 5, 7) },
    Crate { name: "quick-error", version: (2, 0, 1) },
    Crate { name: "event-listener", version: (5, 3, 1) },
    Crate { name: "sha1", version: (0, 10, 6) },
    Crate { name: "time-core", version: (0, 1, 2) },
    Crate { name: "tokio-stream", version: (0, 1, 16) },
    Crate { name: "prost", version: (0, 13, 2) },
    Crate { name: "prost-derive", version: (0, 13, 2) },
    Crate { name: "siphasher", version: (1, 0, 1) },
    Crate { name: "unicase", version: (2, 7, 0) },
    Crate { name: "linked-hash-map", version: (0, 5, 6) },
    Crate { name: "hyper-rustls", version: (0, 27, 3) },
    Crate { name: "js-sys", version: (0, 3, 70) },
    Crate { name: "num-rational", version: (0, 4, 2) },
    Crate { name: "native-tls", version: (0, 2, 12) },
    Crate { name: "winnow", version: (0, 6, 18) },
    Crate { name: "convert_case", version: (0, 6, 0) },
    Crate { name: "vcpkg", version: (0, 2, 15) },
    Crate { name: "semver-parser", version: (0, 10, 2) },
    Crate { name: "phf", version: (0, 11, 2) },
    Crate { name: "winapi-util", version: (0, 1, 9) },
    Crate { name: "proc-macro-hack", version: (0, 5, 20) },
    Crate { name: "matchers", version: (0, 2, 0) },
    Crate { name: "hyper-tls", version: (0, 6, 0) },
    Crate { name: "vec_map", version: (0, 8, 2) },
    Crate { name: "ident_case", version: (1, 0, 1) },
    Crate { name: "shlex", version: (1, 3, 0) },
    Crate { name: "equivalent", version: (1, 0, 1) },
    Crate { name: "half", version: (2, 4, 1) },
    Crate { name: "io-lifetimes", version: (2, 0, 3) },
    Crate { name: "clap_builder", version: (4, 5, 17) },
    Crate { name: "filetime", version: (0, 2, 25) },
    Crate { name: "sha-1", version: (0, 10, 1) },
    Crate { name: "remove_dir_all", version: (0, 8, 3) },
    Crate { name: "core-foundation-sys", version: (0, 8, 7) },
    Crate { name: "phf_generator", version: (0, 11, 2) },
    Crate { name: "os_str_bytes", version: (7, 0, 0) },
    Crate { name: "anstyle", version: (1, 0, 8) },
    Crate { name: "webpki", version: (0, 22, 4) },
    Crate { name: "is-terminal", version: (0, 4, 13) },
    Crate { name: "utf8parse", version: (0, 2, 2) },
    Crate { name: "prost-types", version: (0, 13, 2) },
    Crate { name: "rustls-webpki", version: (0, 102, 8) },
    Crate { name: "num-complex", version: (0, 4, 6) },
    Crate { name: "zstd-safe", version: (7, 2, 1) },
    Crate { name: "num-iter", version: (0, 1, 45) },
    Crate { name: "web-sys", version: (0, 3, 70) },
    Crate { name: "zstd", version: (0, 13, 2) },
    Crate { name: "crossbeam-queue", version: (0, 3, 11) },
    Crate { name: "cipher", version: (0, 4, 4) },
    Crate { name: "toml_datetime", version: (0, 6, 8) },
    Crate { name: "dirs", version: (5, 0, 1) },
    Crate { name: "bindgen", version: (0, 70, 1) },
    Crate { name: "ordered-float", version: (4, 2, 2) },
    Crate { name: "tower-layer", version: (0, 3, 3) },
    Crate { name: "lazycell", version: (1, 3, 0) },
    Crate { name: "tower", version: (0, 5, 1) },
    Crate { name: "libm", version: (0, 2, 8) },
    Crate { name: "pest", version: (2, 7, 12) },
    Crate { name: "derive_more", version: (1, 0, 0) },
    Crate { name: "synstructure", version: (0, 13, 1) },
    Crate { name: "bincode", version: (1, 3, 3) },
    Crate { name: "dirs-sys", version: (0, 4, 1) },
    Crate { name: "serde_yaml", version: (0, 9, 34) },
    Crate { name: "anstream", version: (0, 6, 15) },
    Crate { name: "nu-ansi-term", version: (0, 50, 1) },
    Crate { name: "stable_deref_trait", version: (1, 2, 0) },
    Crate { name: "rand_hc", version: (0, 3, 2) },
    Crate { name: "anstyle-parse", version: (0, 2, 5) },
    Crate { name: "zstd-sys", version: (2, 0, 13) },
    Crate { name: "tokio-native-tls", version: (0, 3, 1) },
    Crate { name: "rustls-native-certs", version: (0, 8, 0) },
    Crate { name: "dashmap", version: (6, 1, 0) },
    Crate { name: "prost-build", version: (0, 13, 2) },
    Crate { name: "aes", version: (0, 8, 4) },
    Crate { name: "libz-sys", version: (1, 1, 20) },
    Crate { name: "ucd-trie", version: (0, 1, 6) },
    Crate { name: "sync_wrapper", version: (1, 0, 1) },
    Crate { name: "mime_guess", version: (2, 0, 5) },
    Crate { name: "crypto-mac", version: (0, 11, 1) },
    Crate { name: "async-stream", version: (0, 3, 5) },
    Crate { name: "async-stream-impl", version: (0, 3, 5) },
    Crate { name: "md-5", version: (0, 10, 6) },
    Crate { name: "anstyle-query", version: (1, 1, 1) },
    Crate { name: "prettyplease", version: (0, 2, 22) },
    Crate { name: "overload", version: (0, 1, 1) },
    Crate { name: "bit-vec", version: (0, 8, 0) },
    Crate { name: "clang-sys", version: (1, 8, 1) },
    Crate { name: "home", version: (0, 5, 9) },
    Crate { name: "block-padding", version: (0, 3, 3) },
    Crate { name: "tonic", version: (0, 12, 2) },
    Crate { name: "colorchoice", version: (1, 0, 2) },
    Crate { name: "openssl-macros", version: (0, 1, 1) },
    Crate { name: "data-encoding", version: (2, 6, 0) },
    Crate { name: "futures-lite", version: (2, 3, 0) },
    Crate { name: "constant_time_eq", version: (0, 3, 1) },
    Crate { name: "signature", version: (2, 2, 0) },
    Crate { name: "cexpr", version: (0, 6, 0) },
    Crate { name: "core-foundation", version: (0, 10, 0) },
    Crate { name: "memmap2", version: (0, 9, 4) },
    Crate { name: "multimap", version: (0, 10, 0) },
    Crate { name: "num", version: (0, 4, 3) },
    Crate { name: "wasm-bindgen-futures", version: (0, 4, 43) },
    Crate { name: "async-channel", version: (2, 3, 1) },
    Crate { name: "winreg", version: (0, 52, 0) },
    Crate { name: "arc-swap", version: (1, 7, 1) },
    Crate { name: "csv", version: (1, 3, 0) },
    Crate { name: "crunchy", version: (0, 2, 2) },
    Crate { name: "concurrent-queue", version: (2, 5, 0) },
    Crate { name: "yaml-rust", version: (0, 4, 5) },
    Crate { name: "axum", version: (0, 7, 5) },
    Crate { name: "der", version: (0, 7, 9) },
    Crate { name: "csv-core", version: (0, 1, 11) },
    Crate { name: "indoc", version: (2, 0, 5) },
    Crate { name: "deranged", version: (0, 3, 11) },
    Crate { name: "console", version: (0, 15, 8) },
    Crate { name: "axum-core", version: (0, 4, 3) },
    Crate { name: "derivative", version: (2, 2, 0) },
    Crate { name: "async-lock", version: (3, 4, 0) },
    Crate { name: "rand_pcg", version: (0, 3, 1) },
    Crate { name: "bitvec", version: (1, 0, 1) },
    Crate { name: "zerocopy", version: (0, 7, 35) },
    Crate { name: "pest_derive", version: (2, 7, 12) },
    Crate { name: "lru", version: (0, 12, 4) },
    Crate { name: "serde_with", version: (3, 9, 0) },
    Crate { name: "bit-set", version: (0, 8, 0) },
    Crate { name: "bytemuck", version: (1, 18, 0) },
    Crate { name: "pest_meta", version: (2, 7, 12) },
    Crate { name: "pest_generator", version: (2, 7, 12) },
    Crate { name: "phf_codegen", version: (0, 11, 2) },
    Crate { name: "radium", version: (1, 1, 0) },
    Crate { name: "matchit", version: (0, 8, 4) },
    Crate { name: "tracing-futures", version: (0, 2, 5) },
    Crate { name: "serde_with_macros", version: (3, 9, 0) },
    Crate { name: "peeking_take_while", version: (1, 0, 0) },
    Crate { name: "spki", version: (0, 7, 3) },
    Crate { name: "scoped-tls", version: (1, 0, 1) },
    Crate { name: "arrayref", version: (0, 3, 8) },
    Crate { name: "pem", version: (3, 0, 4) },
    Crate { name: "target-lexicon", version: (0, 12, 16) },
    Crate { name: "rand_xorshift", version: (0, 3, 0) },
    Crate { name: "base64ct", version: (1, 6, 0) },
    Crate { name: "security-framework", version: (2, 11, 1) },
    Crate { name: "parking", version: (2, 2, 1) },
    Crate { name: "criterion", version: (0, 5, 1) },
    Crate { name: "pkcs8", version: (0, 10, 2) },
    Crate { name: "schannel", version: (0, 1, 24) },
    Crate { name: "dirs-sys-next", version: (0, 1, 2) },
    Crate { name: "const-oid", version: (0, 9, 6) },
    Crate { name: "security-framework-sys", version: (2, 11, 1) },
    Crate { name: "num_threads", version: (0, 1, 7) },
    Crate { name: "serde_spanned", version: (0, 6, 7) },
    Crate { name: "polling", version: (3, 7, 3) },
    Crate { name: "ctor", version: (0, 2, 8) },
    Crate { name: "signal-hook", version: (0, 3, 17) },
    Crate { name: "tap", version: (1, 0, 1) },
    Crate { name: "funty", version: (2, 0, 0) },
    Crate { name: "cast", version: (0, 3, 0) },
    Crate { name: "globset", version: (0, 4, 15) },
    Crate { name: "wyz", version: (0, 6, 1) },
    Crate { name: "combine", version: (4, 6, 7) },
    Crate { name: "tungstenite", version: (0, 24, 0) },
    Crate { name: "hostname", version: (0, 4, 0) },
    Crate { name: "fallible-iterator", version: (0, 3, 0) },
    Crate { name: "quick-xml", version: (0, 36, 1) },
    Crate { name: "cargo_metadata", version: (0, 18, 1) },
    Crate { name: "async-io", version: (2, 3, 4) },
    Crate { name: "waker-fn", version: (1, 2, 0) },
    Crate { name: "tinytemplate", version: (1, 2, 1) },
    Crate { name: "criterion-plot", version: (0, 5, 0) },
    Crate { name: "pbkdf2", version: (0, 12, 2) },
    Crate { name: "cookie", version: (0, 18, 1) },
    Crate { name: "tonic-build", version: (0, 12, 2) },
    Crate { name: "diff", version: (0, 1, 13) },
    Crate { name: "oorandom", version: (11, 1, 4) },
    Crate { name: "void", version: (1, 0, 2) },
    Crate { name: "utf-8", version: (0, 7, 6) },
    Crate { name: "dirs-next", version: (2, 0, 0) },
    Crate { name: "predicates", version: (3, 1, 2) },
    Crate { name: "allocator-api2", version: (0, 2, 18) },
    Crate { name: "num_enum", version: (0, 7, 3) },
    Crate { name: "num_enum_derive", version: (0, 7, 3) },
    Crate { name: "hyper-timeout", version: (0, 5, 1) },
    Crate { name: "dtoa", version: (1, 0, 9) },
    Crate { name: "urlencoding", version: (2, 1, 3) },
    Crate { name: "serde_bytes", version: (0, 11, 15) },
    Crate { name: "terminal_size", version: (0, 3, 0) },
    Crate { name: "tokio-io-timeout", version: (1, 2, 0) },
    Crate { name: "zeroize_derive", version: (1, 4, 2) },
    Crate { name: "android_system_properties", version: (0, 1, 5) },
    Crate { name: "tar", version: (0, 4, 41) },
    Crate { name: "windows", version: (0, 58, 0) },
    Crate { name: "curve25519-dalek", version: (4, 1, 3) },
    Crate { name: "xattr", version: (1, 3, 1) },
    Crate { name: "dyn-clone", version: (1, 0, 17) },
    Crate { name: "match_cfg", version: (0, 1, 0) },
    Crate { name: "aead", version: (0, 5, 2) },
    Crate { name: "plotters", version: (0, 3, 7) },
    Crate { name: "structopt", version: (0, 3, 26) },
    Crate { name: "lexical-core", version: (0, 8, 5) },
    Crate { name: "pretty_assertions", version: (1, 4, 0) },
    Crate { name: "structopt-derive", version: (0, 4, 18) },
    Crate { name: "unindent", version: (0, 2, 3) },
    Crate { name: "powerfmt", version: (0, 2, 0) },
    Crate { name: "float-cmp", version: (0, 9, 0) },
    Crate { name: "atomic-waker", version: (1, 1, 2) },
    Crate { name: "camino", version: (1, 1, 9) },
    Crate { name: "async-task", version: (4, 7, 1) },
    Crate { name: "tracing-serde", version: (0, 1, 3) },
    Crate { name: "hkdf", version: (0, 12, 4) },
    Crate { name: "crc", version: (3, 2, 1) },
    Crate { name: "tokio-tungstenite", version: (0, 23, 1) },
    Crate { name: "tiny-keccak", version: (2, 0, 2) },
    Crate { name: "predicates-core", version: (1, 0, 8) },
    Crate { name: "xml-rs", version: (0, 8, 22) },
    Crate { name: "predicates-tree", version: (1, 0, 11) },
    Crate { name: "crypto-bigint", version: (0, 5, 5) },
    Crate { name: "cmake", version: (0, 1, 51) },
    Crate { name: "ctr", version: (0, 9, 2) },
    Crate { name: "doc-comment", version: (0, 3, 3) },
    Crate { name: "term", version: (1, 0, 0) },
    Crate { name: "redox_users", version: (0, 4, 6) },
    Crate { name: "maplit", version: (1, 0, 2) },
    Crate { name: "plotters-backend", version: (0, 3, 7) },
    Crate { name: "yansi", version: (1, 0, 1) },
    Crate { name: "plotters-svg", version: (0, 3, 7) },
    Crate { name: "fxhash", version: (0, 2, 1) },
    Crate { name: "universal-hash", version: (0, 5, 1) },
    Crate { name: "net2", version: (0, 2, 39) },
    Crate { name: "colored", version: (2, 1, 0) },
    Crate { name: "tower-http", version: (0, 5, 2) },
    Crate { name: "iovec", version: (0, 1, 4) },
    Crate { name: "iana-time-zone-haiku", version: (0, 1, 2) },
    Crate { name: "opentelemetry", version: (0, 25, 0) },
    Crate { name: "cargo-platform", version: (0, 1, 8) },
    Crate { name: "num-derive", version: (0, 4, 2) },
    Crate { name: "futures-timer", version: (3, 0, 3) },
    Crate { name: "valuable", version: (0, 1, 0) },
    Crate { name: "threadpool", version: (1, 8, 1) },
    Crate { name: "hashlink", version: (0, 9, 1) },
    Crate { name: "brotli-decompressor", version: (4, 0, 1) },
    Crate { name: "serde_repr", version: (0, 1, 19) },
    Crate { name: "sha3", version: (0, 10, 8) },
    Crate { name: "alloc-no-stdlib", version: (2, 0, 4) },
    Crate { name: "alloc-stdlib", version: (0, 2, 2) },
    Crate { name: "polyval", version: (0, 6, 2) },
    Crate { name: "pyo3", version: (0, 22, 2) },
    Crate { name: "termtree", version: (0, 5, 1) },
    Crate { name: "brotli", version: (6, 0, 0) },
    Crate { name: "blocking", version: (1, 6, 1) },
    Crate { name: "crossbeam", version: (0, 8, 4) },
    Crate { name: "zip", version: (2, 2, 0) },
    Crate { name: "inout", version: (0, 1, 3) },
    Crate { name: "twox-hash", version: (1, 6, 3) },
    Crate { name: "Inflector", version: (0, 11, 4) },
    Crate { name: "pyo3-macros", version: (0, 22, 2) },
    Crate { name: "pyo3-macros-backend", version: (0, 22, 2) },
    Crate { name: "miow", version: (0, 6, 0) },
    Crate { name: "pem-rfc7468", version: (0, 7, 0) },
    Crate { name: "protobuf", version: (3, 5, 1) },
    Crate { name: "async-executor", version: (1, 13, 1) },
    Crate { name: "encode_unicode", version: (1, 0, 0) },
    Crate { name: "ignore", version: (0, 4, 23) },
    Crate { name: "enum-as-inner", version: (0, 6, 1) },
    Crate { name: "aes-gcm", version: (0, 10, 3) },
    Crate { name: "md5", version: (0, 7, 0) },
    Crate { name: "ff", version: (0, 13, 0) },
    Crate { name: "bzip2-sys", version: (0, 1, 11) },
    Crate { name: "ghash", version: (0, 5, 1) },
    Crate { name: "byte-tools", version: (0, 3, 1) },
    Crate { name: "group", version: (0, 13, 0) },
    Crate { name: "libsqlite3-sys", version: (0, 30, 1) },
    Crate { name: "headers", version: (0, 4, 0) },
    Crate { name: "pyo3-build-config", version: (0, 22, 2) },
    Crate { name: "png", version: (0, 17, 13) },
    Crate { name: "flume", version: (0, 11, 0) },
    Crate { name: "lru-cache", version: (0, 1, 2) },
    Crate { name: "elliptic-curve", version: (0, 13, 8) },
    Crate { name: "difflib", version: (0, 4, 0) },
    Crate { name: "wait-timeout", version: (0, 2, 0) },
    Crate { name: "indicatif", version: (0, 17, 8) },
    Crate { name: "headers-core", version: (0, 3, 0) },
    Crate { name: "number_prefix", version: (0, 4, 0) },
    Crate { name: "http-range-header", version: (0, 4, 1) },
    Crate { name: "async-compression", version: (0, 4, 12) },
    Crate { name: "ecdsa", version: (0, 16, 9) },
    Crate { name: "adler32", version: (1, 2, 0) },
    Crate { name: "new_debug_unreachable", version: (1, 0, 6) },
    Crate { name: "keccak", version: (0, 1, 5) },
    Crate { name: "unsafe-libyaml", version: (0, 2, 11) },
    Crate { name: "fs_extra", version: (1, 3, 0) },
    Crate { name: "error-chain", version: (0, 12, 4) },
    Crate { name: "string_cache", version: (0, 8, 7) },
    Crate { name: "codespan-reporting", version: (0, 11, 1) },
    Crate { name: "pathdiff", version: (0, 2, 1) },
];
