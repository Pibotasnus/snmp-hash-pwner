#[derive(Debug)]
pub struct Context {
    pub message: String,
    pub hash: String,
    pub engine_id: String,
}

impl Context {
    pub fn new(raw_fc: String) -> Result<Context, &'static str> {
        let mut splitted_raw = raw_fc.split(":");
        let expected_length: usize = 3;
        if &splitted_raw.clone().count() != &expected_length {
            return Err("Not enough values in target file or using wrong format");
        }
        let message = match &splitted_raw.next() {
            Some(msg) => msg.to_string(),
            None => panic!("Empty message"),
        };
        let hash = match &splitted_raw.next() {
            Some(hash) => hash.to_string(),
            None => panic!("Empty message"),
        };
        let engine_id = match &splitted_raw.next() {
            Some(engine_id) => engine_id.to_string(),
            None => panic!("Empty message"),
        };
        Ok(Context {
            message: message.replace(&hash, "000000000000000000000000"),
            hash,
            engine_id,
        })
    }
}
