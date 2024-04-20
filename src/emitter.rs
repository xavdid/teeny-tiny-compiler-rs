use std::fs;

pub struct Emitter {
    full_path: String,
    header: String,
    code: String,
}

impl Emitter {
    pub fn new(full_path: String) -> Self {
        Emitter {
            full_path,
            header: String::new(),
            code: String::new(),
        }
    }

    pub fn emit(&mut self, code: &str) {
        self.code.push_str(code);
    }
    pub fn emit_line(&mut self, code: &str) {
        self.emit(code);
        self.code.push('\n');
    }
    pub fn header_line(&mut self, code: &str) {
        self.header.push_str(code);
        self.header.push('\n');
    }
    pub fn write_file(&self) {
        let mut s = String::new();
        s.push_str(&self.header);
        s.push_str(&self.code);
        fs::write(&self.full_path, s)
            .unwrap_or_else(|_| panic!("failed to write file at {}", self.full_path));
    }
}
