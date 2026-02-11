fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("./src/assets/oxide_plus.ico");
        res.compile().unwrap();
    }
}
