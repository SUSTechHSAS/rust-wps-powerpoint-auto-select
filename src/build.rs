use winres;

fn main(){
    // println!("{:?}", std::env::current_dir().unwrap());
    let mut res = winres::WindowsResource::new();
    res.set_icon("res/method-draw-image.ico");
    res.compile().unwrap();
}