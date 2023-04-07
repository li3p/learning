use std::borrow::Cow;

use url::Url;

fn main() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
    let mut pairs = url.query_pairs();

    let (mut k, v) = pairs.next().unwrap();
    // just because k, v are all Cow<str>, so we can use them as &str or String
    // for now, they are all Cow::Borrowed(&str)
    println!("k: {:?}, v: {:?}", k, v);

    // when mutating k, v, they will be converted to Cow::Owned(String)
    k.to_mut().push_str("_lala");

    print_pairs(pairs.next().unwrap());

    print_pairs(pairs.next().unwrap());
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("k: {:?}, v: {:?}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(s) => format!("Borrowed({})", s),
        Cow::Owned(s) => format!("Owned({})", s),
    }
}