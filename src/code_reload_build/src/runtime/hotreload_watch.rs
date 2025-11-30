use crate::runtime::{HotreloadLibrary, IHotreloadPayload};
use notify::Watcher;
use std::sync::RwLock;

pub fn start_watch<T: IHotreloadPayload + Send + Sync>(
    locked_hotreload_library: &'static RwLock<HotreloadLibrary<T>>,
) {
    let mut watcher = notify::recommended_watcher(|event| {
        match &event {
            Ok(notify::Event {
                kind: notify::EventKind::Create(_) | notify::EventKind::Modify(_),
                ..
            }) => {}
            _ => return,
        }
        HotreloadLibrary::reload_locked(locked_hotreload_library);
    })
    .unwrap();

    let watch_target = locked_hotreload_library
        .read()
        .unwrap()
        .get_source_library_path();

    watcher
        .watch(&watch_target, notify::RecursiveMode::NonRecursive)
        .unwrap();

    std::mem::forget(watcher);
}
