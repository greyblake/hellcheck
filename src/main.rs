fn main() {
    // Necessary to make OpenSSL work in a static build.
    // See: https://github.com/emk/rust-musl-builder#making-openssl-work
    openssl_probe::init_ssl_cert_env_vars();

    hellcheck::run();
}
