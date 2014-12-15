#[deriving(PartialEq)]
#[deriving(Show)]
pub struct Pair<'a> {
    pub key: &'a str,
    pub val: Option<&'a str>,
}

pub struct PairCollection<'a> {
    buf: String,
    pub pairs: Vec<Pair<'a>>,
}

fn complete_pair<'a>(buf: &'a str, pair: &Option<Pair<'a>>) -> Pair<'a> {
    match *pair {
        Some(Pair { key: k, val: _ }) =>
            Pair { key: k, val: Some(buf) },
        None =>
            Pair { key: buf, val: None },
    }
}

pub fn parse(message: &str) -> PairCollection {
    let mut pair: Option<Pair> = None;
    let mut pairs: Vec<Pair> = vec![];
    let mut buf = String::with_capacity(message.len());
    let mut buf_pos = 0u;

    let mut escape = false;
    let mut garbage = false;
    let mut quoted = false;

    for (i, c) in message.char_indices() {
        match (quoted, c) {
            (false, ' ') => {
                let slice = buf.as_slice().slice_from(buf_pos);
                if !slice.is_empty() {
                    if !garbage {
                        // the buffer that we just processed is either a value
                        // or a valueless key depending on the current state of
                        // `pair`
                        pairs.push(complete_pair(slice, &pair));
                        pair = None;
                    }
                    buf_pos += slice.len();
                }
                garbage = false;
            },
            (false, '=') => {
                let slice = buf.as_slice().slice_from(buf_pos);
                if !slice.is_empty() {
                    pair = Some(Pair { key: slice, val: None });
                    buf_pos += slice.len();
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
        let slice = buf.as_slice().slice_from(buf_pos);
        pairs.push(complete_pair(slice, &pair));
    }

    // trim any excess capacity off the buffer (discrepancies being caused by
    // quotes and escape characters)
    buf.shrink_to_fit();

    PairCollection { buf: buf, pairs: pairs }
}
