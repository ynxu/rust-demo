
fn main() {
    println!("Hello, world!");
    greet_hello();
}

fn greet_hello(){
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好!";
    let english = "world,hello!";
    let regions = [southern_germany,chinese,english];
    for region in regions.iter() {
        println!("{}",region);
    }
}