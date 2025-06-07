use druid::Lens;

pub struct UsizeToString;

impl Lens<usize, String> for UsizeToString {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &usize, f: F) -> V {
        let s = if *data == 0 {
            String::new() // show empty string instead of "0"
        } else {
            data.to_string()
        };
        f(&s)
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut usize, f: F) -> V {
        let mut s = if *data == 0 {
            String::new()
        } else {
            data.to_string()
        };
        let result = f(&mut s);

        if s.trim().is_empty() {
            *data = 0;
        } else if let Ok(parsed) = s.parse::<usize>() {
            *data = parsed;
        }

        result
    }
}
