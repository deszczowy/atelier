use curl::easy::{Easy2, Handler, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn get(url: String) -> String {
    let mut easy = Easy2::new(Collector(Vec::new()));
    
    easy.get(true).unwrap();
    easy.url("https://wszedziepudla.pl").unwrap();
    easy.perform().unwrap();

    if (easy.response_code().unwrap() == 200) {
        let contents = easy.get_ref();
        String::from_utf8_lossy(&contents.0).to_string()
    } else {
        "".to_string()
    }
}

pub fn go() {
    let mut x = get("http://wszedziepudla.pl".to_string());

    unsafe {
        println!("Length = {} Body = {}", x.len(), x.get_unchecked_mut(0..500));
    }
}
