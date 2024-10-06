use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc
// -> dは空文字, 7, abcの配列を持つ

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),        // aのような場合
    Multiple(Vec<&'buf str>), // d のような場合
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// FromStr はライフタイムを持ってたらつかえないので
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            // &c& のようなケース
            let mut key = sub_str;
            let mut val = "";
            // &b=2&のようなケース
            if let Some(i) = sub_str.find('=') {
                // = も確実に1byteなので
                key = &sub_str[..i]; // = より前の部分
                val = &sub_str[i + 1..] // = より後ろ
            }
            // data HashMapに追加
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    // すでにひとつ値が存在している場合、Value::Single からValue::Multipleに変換する
                    // どちらもValue Enumなので、容量の問題は大丈夫(最大のものに合わせて、確保されるから)
                    Value::Single(prev_val) => *existing = Value::Multiple(vec![prev_val, val]),
                    Value::Multiple(vec) => vec.push(&val),
                })
                .or_insert(Value::Single(&val));
        }
        QueryString { data }
    }
}
