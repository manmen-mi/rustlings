pub struct UrlReplacer {
    base_url: String
}

const EN_BASE_URL: &str = "https://doc.rust-lang.org/book";

impl UrlReplacer {
    pub fn new(mut base_url: String) -> Self {
        base_url = if base_url.ends_with('/') {
            base_url.trim_end_matches('/').to_owned()
        } else {
            base_url
        };

        Self {
            base_url
        }
    }

    pub fn replace(&self, hint: &str) -> String {
        hint.replace(EN_BASE_URL, &self.base_url)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DOMAIN: &str = "https://doc.rust-kr.org";

    #[test]
    fn non_rustbook_url() {
        let replacer = UrlReplacer::new(String::from(TEST_DOMAIN));

        let hint = "\
hints (...) lines (...)
link: https://example.com/ch03-02-data-types.html";

        assert_eq!(hint, replacer.replace(hint));
    }


    #[test]
    fn replace_rustbook_url() {
        let replacer = UrlReplacer::new(String::from(TEST_DOMAIN));

        let hint = "\
hints (...) lines (...)
link: https://doc.rust-lang.org/book/ch03-02-data-types.html";

        assert_eq!("\
hints (...) lines (...)
link: https://doc.rust-kr.org/ch03-02-data-types.html", replacer.replace(hint));
    }

    #[test]
    fn trim_end_with_slash() {
        let replacer = UrlReplacer::new(String::from(TEST_DOMAIN));

        assert_eq!(TEST_DOMAIN, replacer.base_url);
    }
}