use gdk_pixbuf::Pixbuf;
use gdk_pixbuf::InterpType::Bilinear;

use std::path::Path;

pub struct BufManager {
    bufs: Vec<Option<Pixbuf>>,
    index: usize,
    current_buf: Option<Pixbuf>,
    width: i32,
    height: i32
}


const ZOOM_FACTOR: f32 = 1.414;
impl BufManager {
    pub fn new_from_file(file: &str) -> BufManager {
        let path = Path::new(file).canonicalize().unwrap();
        let parent = path.parent().unwrap();
        let mut files: Vec<_> = parent.read_dir().unwrap()
            .filter(|d| d.as_ref().unwrap().path().is_file())
            .map(|d| d.unwrap().path().to_str().unwrap().to_string())
            .collect();
        files.sort();

        let bufs: Vec<Option<Pixbuf>> = files.iter().map(|f| Pixbuf::new_from_file(&f).ok())
            .filter(|x| x.is_some())
            .collect();

        let width = 400;
        let height = 400;

        let current_buf = bufs[0].as_ref().and_then(|b| b.scale_simple(width, height, Bilinear).ok());

        BufManager {
            bufs: bufs,
            index: 0,
            width: width,
            height: height,
            current_buf: current_buf
        }
    }


    pub fn go_right(&mut self) {
        self.index += 1;
        self.index %= self.bufs.len();
        self.update_buf();
        println!("{}", self.index);
    }

    pub fn go_left(&mut self) {
        self.index += self.bufs.len() - 1;
        self.index %= self.bufs.len();
        self.update_buf();
        println!("{}", self.index);
    }

    pub fn zoom_in(&mut self) {
        self.width = (self.width as f32 * ZOOM_FACTOR) as i32;
        self.height = (self.width as f32 * ZOOM_FACTOR) as i32;
        self.update_buf();
    }


    pub fn zoom_out(&mut self) {
        self.width = (self.width as f32 / ZOOM_FACTOR) as i32;
        self.height = (self.width as f32 / ZOOM_FACTOR) as i32;
        self.update_buf();
    }

    fn update_buf(&mut self) {
        self.current_buf = self.bufs[self.index].as_ref().and_then(|b| b.scale_simple(self.width, self.height, Bilinear).ok())
    }

    pub fn get_buf(&self) -> Option<&Pixbuf> {
        self.current_buf.as_ref()
    }
}
