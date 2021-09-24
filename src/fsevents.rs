use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use nightmare::events::EventProxy;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

use crate::listener::{Listener, MessageCtx};
use crate::message::Message;

pub struct FsEvent(PathBuf);

pub struct PluginWatcher {
    watcher: RecommendedWatcher,
    rx: mpsc::Receiver<DebouncedEvent>,
    proxy: EventProxy<PathBuf>,
    path: PathBuf,
}

impl PluginWatcher {
    pub fn new(path: impl Into<PathBuf>, proxy: EventProxy<PathBuf>) -> Result<Self> {
        let (tx, rx) = mpsc::channel();
        let watcher = RecommendedWatcher::new(tx, Duration::from_secs(2))?;
        let inst = Self {
            watcher,
            rx,
            proxy,
            path: path.into(),
        };

        Ok(inst)
    }

    pub fn watch(mut self) -> ! {
        self.watcher.watch(&self.path, RecursiveMode::Recursive);

        use DebouncedEvent::*;
        loop {
            match self.rx.recv() {
                Ok(
                    NoticeWrite(pb) | NoticeRemove(pb) | Create(pb) | Write(pb) | Remove(pb)
                    | Chmod(pb),
                ) => drop(self.proxy.send_event(pb)),
                _ => {}
            }
        }
    }
}
