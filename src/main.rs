use std::process::Command;

fn main() {
    let mdn_url = build_mdn_url();
    let open_command = Command::new("open")
        .arg(mdn_url)
        .spawn();
}

fn build_mdn_url() -> &'static str {
    const ROOT : &'static str = "https://developer.mozilla.org/en-US/";

    return ROOT;
}
