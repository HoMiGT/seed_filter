use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <txt> <source_dir> <target_dir>", args[0]);
        std::process::exit(1);
    }

    let txt_path = &args[1];
    let source_dir = &args[2];
    let target_dir = &args[3];

    // 读取txt文件中的文件夹名
    let file = File::open(txt_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let folder_name = line?;
        let src_folder_path = Path::new(source_dir).join(&folder_name);
        let dst_folder_path = Path::new(target_dir).join(&folder_name);

        if src_folder_path.is_dir() {
            // 创建目标文件夹
            fs::create_dir_all(&dst_folder_path)?;

            // 复制文件夹中的文件
            for entry in fs::read_dir(src_folder_path)? {
                let entry = entry?;
                let src_file_path = entry.path();
                if src_file_path.is_file() {
                    let file_name = src_file_path.file_name().unwrap();
                    let dst_file_path = dst_folder_path.join(file_name);
                    fs::copy(&src_file_path, &dst_file_path)?;
                }
            }
        } else {
            eprintln!("Source folder not found: {}", src_folder_path.display());
        }
    }

    Ok(())
}
