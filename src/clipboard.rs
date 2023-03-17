use arboard::Clipboard;

pub fn copy(contents: String) {
    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => { clipboard },
        Err(error) => { panic!("Problem opening the clipboard: {:?}", error) },
    };

    match clipboard.set_text(contents) {
        Ok(()) => { println!("copied!") }
        Err(error) => { panic!("Problem setting the clipboard: {:?}", error) },
    };
}