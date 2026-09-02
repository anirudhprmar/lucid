use arboard::Clipboard;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

static PASTE_GENERATION: AtomicU64 = AtomicU64::new(0);

pub fn paste_text(text: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if text.is_empty() {
        return Ok(());
    }

    let mut enigo = Enigo::new(&Settings::default())?;

    // Direct typing via unicode SendInput (KEYEVENTF_UNICODE)
    // Does not touch modifier states, preventing physical Space repeats from leaking
    if let Err(e) = enigo.text(text) {
        eprintln!(
            "Direct text input failed: {}, falling back to clipboard paste",
            e
        );

        let my_gen = PASTE_GENERATION.fetch_add(1, Ordering::SeqCst) + 1;
        let saved: Option<String> = {
            let mut clipboard = Clipboard::new()?;
            clipboard.get_text().ok()
        };

        {
            let mut clipboard = Clipboard::new()?;
            clipboard.set_text(text)?;
        }

        let _ = enigo.key(Key::Control, Direction::Press);
        let _ = enigo.key(Key::V, Direction::Click);
        let _ = enigo.key(Key::Control, Direction::Release);

        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(200));
            let current = PASTE_GENERATION.load(Ordering::SeqCst);
            if current != my_gen {
                return;
            }
            if let Some(prev) = saved {
                if let Ok(mut clipboard) = Clipboard::new() {
                    let _ = clipboard.set_text(prev);
                }
            }
        });
    }

    Ok(())
}
