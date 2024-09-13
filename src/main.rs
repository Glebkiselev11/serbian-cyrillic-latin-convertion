use translate::SerbianCyrillic;

mod translate;

fn main() {
    println!("{}", SerbianCyrillic::from_latin("njegov čaj"));
    println!("{}", SerbianCyrillic::from_cyrillic("његов чај"));
}
