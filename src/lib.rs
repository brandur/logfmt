#[deriving(PartialEq)]
#[deriving(Show)]
pub struct Pair<'a> {
    key_slice: Option<&'a str>,
    val_slice: Option<&'a str>,

    key_string: Option<String>,
    val_string: Option<String>,
}

impl<'a> Pair<'a> {
    fn key(&self) -> &str {
        match (&self.key_slice, &self.key_string) {
            (Some(ref v), None) => v,
            (None, Some(ref v)) => v.as_slice(),
            _               => panic!(),
        }
    }

/*
    fn val(&self) -> &str {
        match (self.val_slice, self.val_string) {
            (Some(v), None) => v,
            (None, Some(v)) => &v,
            _               => panic!(),
        }
    }
*/
}

/*
#[deriving(PartialEq)]
#[deriving(Show)]
pub struct Pair {
    pub key: String,
    pub val: Option<String>,
}

fn complete_pair(buf: String, pair: Option<Pair>) -> Pair {
    match pair {
        Some(Pair { key: k, val: _ }) =>
            Pair { key: k, val: Some(buf) },
        None =>
            Pair { key: buf, val: None },
    }
}
*/

pub fn parse(message: &str) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = vec![];

    let p = Pair { key_slice: Some(message), key_string: None, val_slice: Some(message), val_string: None };
    pairs.push(p);

    let s = message.to_string();
    let p = Pair { key_slice: None, key_string: Some(s), val_slice: Some(message), val_string: None };

    pairs.push(p);
    pairs

/*
    let mut pair: Option<Pair> = None;
    let mut pairs: Vec<Pair> = vec![];
    let mut buf = String::with_capacity(message.len());
    let mut buf_pos = 0u;

    let mut escape = false;
    let mut garbage = false;
    let mut quoted = false;

    for c in message.chars() {
        match (quoted, c) {
            (false, ' ') => {
                let slice = buf.as_slice().slice_from(buf_pos).to_string();
                if !slice.is_empty() {
                    buf_pos += slice.len();
                    if !garbage {
                        // the buffer that we just processed is either a value
                        // or a valueless key depending on the current state of
                        // `pair`
                        pairs.push(complete_pair(slice, pair));
                        pair = None;
                    }
                }
                garbage = false;
            },
            (false, '=') => {
                let slice = buf.as_slice().slice_from(buf_pos).to_string();
                if !slice.is_empty() {
                    buf_pos += slice.len();
                    pair = Some(Pair { key: slice, val: None });
                } else {
                    garbage = true;
                }
            },
            (true, '\\') => {
                escape = true;
            }
            (_, '"') => {
                if escape {
                    buf.push(c);
                    escape = false;
                } else {
                    quoted = !quoted;
                }
            },
            _ => {
                // if the last character we read was an escape, but this
                // character was not a quote, then store the escape back into the
                // buffer
                if escape {
                    buf.push('\\');
                    escape = false;
                }
                buf.push(c);
            },
        }
    }

    // and process one final time at the end of the message to get the last
    // data point
    if !garbage {
        let slice = buf.as_slice().slice_from(buf_pos).to_string();
        pairs.push(complete_pair(slice, pair));
    }

    pairs
*/
}
