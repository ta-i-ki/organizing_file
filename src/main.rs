use std::env;
use std::fs;
use std::io::{ self, Write };
use std::path::{ Path, PathBuf };

use walkdir::WalkDir;

fn get_category(extension: &str) -> &str {
    match extension {
        ".md" | ".pdf" | ".doc" | ".docx" | ".txt" | ".xlsx" | ".pptx" => "ドキュメント",
        ".jpg" | ".jpeg" | ".png" | ".gif" | ".bmp" => "画像",
        ".mp4" | ".mov" | ".avi" | ".mkv" => "動画",
        ".mp3" | ".wav" | ".m4a" => "音声",
        ".zip" | ".rar" | ".7z" => "アーカイブ",
        ".exe" | ".msi" => "アプリ",
        _ => "その他",
    }
}

fn create_unique_path(mut path: PathBuf) -> PathBuf {
    let mut counter = 1;
    while path.exists() {
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let parent = path.parent().unwrap();
                path = parent.join(format!("{} ({}){}.{}", stem, counter, "", ext));
            } else {
                path = path.with_file_name(format!("{} ({})", stem, counter));
            }
        }
        counter += 1;
    }
    path
}

fn sort_files(user_name: &str) -> io::Result<()> {
    //let download_path = Path::new("C:/Users").join(user_name).join("Downloads");
    let download_path = Path::new("C:/Users").join(user_name).join("Documents");
    let sorted_base = Path::new("C:/Users").join(user_name).join("sorted_downloads");

    if !download_path.exists() {
        println!("ダウンロードフォルダが存在しません: {:?}", download_path);
        return Ok(());
    }

    for entry in fs::read_dir(download_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext_osstr) = path.extension() {
                if let Some(ext_str) = ext_osstr.to_str() {
                    let ext = format!(".{}", ext_str.to_lowercase());
                    let category = get_category(&ext);
                    let dest_dir = sorted_base.join(category);
                    fs::create_dir_all(&dest_dir)?;

                    let dest_path = create_unique_path(dest_dir.join(path.file_name().unwrap()));
                    fs::rename(&path, &dest_path)?;
                    println!("✔ 移動: {:?} → {:?}", path.file_name().unwrap(), dest_path);
                }
            }
        }
    }
    Ok(())
}

fn main() {
    println!("ユーザー名を入力してください（例：taiki）: ");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).unwrap();
    let user_name = user_name.trim();

    match sort_files(user_name) {
        Ok(_) => println!("\n分類が完了しました。"),
        Err(e) => eprintln!("エラーが発生しました: {}", e),
    }
}
