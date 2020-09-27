use web_view::*;

fn main() {
    web_view::builder()
        .title("rFactor2 Manager")
        .content(Content::Html(include_str!("../web/dist/index.html")))
        .size(1024, 768)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
