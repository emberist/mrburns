use regex::Regex;
use unidecode::unidecode;

pub fn slugify(a_string: &str) -> String {
    let text = unidecode(a_string).replace('&', "-and-").replace('_', "-");

    let spaces_re = Regex::new(r"\s+").unwrap();
    let special_chars_re = Regex::new(r"[^\w-]+").unwrap();
    let double_or_more_dash_re = Regex::new(r"--+").unwrap();

    let text_without_spaces = spaces_re.replace_all(&text, "-").to_string();

    let text_sanitized = special_chars_re.replace_all(&text_without_spaces, "-");

    double_or_more_dash_re
        .replace_all(&text_sanitized, "-")
        .to_string()
        .trim_matches('-')
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_slugify_a_simple_string() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn it_removes_special_characters() {
        assert_eq!(
            slugify("[Hello][world], & friends"),
            "hello-world-and-friends"
        );
        assert_eq!(slugify("   "), "");
        assert_eq!(slugify("hello_world"), "hello-world");
        assert_eq!(slugify("à"), "a");
        assert_eq!(slugify("A"), "a");
        assert_eq!(slugify("Ciao > foo--bar<"), "ciao-foo-bar");
        assert_eq!(
            slugify("bräñdßçhūtžtür & brændšchútztür"),
            "brandsschutztur-and-braendschutztur"
        );
        assert_eq!(slugify("BAZ!!!! 00?35Ae"), "baz-00-35ae");
    }

    #[test]
    fn it_removes_double_dashes_() {
        assert_eq!(slugify("foo--bar"), "foo-bar");
        assert_eq!(slugify("  foo bar   "), "foo-bar");
        assert_eq!(slugify("--foo bar----"), "foo-bar");
    }
}
