fn xfind(path: &str) {
    let unknown = std::path::Path::new(path);
    // ディレクトリかどうか
    if unknown.is_dir() {
        let name = unknown.file_name().unwrap();
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
