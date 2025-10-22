use std::path::PathBuf;

#[cfg(not(all(feature = "gtk4-native", target_os = "linux")))]
mod backend {
    use super::*;
    use rfd::FileDialog;

    pub fn pick_file() -> Option<PathBuf> {
        FileDialog::new().pick_file()
    }

    pub fn pick_folder() -> Option<PathBuf> {
        FileDialog::new().pick_folder()
    }

    pub fn save_file_with_name(suggested_name: &str) -> Option<PathBuf> {
        FileDialog::new().set_file_name(suggested_name).save_file()
    }

    pub fn pick_manifest_file() -> Option<PathBuf> {
        FileDialog::new()
            .add_filter("Manifest", &["json", "csv", "txt", "mf"])
            .pick_file()
    }
}

#[cfg(all(feature = "gtk4-native", target_os = "linux"))]
mod backend {
    use super::*;
    use gio::prelude::*;
    use gio::{File, ListStore};
    use glib::MainContext;
    use glib::MainContextExtManual;
    use glib::{self, BoolError};
    use gtk::prelude::*;
    use gtk::{FileDialog as GtkFileDialog, FileFilter};
    use tracing::warn;

    pub fn pick_file() -> Option<PathBuf> {
        with_dialog(|dialog| dialog.open_future(None::<&gtk::Window>))
    }

    pub fn pick_folder() -> Option<PathBuf> {
        with_dialog(|dialog| dialog.select_folder_future(None::<&gtk::Window>))
    }

    pub fn save_file_with_name(suggested_name: &str) -> Option<PathBuf> {
        with_dialog(|dialog| {
            dialog.set_initial_name(Some(suggested_name));
            dialog.save_future(None::<&gtk::Window>)
        })
    }

    pub fn pick_manifest_file() -> Option<PathBuf> {
        with_custom_dialog(|dialog| {
            let list: ListStore = ListStore::new::<FileFilter>();
            let filter = FileFilter::new();
            filter.set_name(Some("Manifest"));
            for ext in ["json", "csv", "txt", "mf"] {
                filter.add_pattern(&format!("*.{}", ext));
            }
            list.append(&filter);
            dialog.set_filters(Some(&list));
            dialog.open_future(None::<&gtk::Window>)
        })
    }

    fn with_dialog<F, Fut>(configure: F) -> Option<PathBuf>
    where
        F: FnOnce(&GtkFileDialog) -> Fut,
        Fut: std::future::Future<Output = Result<Option<File>, glib::Error>>,
    {
        with_custom_dialog(configure)
    }

    fn with_custom_dialog<F, Fut>(configure: F) -> Option<PathBuf>
    where
        F: FnOnce(&GtkFileDialog) -> Fut,
        Fut: std::future::Future<Output = Result<Option<File>, glib::Error>>,
    {
        if let Err(err) = ensure_gtk_init() {
            warn!("failed to initialise GTK4 dialog: {err}");
            return None;
        }
        let dialog = GtkFileDialog::builder().modal(true).build();
        let ctx = MainContext::default();
        let guard = match ctx.acquire() {
            Some(guard) => guard,
            None => {
                warn!("failed to acquire GTK main context");
                return None;
            }
        };
        let result = ctx.block_on(configure(&dialog));
        drop(guard);
        match result {
            Ok(Some(file)) => file.path().map(PathBuf::from),
            Ok(None) => None,
            Err(err) => {
                warn!("GTK4 dialog error: {err}");
                None
            }
        }
    }

    fn ensure_gtk_init() -> Result<(), BoolError> {
        if gtk::is_initialized() {
            Ok(())
        } else {
            gtk::init()
        }
    }
}

pub use backend::{pick_file, pick_folder, pick_manifest_file, save_file_with_name};
