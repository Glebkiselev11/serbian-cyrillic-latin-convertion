use translate::SerbianCyrillic;

mod translate;

fn main() {
    println!("{}", SerbianCyrillic::from_latin("njegov čaj"));
}
