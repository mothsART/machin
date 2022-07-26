#[macro_export]
macro_rules! colored_err {
    ($content:expr) => {
        eprintln!(
            "{}",
            $content
            .white()
            .on_red()
        )
    }
}

#[macro_export]
macro_rules! colored_success {
    ($content:expr) => {
        println!("{}", $content.white().on_green());
    }
}
