use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
// use std::time::{Duration, Instant};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Eq, PartialEq)]
pub enum PlayStatus {
    Waiting,
    // Playing(Instant, Duration),
    // Paused(Duration),
    Playing,
    Paused,
}

pub struct PlayListItem {
    pub name: String,
    pub status: PlayStatus,
    pub path: String,
    // pub duration: Duration,
}

pub struct PlayList {
    pub list: Vec<PlayListItem>,
}
pub struct MusicPlayer {
    pub play_list: PlayList,
    pub output_stream: OutputStream,
    pub output_stream_handle: OutputStreamHandle,
    pub sink: Sink,
}

impl MusicPlayer {
    pub fn new() -> Self {
        let (output_stream, output_stream_handle) =
            OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();
        let play_list = PlayList {
            list: vec![],
        };
        MusicPlayer {
            play_list,
            output_stream,
            output_stream_handle,
            sink,
        }
    }

    // fn tick(&mut self) -> bool {
    //     todo!()
    // }
    pub fn play(&mut self) -> bool {
        self.sink.play();
        if let Some(item) = self.play_list.list.first_mut() {
            item.status = PlayStatus::Playing;
        }
        true
    }
    pub fn pause(&mut self) -> bool {
        self.sink.pause();
        if let Some(item) = self.play_list.list.first_mut() {
            item.status = PlayStatus::Paused;
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        self.sink.stop();
        true
    }

    pub fn add_to_playlist(&mut self, path: String, rebuild: bool) -> bool {
        self.play_from_path(path, rebuild)
    }
    fn play_from_path(&mut self, path: String, rebuild: bool) -> bool {
        match File::open(&path) {
            Ok(f) => {
                let p = Path::new(path.as_str());
                let file_name = p.file_name().unwrap().to_string_lossy().to_string();
                if rebuild || self.play_list.list.is_empty() {
                    self.stop();
                    let buf = BufReader::new(f);
                    let sink = self.output_stream_handle.play_once(buf).unwrap();
                    self.sink = sink;
                    self.play_list.list.clear();
                }
                let new_item = PlayListItem {
                    name: file_name,
                    path: path.clone(),
                    status: PlayStatus::Waiting,
                };
                self.play_list.list.push(new_item);
                self.play();
                // self.tick();
                return true;
            }
            _ => false
        }
    }

    pub fn is_play(&self) -> bool {
        if let Some(item) = self.play_list.list.first() {
            return item.status == PlayStatus::Playing;
        }
        false
    }
}