use jstime_core as jstime;

fn main() {
    jstime::init(None);
    let mut scope = jstime::JSTime::new(
        jstime::Options::default()
    );
    scope.run_script("console.log('Hello, World!');", "jstime")
        .expect("ruhroh something went wrong");
}
