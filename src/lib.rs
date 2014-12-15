#[deriving(PartialEq)]
#[deriving(Show)]
pub struct Pair<'a> {
    pub key: &'a str,
    pub val: Option<&'a str>,
}

fn complete_pair<'a>(buf: &'a str, pair: Option<Pair<'a>>) -> Pair<'a> {
    match pair {
        Some(Pair { key: k, val: _ }) =>
            Pair { key: k, val: Some(buf) },
        None =>
            Pair { key: buf, val: None },
    }
}

pub fn parse(message: &str) -> Vec<Pair> {
    let mut buf_start = 0u;
    let mut pair: Option<Pair> = None;
    let mut pairs: Vec<Pair> = vec![];

    let mut escape = false;
    let mut garbage = false;
    let mut next = false;
    let mut quoted = false;

    for (i, c) in message.char_indices() {
        // if set, we've been told to move to the next token
        if next {
            buf_start = i;
            next = false;
        }

        match (quoted, c) {
            (false, ' ') => {
                if i > buf_start {
                    if !garbage {
                        // the buffer that we just processed is either a value
                        // or a valueless key depending on the current state of
                        // `pair`
                        pairs.push(complete_pair(message.slice(buf_start, i), pair));
                        pair = None;
                    }
                    next = true;
                }
                garbage = false;
            },
            (false, '=') => {
                if i > buf_start {
                    pair = Some(Pair { key: message.slice(buf_start, i), val: None });
                    next = true;
                } else {
                    garbage = true;
                }
            },
            (true, '\\') => {
                // @todo: this will need to be fixed
                //buf_start = buf_start + 1
                escape = true;
            }
            (_, '"') => {
                if escape {
                    // @todo: this will need to be fixed
                    //buf.push(c);
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
                    // @todo: this will need to be fixed
                    //buf.push('\\');
                    escape = false;
                }
            },
        }
    }

    // and process one final time at the end of the message to get the last
    // data point
    if !garbage {
        pairs.push(complete_pair(message.slice_from(buf_start), pair));
    }

    pairs
}
