use translate::SerbianTranslation;

mod translate;

fn main() {
    println!("{}", SerbianTranslation::from_latin("njegov čaj"));
    println!("{}", SerbianTranslation::from_cyrillic("његов чај"));
}
