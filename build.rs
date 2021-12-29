use chrono::Utc;

fn main() {
    let utc_date = Utc::today();

    println!("cargo:rustc-env=RELEASE_DATE={}", utc_date.format("%d/%m/%Y"));
}
