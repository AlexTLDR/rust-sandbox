// This struct holds a reference, so it needs a lifetime 'a
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ",
    );
    {
        let novel_2 = String::from("FooBarBuzzFooBarBuzz...");
        let e2 = ImportantExcerpt {
            part: &novel_2[0..3],
        };
    }
    // The instance 'first_sentence' cannot outlive 'novel'
    let first_sentence = ImportantExcerpt { part: &novel[0..5] };
    // If 'novel' were dropped here, 'first_sentence' would become invalid.
}
