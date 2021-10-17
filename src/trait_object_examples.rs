pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&mut self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    width: i32,
    height: i32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {}x{}", self.width, self.height);
    }
}

pub fn run(){
    let mut screen = Screen {
        components: vec![Box::new(Button {
            width: 50,
            height: 40,
            label: String::from("Click me baby one more time"),
        })],
    };
    screen.run();
}