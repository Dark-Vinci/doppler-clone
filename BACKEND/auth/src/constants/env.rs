pub const APP_NAME: &'static str = "APP_NAME";
pub const PORT: &'static str = "PORT";
pub const JWT_SECRET: &'static str = "JWT_SECRET";


fn me(a: i32) -> bool {
    if a.ge(&10) {
        true
    }

    let mut a = a;

    let num = loop {
        a += 1;

        if a < 30 {
            break a;
        }
    };
}

fn first_word(v: &String) -> &str {
    let by = v.as_bytes();

    for (i, &item) in by.iter().enumerate() {
        if item == b' ' {
            return &v[..i];
        }
    }

    return &v[..];
}

fn v<'a: 'b, 'b>(v1: &'a str, _: &'b str) -> &'b usize {
    return &v1.len();
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() -> () {
        let res = 2 + 2;

        assert_eq!(res, 4);
    }

    #[test]
    #[should_panic(expected = "meme")]
    fn panicker() {
        panic!("meme")
    }
}
