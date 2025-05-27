fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("res/app.ico"); // Or your actual path
    res.compile().expect("Failed to compile resources");
}
