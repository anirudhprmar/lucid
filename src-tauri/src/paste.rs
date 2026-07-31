use arboard::Clipboard;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

static PASTE_GENERATION: AtomicU64 = AtomicU64::new(0);

pub fn paste_text(text: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let my_gen = PASTE_GENERATION.fetch_add(1, Ordering::SeqCst) + 1;

    let saved: Option<String> = {
        let mut clipboard = Clipboard::new()?;
        clipboard.get_text().ok()
    };

    {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(text)?;
    }

    let mut enigo = Enigo::new(&Settings::default())?;
    enigo.key(Key::Control, Direction::Press)?;
    enigo.key(Key::V, Direction::Click)?;
    enigo.key(Key::Control, Direction::Release)?;

    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(150));

        let current = PASTE_GENERATION.load(Ordering::SeqCst);
        if current != my_gen {
            println!("Skipping clipboard restore for generation {}", my_gen);
            return;
        }

        match saved {
            Some(prev) => {
                if let Ok(mut clipboard) = Clipboard::new() {
                    if let Err(err) = clipboard.set_text(prev) {
                        println!("Failed to restore clipboard: {}", err);
                    }
                }
            }

            None => {
                println!("No clipboard content to restore")
            }
        }
    });

    Ok(())
}
