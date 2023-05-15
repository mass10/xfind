fn fix_drive_path(s: &str) -> String {
    if s.len() == 2 {
        // "C:" のような場合
        let first_letter = s.chars().nth(0).unwrap();
        if !first_letter.is_ascii_alphabetic() {
            return s.to_string();
        }
        let second_letter = s.chars().nth(1).unwrap();
        if second_letter != ':' {
            return s.to_string();
        }
        return format!("{}:\\", first_letter.to_ascii_uppercase()).to_string();
    }
    return s.to_string();
}

fn xfind(path: &str) {
    let path = fix_drive_path(path);
    let unknown = std::path::Path::new(&path);
    // ディレクトリかどうか
    if unknown.is_dir() {
        let name = unknown.file_name().unwrap_or_default();
        if name == "node_modules" {
            return;
        }
        if name == ".git" {
            return;
        }
        if name == ".dotnet" {
            return;
        }
        if name == ".cargo" {
            return;
        }
        if name == ".node-gyp" {
            // ".node-gyp" は出力
            println!("directory: {}", path);
        } else if name == "Cache" {
            if path.contains("node-gyp") {
                // 上位に node-gyp を含む場合は出力
                println!("directory: {}", path);
            }
        }
        // ディレクトリの場合
        let result = std::fs::read_dir(path);
        if result.is_err() {
            return;
        }
        let dir = result.unwrap();
        for entry in dir {
            let entry = entry;
            if entry.is_err() {
                continue;
            }
            let path = entry.unwrap().path();
            let path_str = path.to_str().unwrap();
            xfind(path_str);
        }
    } else {
        // ファイルの場合
        // println!("{}", path);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for e in args {
        xfind(&e);
    }
    println!("ok.");
}
