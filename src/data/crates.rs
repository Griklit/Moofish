/// 热度前512的Crate
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CrateVersion(u16, u16, u16);

impl Display for CrateVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}.{}", self.0, self.1, self.2)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Crate {
    pub name: &'static str,
    pub version: CrateVersion,
}

pub static CRATES: [Crate; 512] = [
    Crate { name: "syn", version: CrateVersion(2, 0, 77) },
    Crate { name: "proc-macro2", version: CrateVersion(1, 0, 86) },
    Crate { name: "bitflags", version: CrateVersion(2, 6, 0) },
    Crate { name: "quote", version: CrateVersion(1, 0, 37) },
    Crate { name: "libc", version: CrateVersion(0, 2, 158) },
    Crate { name: "hashbrown", version: CrateVersion(0, 14, 5) },
    Crate { name: "cfg-if", version: CrateVersion(1, 0, 0) },
    Crate { name: "rand_core", version: CrateVersion(0, 6, 4) },
    Crate { name: "rand", version: CrateVersion(0, 8, 5) },
    Crate { name: "base64", version: CrateVersion(0, 22, 1) },
    Crate { name: "serde", version: CrateVersion(1, 0, 210) },
    Crate { name: "regex-syntax", version: CrateVersion(0, 8, 4) },
    Crate { name: "itoa", version: CrateVersion(1, 0, 11) },
    Crate { name: "serde_derive", version: CrateVersion(1, 0, 210) },
    Crate { name: "autocfg", version: CrateVersion(1, 3, 0) },
    Crate { name: "getrandom", version: CrateVersion(0, 2, 15) },
    Crate { name: "memchr", version: CrateVersion(2, 7, 4) },
    Crate { name: "serde_json", version: CrateVersion(1, 0, 128) },
    Crate { name: "log", version: CrateVersion(0, 4, 22) },
    Crate { name: "indexmap", version: CrateVersion(2, 5, 0) },
    Crate { name: "regex", version: CrateVersion(1, 10, 6) },
    Crate { name: "rand_chacha", version: CrateVersion(0, 3, 1) },
    Crate { name: "aho-corasick", version: CrateVersion(1, 1, 3) },
    Crate { name: "once_cell", version: CrateVersion(1, 19, 0) },
    Crate { name: "parking_lot_core", version: CrateVersion(0, 9, 10) },
    Crate { name: "ryu", version: CrateVersion(1, 0, 18) },
    Crate { name: "cc", version: CrateVersion(1, 1, 18) },
    Crate { name: "lazy_static", version: CrateVersion(1, 5, 0) },
    Crate { name: "smallvec", version: CrateVersion(1, 13, 2) },
    Crate { name: "parking_lot", version: CrateVersion(0, 12, 3) },
    Crate { name: "time", version: CrateVersion(0, 3, 36) },
    Crate { name: "clap", version: CrateVersion(4, 5, 17) },
    Crate { name: "num-traits", version: CrateVersion(0, 2, 19) },
    Crate { name: "itertools", version: CrateVersion(0, 13, 0) },
    Crate { name: "heck", version: CrateVersion(0, 5, 0) },
    Crate { name: "strsim", version: CrateVersion(0, 11, 1) },
    Crate { name: "version_check", version: CrateVersion(0, 9, 5) },
    Crate { name: "semver", version: CrateVersion(1, 0, 23) },
    Crate { name: "unicode-ident", version: CrateVersion(1, 0, 13) },
    Crate { name: "lock_api", version: CrateVersion(0, 4, 12) },
    Crate { name: "digest", version: CrateVersion(0, 10, 7) },
    Crate { name: "scopeguard", version: CrateVersion(1, 2, 0) },
    Crate { name: "either", version: CrateVersion(1, 13, 0) },
    Crate { name: "thiserror", version: CrateVersion(1, 0, 63) },
    Crate { name: "thiserror-impl", version: CrateVersion(1, 0, 63) },
    Crate { name: "idna", version: CrateVersion(1, 0, 2) },
    Crate { name: "bytes", version: CrateVersion(1, 7, 1) },
    Crate { name: "block-buffer", version: CrateVersion(0, 10, 4) },
    Crate { name: "crossbeam-utils", version: CrateVersion(0, 8, 20) },
    Crate { name: "regex-automata", version: CrateVersion(0, 4, 7) },
    Crate { name: "socket2", version: CrateVersion(0, 5, 7) },
    Crate { name: "memoffset", version: CrateVersion(0, 9, 1) },
    Crate { name: "pin-project-lite", version: CrateVersion(0, 2, 14) },
    Crate { name: "num_cpus", version: CrateVersion(1, 16, 0) },
    Crate { name: "percent-encoding", version: CrateVersion(2, 3, 1) },
    Crate { name: "generic-array", version: CrateVersion(1, 1, 0) },
    Crate { name: "mio", version: CrateVersion(1, 0, 2) },
    Crate { name: "ppv-lite86", version: CrateVersion(0, 2, 20) },
    Crate { name: "tokio", version: CrateVersion(1, 40, 0) },
    Crate { name: "anyhow", version: CrateVersion(1, 0, 88) },
    Crate { name: "byteorder", version: CrateVersion(1, 5, 0) },
    Crate { name: "miniz_oxide", version: CrateVersion(0, 8, 0) },
    Crate { name: "url", version: CrateVersion(2, 5, 2) },
    Crate { name: "unicode-xid", version: CrateVersion(0, 2, 5) },
    Crate { name: "slab", version: CrateVersion(0, 4, 9) },
    Crate { name: "toml", version: CrateVersion(0, 8, 19) },
    Crate { name: "futures-core", version: CrateVersion(0, 3, 30) },
    Crate { name: "unicode-normalization", version: CrateVersion(0, 1, 23) },
    Crate { name: "num-integer", version: CrateVersion(0, 1, 46) },
    Crate { name: "chrono", version: CrateVersion(0, 4, 38) },
    Crate { name: "futures-util", version: CrateVersion(0, 3, 30) },
    Crate { name: "futures-task", version: CrateVersion(0, 3, 30) },
    Crate { name: "ahash", version: CrateVersion(0, 8, 11) },
    Crate { name: "unicode-bidi", version: CrateVersion(0, 3, 15) },
    Crate { name: "sha2", version: CrateVersion(0, 10, 8) },
    Crate { name: "fnv", version: CrateVersion(1, 0, 7) },
    Crate { name: "futures-sink", version: CrateVersion(0, 3, 30) },
    Crate { name: "typenum", version: CrateVersion(1, 17, 0) },
    Crate { name: "futures", version: CrateVersion(0, 3, 30) },
    Crate { name: "futures-channel", version: CrateVersion(0, 3, 30) },
    Crate { name: "rustix", version: CrateVersion(0, 38, 37) },
    Crate { name: "fastrand", version: CrateVersion(2, 1, 1) },
    Crate { name: "hyper", version: CrateVersion(1, 4, 1) },
    Crate { name: "http", version: CrateVersion(1, 1, 0) },
    Crate { name: "tokio-util", version: CrateVersion(0, 7, 12) },
    Crate { name: "tracing", version: CrateVersion(0, 1, 40) },
    Crate { name: "tracing-core", version: CrateVersion(0, 1, 32) },
    Crate { name: "tinyvec", version: CrateVersion(1, 8, 0) },
    Crate { name: "pkg-config", version: CrateVersion(0, 3, 30) },
    Crate { name: "spin", version: CrateVersion(0, 9, 8) },
    Crate { name: "futures-io", version: CrateVersion(0, 3, 30) },
    Crate { name: "rustc_version", version: CrateVersion(0, 4, 1) },
    Crate { name: "pin-utils", version: CrateVersion(0, 1, 0) },
    Crate { name: "tempfile", version: CrateVersion(3, 12, 0) },
    Crate { name: "unicode-width", version: CrateVersion(0, 1, 13) },
    Crate { name: "uuid", version: CrateVersion(1, 10, 0) },
    Crate { name: "textwrap", version: CrateVersion(0, 16, 1) },
    Crate { name: "termcolor", version: CrateVersion(1, 4, 1) },
    Crate { name: "nom", version: CrateVersion(7, 1, 3) },
    Crate { name: "linux-raw-sys", version: CrateVersion(0, 6, 5) },
    Crate { name: "nix", version: CrateVersion(0, 29, 0) },
    Crate { name: "form_urlencoded", version: CrateVersion(1, 2, 1) },
    Crate { name: "windows_x86_64_msvc", version: CrateVersion(0, 52, 6) },
    Crate { name: "tinyvec_macros", version: CrateVersion(0, 1, 1) },
    Crate { name: "h2", version: CrateVersion(0, 4, 6) },
    Crate { name: "crossbeam-epoch", version: CrateVersion(0, 9, 18) },
    Crate { name: "env_logger", version: CrateVersion(0, 11, 5) },
    Crate { name: "http-body", version: CrateVersion(1, 0, 1) },
    Crate { name: "crossbeam-channel", version: CrateVersion(0, 5, 13) },
    Crate { name: "futures-macro", version: CrateVersion(0, 3, 30) },
    Crate { name: "adler", version: CrateVersion(1, 0, 2) },
    Crate { name: "windows-sys", version: CrateVersion(0, 59, 0) },
    Crate { name: "atty", version: CrateVersion(0, 2, 14) },
    Crate { name: "thread_local", version: CrateVersion(1, 1, 8) },
    Crate { name: "crc32fast", version: CrateVersion(1, 4, 2) },
    Crate { name: "rustls", version: CrateVersion(0, 23, 13) },
    Crate { name: "cpufeatures", version: CrateVersion(0, 2, 14) },
    Crate { name: "httparse", version: CrateVersion(1, 9, 4) },
    Crate { name: "futures-executor", version: CrateVersion(0, 3, 30) },
    Crate { name: "windows_x86_64_gnu", version: CrateVersion(0, 52, 6) },
    Crate { name: "windows_i686_msvc", version: CrateVersion(0, 52, 6) },
    Crate { name: "windows_aarch64_msvc", version: CrateVersion(0, 52, 6) },
    Crate { name: "windows_i686_gnu", version: CrateVersion(0, 52, 6) },
    Crate { name: "crossbeam-deque", version: CrateVersion(0, 8, 5) },
    Crate { name: "flate2", version: CrateVersion(1, 0, 33) },
    Crate { name: "tokio-macros", version: CrateVersion(2, 4, 0) },
    Crate { name: "async-trait", version: CrateVersion(0, 1, 82) },
    Crate { name: "hex", version: CrateVersion(0, 4, 3) },
    Crate { name: "clap_lex", version: CrateVersion(0, 7, 2) },
    Crate { name: "pin-project", version: CrateVersion(1, 1, 5) },
    Crate { name: "pin-project-internal", version: CrateVersion(1, 1, 5) },
    Crate { name: "redox_syscall", version: CrateVersion(0, 5, 4) },
    Crate { name: "tracing-attributes", version: CrateVersion(0, 1, 27) },
    Crate { name: "mime", version: CrateVersion(0, 3, 17) },
    Crate { name: "winapi", version: CrateVersion(0, 3, 9) },
    Crate { name: "object", version: CrateVersion(0, 36, 4) },
    Crate { name: "want", version: CrateVersion(0, 3, 1) },
    Crate { name: "try-lock", version: CrateVersion(0, 2, 5) },
    Crate { name: "wasi", version: CrateVersion(0, 13, 2) },
    Crate { name: "arrayvec", version: CrateVersion(0, 7, 6) },
    Crate { name: "rustc-demangle", version: CrateVersion(0, 1, 24) },
    Crate { name: "httpdate", version: CrateVersion(1, 0, 3) },
    Crate { name: "untrusted", version: CrateVersion(0, 9, 0) },
    Crate { name: "ring", version: CrateVersion(0, 17, 8) },
    Crate { name: "walkdir", version: CrateVersion(2, 5, 0) },
    Crate { name: "gimli", version: CrateVersion(0, 31, 0) },
    Crate { name: "humantime", version: CrateVersion(2, 1, 0) },
    Crate { name: "backtrace", version: CrateVersion(0, 3, 74) },
    Crate { name: "time-macros", version: CrateVersion(0, 2, 18) },
    Crate { name: "unicode-segmentation", version: CrateVersion(1, 11, 0) },
    Crate { name: "tower-service", version: CrateVersion(0, 3, 3) },
    Crate { name: "signal-hook-registry", version: CrateVersion(1, 4, 2) },
    Crate { name: "subtle", version: CrateVersion(2, 6, 1) },
    Crate { name: "glob", version: CrateVersion(0, 3, 1) },
    Crate { name: "num-bigint", version: CrateVersion(0, 4, 6) },
    Crate { name: "windows-targets", version: CrateVersion(0, 52, 6) },
    Crate { name: "hmac", version: CrateVersion(0, 12, 1) },
    Crate { name: "same-file", version: CrateVersion(1, 0, 6) },
    Crate { name: "hermit-abi", version: CrateVersion(0, 4, 0) },
    Crate { name: "crypto-common", version: CrateVersion(0, 1, 6) },
    Crate { name: "instant", version: CrateVersion(0, 1, 13) },
    Crate { name: "addr2line", version: CrateVersion(0, 24, 1) },
    Crate { name: "rayon", version: CrateVersion(1, 10, 0) },
    Crate { name: "encoding_rs", version: CrateVersion(0, 8, 34) },
    Crate { name: "windows_aarch64_gnullvm", version: CrateVersion(0, 52, 6) },
    Crate { name: "windows_x86_64_gnullvm", version: CrateVersion(0, 52, 6) },
    Crate { name: "serde_urlencoded", version: CrateVersion(0, 7, 1) },
    Crate { name: "rayon-core", version: CrateVersion(1, 12, 1) },
    Crate { name: "paste", version: CrateVersion(1, 0, 15) },
    Crate { name: "reqwest", version: CrateVersion(0, 12, 7) },
    Crate { name: "clap_derive", version: CrateVersion(4, 5, 13) },
    Crate { name: "tokio-rustls", version: CrateVersion(0, 26, 0) },
    Crate { name: "proc-macro-error", version: CrateVersion(1, 0, 4) },
    Crate { name: "openssl-sys", version: CrateVersion(0, 9, 103) },
    Crate { name: "darling_core", version: CrateVersion(0, 20, 10) },
    Crate { name: "darling_macro", version: CrateVersion(0, 20, 10) },
    Crate { name: "darling", version: CrateVersion(0, 20, 10) },
    Crate { name: "proc-macro-error-attr", version: CrateVersion(1, 0, 4) },
    Crate { name: "opaque-debug", version: CrateVersion(0, 3, 1) },
    Crate { name: "iana-time-zone", version: CrateVersion(0, 1, 60) },
    Crate { name: "errno", version: CrateVersion(0, 3, 9) },
    Crate { name: "which", version: CrateVersion(6, 0, 3) },
    Crate { name: "libloading", version: CrateVersion(0, 8, 5) },
    Crate { name: "static_assertions", version: CrateVersion(1, 1, 0) },
    Crate { name: "openssl-probe", version: CrateVersion(0, 1, 5) },
    Crate { name: "ipnet", version: CrateVersion(2, 10, 0) },
    Crate { name: "ansi_term", version: CrateVersion(0, 12, 1) },
    Crate { name: "toml_edit", version: CrateVersion(0, 22, 20) },
    Crate { name: "rustversion", version: CrateVersion(1, 0, 17) },
    Crate { name: "minimal-lexical", version: CrateVersion(0, 2, 1) },
    Crate { name: "tracing-subscriber", version: CrateVersion(0, 3, 18) },
    Crate { name: "strum_macros", version: CrateVersion(0, 26, 4) },
    Crate { name: "webpki-roots", version: CrateVersion(0, 26, 5) },
    Crate { name: "sct", version: CrateVersion(0, 7, 1) },
    Crate { name: "rustls-pemfile", version: CrateVersion(2, 1, 3) },
    Crate { name: "openssl", version: CrateVersion(0, 10, 66) },
    Crate { name: "zeroize", version: CrateVersion(1, 8, 1) },
    Crate { name: "strum", version: CrateVersion(0, 26, 3) },
    Crate { name: "foreign-types", version: CrateVersion(0, 5, 0) },
    Crate { name: "foreign-types-shared", version: CrateVersion(0, 3, 1) },
    Crate { name: "tracing-log", version: CrateVersion(0, 2, 0) },
    Crate { name: "bumpalo", version: CrateVersion(3, 16, 0) },
    Crate { name: "winapi-x86_64-pc-windows-gnu", version: CrateVersion(0, 4, 0) },
    Crate { name: "sharded-slab", version: CrateVersion(0, 1, 7) },
    Crate { name: "winapi-i686-pc-windows-gnu", version: CrateVersion(0, 4, 0) },
    Crate { name: "jobserver", version: CrateVersion(0, 1, 32) },
    Crate { name: "phf_shared", version: CrateVersion(0, 11, 2) },
    Crate { name: "rustc-hash", version: CrateVersion(2, 0, 0) },
    Crate { name: "bstr", version: CrateVersion(1, 10, 0) },
    Crate { name: "proc-macro-crate", version: CrateVersion(3, 2, 0) },
    Crate { name: "matches", version: CrateVersion(0, 1, 10) },
    Crate { name: "wasm-bindgen", version: CrateVersion(0, 2, 93) },
    Crate { name: "wasm-bindgen-backend", version: CrateVersion(0, 2, 93) },
    Crate { name: "wasm-bindgen-macro", version: CrateVersion(0, 2, 93) },
    Crate { name: "wasm-bindgen-macro-support", version: CrateVersion(0, 2, 93) },
    Crate { name: "wasm-bindgen-shared", version: CrateVersion(0, 2, 93) },
    Crate { name: "petgraph", version: CrateVersion(0, 6, 5) },
    Crate { name: "fixedbitset", version: CrateVersion(0, 5, 7) },
    Crate { name: "quick-error", version: CrateVersion(2, 0, 1) },
    Crate { name: "event-listener", version: CrateVersion(5, 3, 1) },
    Crate { name: "sha1", version: CrateVersion(0, 10, 6) },
    Crate { name: "time-core", version: CrateVersion(0, 1, 2) },
    Crate { name: "tokio-stream", version: CrateVersion(0, 1, 16) },
    Crate { name: "prost", version: CrateVersion(0, 13, 2) },
    Crate { name: "prost-derive", version: CrateVersion(0, 13, 2) },
    Crate { name: "siphasher", version: CrateVersion(1, 0, 1) },
    Crate { name: "unicase", version: CrateVersion(2, 7, 0) },
    Crate { name: "linked-hash-map", version: CrateVersion(0, 5, 6) },
    Crate { name: "hyper-rustls", version: CrateVersion(0, 27, 3) },
    Crate { name: "js-sys", version: CrateVersion(0, 3, 70) },
    Crate { name: "num-rational", version: CrateVersion(0, 4, 2) },
    Crate { name: "native-tls", version: CrateVersion(0, 2, 12) },
    Crate { name: "winnow", version: CrateVersion(0, 6, 18) },
    Crate { name: "convert_case", version: CrateVersion(0, 6, 0) },
    Crate { name: "vcpkg", version: CrateVersion(0, 2, 15) },
    Crate { name: "semver-parser", version: CrateVersion(0, 10, 2) },
    Crate { name: "phf", version: CrateVersion(0, 11, 2) },
    Crate { name: "winapi-util", version: CrateVersion(0, 1, 9) },
    Crate { name: "proc-macro-hack", version: CrateVersion(0, 5, 20) },
    Crate { name: "matchers", version: CrateVersion(0, 2, 0) },
    Crate { name: "hyper-tls", version: CrateVersion(0, 6, 0) },
    Crate { name: "vec_map", version: CrateVersion(0, 8, 2) },
    Crate { name: "ident_case", version: CrateVersion(1, 0, 1) },
    Crate { name: "shlex", version: CrateVersion(1, 3, 0) },
    Crate { name: "equivalent", version: CrateVersion(1, 0, 1) },
    Crate { name: "half", version: CrateVersion(2, 4, 1) },
    Crate { name: "io-lifetimes", version: CrateVersion(2, 0, 3) },
    Crate { name: "clap_builder", version: CrateVersion(4, 5, 17) },
    Crate { name: "filetime", version: CrateVersion(0, 2, 25) },
    Crate { name: "sha-1", version: CrateVersion(0, 10, 1) },
    Crate { name: "remove_dir_all", version: CrateVersion(0, 8, 3) },
    Crate { name: "core-foundation-sys", version: CrateVersion(0, 8, 7) },
    Crate { name: "phf_generator", version: CrateVersion(0, 11, 2) },
    Crate { name: "os_str_bytes", version: CrateVersion(7, 0, 0) },
    Crate { name: "anstyle", version: CrateVersion(1, 0, 8) },
    Crate { name: "webpki", version: CrateVersion(0, 22, 4) },
    Crate { name: "is-terminal", version: CrateVersion(0, 4, 13) },
    Crate { name: "utf8parse", version: CrateVersion(0, 2, 2) },
    Crate { name: "prost-types", version: CrateVersion(0, 13, 2) },
    Crate { name: "rustls-webpki", version: CrateVersion(0, 102, 8) },
    Crate { name: "num-complex", version: CrateVersion(0, 4, 6) },
    Crate { name: "zstd-safe", version: CrateVersion(7, 2, 1) },
    Crate { name: "num-iter", version: CrateVersion(0, 1, 45) },
    Crate { name: "web-sys", version: CrateVersion(0, 3, 70) },
    Crate { name: "zstd", version: CrateVersion(0, 13, 2) },
    Crate { name: "crossbeam-queue", version: CrateVersion(0, 3, 11) },
    Crate { name: "cipher", version: CrateVersion(0, 4, 4) },
    Crate { name: "toml_datetime", version: CrateVersion(0, 6, 8) },
    Crate { name: "dirs", version: CrateVersion(5, 0, 1) },
    Crate { name: "bindgen", version: CrateVersion(0, 70, 1) },
    Crate { name: "ordered-float", version: CrateVersion(4, 2, 2) },
    Crate { name: "tower-layer", version: CrateVersion(0, 3, 3) },
    Crate { name: "lazycell", version: CrateVersion(1, 3, 0) },
    Crate { name: "tower", version: CrateVersion(0, 5, 1) },
    Crate { name: "libm", version: CrateVersion(0, 2, 8) },
    Crate { name: "pest", version: CrateVersion(2, 7, 12) },
    Crate { name: "derive_more", version: CrateVersion(1, 0, 0) },
    Crate { name: "synstructure", version: CrateVersion(0, 13, 1) },
    Crate { name: "bincode", version: CrateVersion(1, 3, 3) },
    Crate { name: "dirs-sys", version: CrateVersion(0, 4, 1) },
    Crate { name: "serde_yaml", version: CrateVersion(0, 9, 34) },
    Crate { name: "anstream", version: CrateVersion(0, 6, 15) },
    Crate { name: "nu-ansi-term", version: CrateVersion(0, 50, 1) },
    Crate { name: "stable_deref_trait", version: CrateVersion(1, 2, 0) },
    Crate { name: "rand_hc", version: CrateVersion(0, 3, 2) },
    Crate { name: "anstyle-parse", version: CrateVersion(0, 2, 5) },
    Crate { name: "zstd-sys", version: CrateVersion(2, 0, 13) },
    Crate { name: "tokio-native-tls", version: CrateVersion(0, 3, 1) },
    Crate { name: "rustls-native-certs", version: CrateVersion(0, 8, 0) },
    Crate { name: "dashmap", version: CrateVersion(6, 1, 0) },
    Crate { name: "prost-build", version: CrateVersion(0, 13, 2) },
    Crate { name: "aes", version: CrateVersion(0, 8, 4) },
    Crate { name: "libz-sys", version: CrateVersion(1, 1, 20) },
    Crate { name: "ucd-trie", version: CrateVersion(0, 1, 6) },
    Crate { name: "sync_wrapper", version: CrateVersion(1, 0, 1) },
    Crate { name: "mime_guess", version: CrateVersion(2, 0, 5) },
    Crate { name: "crypto-mac", version: CrateVersion(0, 11, 1) },
    Crate { name: "async-stream", version: CrateVersion(0, 3, 5) },
    Crate { name: "async-stream-impl", version: CrateVersion(0, 3, 5) },
    Crate { name: "md-5", version: CrateVersion(0, 10, 6) },
    Crate { name: "anstyle-query", version: CrateVersion(1, 1, 1) },
    Crate { name: "prettyplease", version: CrateVersion(0, 2, 22) },
    Crate { name: "overload", version: CrateVersion(0, 1, 1) },
    Crate { name: "bit-vec", version: CrateVersion(0, 8, 0) },
    Crate { name: "clang-sys", version: CrateVersion(1, 8, 1) },
    Crate { name: "home", version: CrateVersion(0, 5, 9) },
    Crate { name: "block-padding", version: CrateVersion(0, 3, 3) },
    Crate { name: "tonic", version: CrateVersion(0, 12, 2) },
    Crate { name: "colorchoice", version: CrateVersion(1, 0, 2) },
    Crate { name: "openssl-macros", version: CrateVersion(0, 1, 1) },
    Crate { name: "data-encoding", version: CrateVersion(2, 6, 0) },
    Crate { name: "futures-lite", version: CrateVersion(2, 3, 0) },
    Crate { name: "constant_time_eq", version: CrateVersion(0, 3, 1) },
    Crate { name: "signature", version: CrateVersion(2, 2, 0) },
    Crate { name: "cexpr", version: CrateVersion(0, 6, 0) },
    Crate { name: "core-foundation", version: CrateVersion(0, 10, 0) },
    Crate { name: "memmap2", version: CrateVersion(0, 9, 4) },
    Crate { name: "multimap", version: CrateVersion(0, 10, 0) },
    Crate { name: "num", version: CrateVersion(0, 4, 3) },
    Crate { name: "wasm-bindgen-futures", version: CrateVersion(0, 4, 43) },
    Crate { name: "async-channel", version: CrateVersion(2, 3, 1) },
    Crate { name: "winreg", version: CrateVersion(0, 52, 0) },
    Crate { name: "arc-swap", version: CrateVersion(1, 7, 1) },
    Crate { name: "csv", version: CrateVersion(1, 3, 0) },
    Crate { name: "crunchy", version: CrateVersion(0, 2, 2) },
    Crate { name: "concurrent-queue", version: CrateVersion(2, 5, 0) },
    Crate { name: "yaml-rust", version: CrateVersion(0, 4, 5) },
    Crate { name: "axum", version: CrateVersion(0, 7, 5) },
    Crate { name: "der", version: CrateVersion(0, 7, 9) },
    Crate { name: "csv-core", version: CrateVersion(0, 1, 11) },
    Crate { name: "indoc", version: CrateVersion(2, 0, 5) },
    Crate { name: "deranged", version: CrateVersion(0, 3, 11) },
    Crate { name: "console", version: CrateVersion(0, 15, 8) },
    Crate { name: "axum-core", version: CrateVersion(0, 4, 3) },
    Crate { name: "derivative", version: CrateVersion(2, 2, 0) },
    Crate { name: "async-lock", version: CrateVersion(3, 4, 0) },
    Crate { name: "rand_pcg", version: CrateVersion(0, 3, 1) },
    Crate { name: "bitvec", version: CrateVersion(1, 0, 1) },
    Crate { name: "zerocopy", version: CrateVersion(0, 7, 35) },
    Crate { name: "pest_derive", version: CrateVersion(2, 7, 12) },
    Crate { name: "lru", version: CrateVersion(0, 12, 4) },
    Crate { name: "serde_with", version: CrateVersion(3, 9, 0) },
    Crate { name: "bit-set", version: CrateVersion(0, 8, 0) },
    Crate { name: "bytemuck", version: CrateVersion(1, 18, 0) },
    Crate { name: "pest_meta", version: CrateVersion(2, 7, 12) },
    Crate { name: "pest_generator", version: CrateVersion(2, 7, 12) },
    Crate { name: "phf_codegen", version: CrateVersion(0, 11, 2) },
    Crate { name: "radium", version: CrateVersion(1, 1, 0) },
    Crate { name: "matchit", version: CrateVersion(0, 8, 4) },
    Crate { name: "tracing-futures", version: CrateVersion(0, 2, 5) },
    Crate { name: "serde_with_macros", version: CrateVersion(3, 9, 0) },
    Crate { name: "peeking_take_while", version: CrateVersion(1, 0, 0) },
    Crate { name: "spki", version: CrateVersion(0, 7, 3) },
    Crate { name: "scoped-tls", version: CrateVersion(1, 0, 1) },
    Crate { name: "arrayref", version: CrateVersion(0, 3, 8) },
    Crate { name: "pem", version: CrateVersion(3, 0, 4) },
    Crate { name: "target-lexicon", version: CrateVersion(0, 12, 16) },
    Crate { name: "rand_xorshift", version: CrateVersion(0, 3, 0) },
    Crate { name: "base64ct", version: CrateVersion(1, 6, 0) },
    Crate { name: "security-framework", version: CrateVersion(2, 11, 1) },
    Crate { name: "parking", version: CrateVersion(2, 2, 1) },
    Crate { name: "criterion", version: CrateVersion(0, 5, 1) },
    Crate { name: "pkcs8", version: CrateVersion(0, 10, 2) },
    Crate { name: "schannel", version: CrateVersion(0, 1, 24) },
    Crate { name: "dirs-sys-next", version: CrateVersion(0, 1, 2) },
    Crate { name: "const-oid", version: CrateVersion(0, 9, 6) },
    Crate { name: "security-framework-sys", version: CrateVersion(2, 11, 1) },
    Crate { name: "num_threads", version: CrateVersion(0, 1, 7) },
    Crate { name: "serde_spanned", version: CrateVersion(0, 6, 7) },
    Crate { name: "polling", version: CrateVersion(3, 7, 3) },
    Crate { name: "ctor", version: CrateVersion(0, 2, 8) },
    Crate { name: "signal-hook", version: CrateVersion(0, 3, 17) },
    Crate { name: "tap", version: CrateVersion(1, 0, 1) },
    Crate { name: "funty", version: CrateVersion(2, 0, 0) },
    Crate { name: "cast", version: CrateVersion(0, 3, 0) },
    Crate { name: "globset", version: CrateVersion(0, 4, 15) },
    Crate { name: "wyz", version: CrateVersion(0, 6, 1) },
    Crate { name: "combine", version: CrateVersion(4, 6, 7) },
    Crate { name: "tungstenite", version: CrateVersion(0, 24, 0) },
    Crate { name: "hostname", version: CrateVersion(0, 4, 0) },
    Crate { name: "fallible-iterator", version: CrateVersion(0, 3, 0) },
    Crate { name: "quick-xml", version: CrateVersion(0, 36, 1) },
    Crate { name: "cargo_metadata", version: CrateVersion(0, 18, 1) },
    Crate { name: "async-io", version: CrateVersion(2, 3, 4) },
    Crate { name: "waker-fn", version: CrateVersion(1, 2, 0) },
    Crate { name: "tinytemplate", version: CrateVersion(1, 2, 1) },
    Crate { name: "criterion-plot", version: CrateVersion(0, 5, 0) },
    Crate { name: "pbkdf2", version: CrateVersion(0, 12, 2) },
    Crate { name: "cookie", version: CrateVersion(0, 18, 1) },
    Crate { name: "tonic-build", version: CrateVersion(0, 12, 2) },
    Crate { name: "diff", version: CrateVersion(0, 1, 13) },
    Crate { name: "oorandom", version: CrateVersion(11, 1, 4) },
    Crate { name: "void", version: CrateVersion(1, 0, 2) },
    Crate { name: "utf-8", version: CrateVersion(0, 7, 6) },
    Crate { name: "dirs-next", version: CrateVersion(2, 0, 0) },
    Crate { name: "predicates", version: CrateVersion(3, 1, 2) },
    Crate { name: "allocator-api2", version: CrateVersion(0, 2, 18) },
    Crate { name: "num_enum", version: CrateVersion(0, 7, 3) },
    Crate { name: "num_enum_derive", version: CrateVersion(0, 7, 3) },
    Crate { name: "hyper-timeout", version: CrateVersion(0, 5, 1) },
    Crate { name: "dtoa", version: CrateVersion(1, 0, 9) },
    Crate { name: "urlencoding", version: CrateVersion(2, 1, 3) },
    Crate { name: "serde_bytes", version: CrateVersion(0, 11, 15) },
    Crate { name: "terminal_size", version: CrateVersion(0, 3, 0) },
    Crate { name: "tokio-io-timeout", version: CrateVersion(1, 2, 0) },
    Crate { name: "zeroize_derive", version: CrateVersion(1, 4, 2) },
    Crate { name: "android_system_properties", version: CrateVersion(0, 1, 5) },
    Crate { name: "tar", version: CrateVersion(0, 4, 41) },
    Crate { name: "windows", version: CrateVersion(0, 58, 0) },
    Crate { name: "curve25519-dalek", version: CrateVersion(4, 1, 3) },
    Crate { name: "xattr", version: CrateVersion(1, 3, 1) },
    Crate { name: "dyn-clone", version: CrateVersion(1, 0, 17) },
    Crate { name: "match_cfg", version: CrateVersion(0, 1, 0) },
    Crate { name: "aead", version: CrateVersion(0, 5, 2) },
    Crate { name: "plotters", version: CrateVersion(0, 3, 7) },
    Crate { name: "structopt", version: CrateVersion(0, 3, 26) },
    Crate { name: "lexical-core", version: CrateVersion(0, 8, 5) },
    Crate { name: "pretty_assertions", version: CrateVersion(1, 4, 0) },
    Crate { name: "structopt-derive", version: CrateVersion(0, 4, 18) },
    Crate { name: "unindent", version: CrateVersion(0, 2, 3) },
    Crate { name: "powerfmt", version: CrateVersion(0, 2, 0) },
    Crate { name: "float-cmp", version: CrateVersion(0, 9, 0) },
    Crate { name: "atomic-waker", version: CrateVersion(1, 1, 2) },
    Crate { name: "camino", version: CrateVersion(1, 1, 9) },
    Crate { name: "async-task", version: CrateVersion(4, 7, 1) },
    Crate { name: "tracing-serde", version: CrateVersion(0, 1, 3) },
    Crate { name: "hkdf", version: CrateVersion(0, 12, 4) },
    Crate { name: "crc", version: CrateVersion(3, 2, 1) },
    Crate { name: "tokio-tungstenite", version: CrateVersion(0, 23, 1) },
    Crate { name: "tiny-keccak", version: CrateVersion(2, 0, 2) },
    Crate { name: "predicates-core", version: CrateVersion(1, 0, 8) },
    Crate { name: "xml-rs", version: CrateVersion(0, 8, 22) },
    Crate { name: "predicates-tree", version: CrateVersion(1, 0, 11) },
    Crate { name: "crypto-bigint", version: CrateVersion(0, 5, 5) },
    Crate { name: "cmake", version: CrateVersion(0, 1, 51) },
    Crate { name: "ctr", version: CrateVersion(0, 9, 2) },
    Crate { name: "doc-comment", version: CrateVersion(0, 3, 3) },
    Crate { name: "term", version: CrateVersion(1, 0, 0) },
    Crate { name: "redox_users", version: CrateVersion(0, 4, 6) },
    Crate { name: "maplit", version: CrateVersion(1, 0, 2) },
    Crate { name: "plotters-backend", version: CrateVersion(0, 3, 7) },
    Crate { name: "yansi", version: CrateVersion(1, 0, 1) },
    Crate { name: "plotters-svg", version: CrateVersion(0, 3, 7) },
    Crate { name: "fxhash", version: CrateVersion(0, 2, 1) },
    Crate { name: "universal-hash", version: CrateVersion(0, 5, 1) },
    Crate { name: "net2", version: CrateVersion(0, 2, 39) },
    Crate { name: "colored", version: CrateVersion(2, 1, 0) },
    Crate { name: "tower-http", version: CrateVersion(0, 5, 2) },
    Crate { name: "iovec", version: CrateVersion(0, 1, 4) },
    Crate { name: "iana-time-zone-haiku", version: CrateVersion(0, 1, 2) },
    Crate { name: "opentelemetry", version: CrateVersion(0, 25, 0) },
    Crate { name: "cargo-platform", version: CrateVersion(0, 1, 8) },
    Crate { name: "num-derive", version: CrateVersion(0, 4, 2) },
    Crate { name: "futures-timer", version: CrateVersion(3, 0, 3) },
    Crate { name: "valuable", version: CrateVersion(0, 1, 0) },
    Crate { name: "threadpool", version: CrateVersion(1, 8, 1) },
    Crate { name: "hashlink", version: CrateVersion(0, 9, 1) },
    Crate { name: "brotli-decompressor", version: CrateVersion(4, 0, 1) },
    Crate { name: "serde_repr", version: CrateVersion(0, 1, 19) },
    Crate { name: "sha3", version: CrateVersion(0, 10, 8) },
    Crate { name: "alloc-no-stdlib", version: CrateVersion(2, 0, 4) },
    Crate { name: "alloc-stdlib", version: CrateVersion(0, 2, 2) },
    Crate { name: "polyval", version: CrateVersion(0, 6, 2) },
    Crate { name: "pyo3", version: CrateVersion(0, 22, 2) },
    Crate { name: "termtree", version: CrateVersion(0, 5, 1) },
    Crate { name: "brotli", version: CrateVersion(6, 0, 0) },
    Crate { name: "blocking", version: CrateVersion(1, 6, 1) },
    Crate { name: "crossbeam", version: CrateVersion(0, 8, 4) },
    Crate { name: "zip", version: CrateVersion(2, 2, 0) },
    Crate { name: "inout", version: CrateVersion(0, 1, 3) },
    Crate { name: "twox-hash", version: CrateVersion(1, 6, 3) },
    Crate { name: "Inflector", version: CrateVersion(0, 11, 4) },
    Crate { name: "pyo3-macros", version: CrateVersion(0, 22, 2) },
    Crate { name: "pyo3-macros-backend", version: CrateVersion(0, 22, 2) },
    Crate { name: "miow", version: CrateVersion(0, 6, 0) },
    Crate { name: "pem-rfc7468", version: CrateVersion(0, 7, 0) },
    Crate { name: "protobuf", version: CrateVersion(3, 5, 1) },
    Crate { name: "async-executor", version: CrateVersion(1, 13, 1) },
    Crate { name: "encode_unicode", version: CrateVersion(1, 0, 0) },
    Crate { name: "ignore", version: CrateVersion(0, 4, 23) },
    Crate { name: "enum-as-inner", version: CrateVersion(0, 6, 1) },
    Crate { name: "aes-gcm", version: CrateVersion(0, 10, 3) },
    Crate { name: "md5", version: CrateVersion(0, 7, 0) },
    Crate { name: "ff", version: CrateVersion(0, 13, 0) },
    Crate { name: "bzip2-sys", version: CrateVersion(0, 1, 11) },
    Crate { name: "ghash", version: CrateVersion(0, 5, 1) },
    Crate { name: "byte-tools", version: CrateVersion(0, 3, 1) },
    Crate { name: "group", version: CrateVersion(0, 13, 0) },
    Crate { name: "libsqlite3-sys", version: CrateVersion(0, 30, 1) },
    Crate { name: "headers", version: CrateVersion(0, 4, 0) },
    Crate { name: "pyo3-build-config", version: CrateVersion(0, 22, 2) },
    Crate { name: "png", version: CrateVersion(0, 17, 13) },
    Crate { name: "flume", version: CrateVersion(0, 11, 0) },
    Crate { name: "lru-cache", version: CrateVersion(0, 1, 2) },
    Crate { name: "elliptic-curve", version: CrateVersion(0, 13, 8) },
    Crate { name: "difflib", version: CrateVersion(0, 4, 0) },
    Crate { name: "wait-timeout", version: CrateVersion(0, 2, 0) },
    Crate { name: "indicatif", version: CrateVersion(0, 17, 8) },
    Crate { name: "headers-core", version: CrateVersion(0, 3, 0) },
    Crate { name: "number_prefix", version: CrateVersion(0, 4, 0) },
    Crate { name: "http-range-header", version: CrateVersion(0, 4, 1) },
    Crate { name: "async-compression", version: CrateVersion(0, 4, 12) },
    Crate { name: "ecdsa", version: CrateVersion(0, 16, 9) },
    Crate { name: "adler32", version: CrateVersion(1, 2, 0) },
    Crate { name: "new_debug_unreachable", version: CrateVersion(1, 0, 6) },
    Crate { name: "keccak", version: CrateVersion(0, 1, 5) },
    Crate { name: "unsafe-libyaml", version: CrateVersion(0, 2, 11) },
    Crate { name: "fs_extra", version: CrateVersion(1, 3, 0) },
    Crate { name: "error-chain", version: CrateVersion(0, 12, 4) },
    Crate { name: "string_cache", version: CrateVersion(0, 8, 7) },
    Crate { name: "codespan-reporting", version: CrateVersion(0, 11, 1) },
    Crate { name: "pathdiff", version: CrateVersion(0, 2, 1) },
];
