use rtop_dev::plugin::Plugin;

struct ExampleWidget {}

impl Plugin for ExampleWidget {
    fn display(&mut self, _height: i32, _width: i32) -> String {
        String::from("Hello World RTop!")
    }
}

#[no_mangle]
pub fn init_template() -> (Box<dyn Plugin>, bool) {
    (Box::new(ExampleWidget {}), false)
}
