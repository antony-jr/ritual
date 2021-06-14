use json::parse;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

use crate::banner;

fn get_directory_name(src_dir: &str) -> Option<String> {
    let root = Path::new(src_dir);
    if !root.is_dir() {
        return None;
    }

    match fs::canonicalize(root) {
        Ok(abs_path) => {
            let absolute_path = abs_path.as_path();

            match absolute_path.file_name() {
                Some(name) => match name.to_str() {
                    Some(name_str) => {
                        return Some(String::from(name_str));
                    }
                    _ => {}
                },
                _ => {}
            };
        }
        _ => {}
    };

    None
}

fn validate_directory_structure(src_dir: &str, pb: &ProgressBar) -> bool {
    match fs::read_dir(src_dir) {
        Ok(dir) => {
            let root = Path::new(src_dir);
            if !root.is_dir() {
                return false;
            }
            for entry in dir {
                pb.set_position(pb.position());

                match entry {
                    Ok(dentry) => {
                        let path = dentry.path();
                        match path.file_name() {
                            Some(name) => {
                                match name.to_str() {
                                    Some(name_str) => {
                                        if name_str != "audio"
                                            && name_str != "actions"
                                            && name_str != "meta.json"
                                        {
                                            return false;
                                        }

                                        // If it's the audio directory,
                                        // check if it's actually a directory.
                                        if name_str == "audio" && !path.is_dir() {
                                            return false;
                                        }

                                        if name_str == "actions" && !path.is_dir() {
                                            return false;
                                        }

                                        if name_str == "meta.json" && !path.is_file() {
                                            return false;
                                        }
                                    }
                                    _ => {}
                                };
                            }
                            _ => {}
                        };
                    }
                    Err(_) => {
                        return false;
                    }
                }
            }
            pb.inc(10);
            true
        }
        Err(_) => false,
    }
}

fn get_frame_count(src_dir: &Path) -> Option<i32> {
    let mut file_count: i32 = 0;
    match fs::read_dir(src_dir) {
        Ok(dir) => {
            for entry in dir {
                match entry {
                    Ok(dentry) => {
                        let path = dentry.path();
                        if !path.is_file() {
                            return None;
                        }
                        file_count += 1;
                    }
                    Err(_) => {
                        return None;
                    }
                }
            }
            return Some(file_count);
        }
        Err(_) => {
            return None;
        }
    };
}

// Parse a Spirit Directory
// Assumes that the given src_dir is a valid one.
fn parse_directory(src_dir: &str, pb: &ProgressBar) -> Option<json::JsonValue> {
    // First try parse meta.json
    let mut path = PathBuf::new();
    path.push(src_dir);
    path.push("meta.json");

    match fs::read_to_string(path.as_path()) {
        Ok(contents) => {
            match parse(&contents) {
                Ok(value) => {
                    pb.set_position(pb.position());
                    // Check for name key
                    if !value.has_key("name") || !value["name"].is_string() {
                        return None;
                    }
                    if !value.has_key("version") || !value["version"].is_string() {
                        return None;
                    }
                    if !value.has_key("author") || !value["author"].is_string() {
                        return None;
                    }
                    if !value.has_key("copyright") || !value["copyright"].is_string() {
                        return None;
                    }
                    if !value.has_key("positions") || !value["positions"].is_object() {
                        return None;
                    }

                    if !value.has_key("actions") || !value["actions"].is_object() {
                        return None;
                    }

                    pb.inc(20);

                    // Check if all actions present in meta.json
                    // exists
                    // Also check frames
                    let mut actions_count = 0;
                    let mut actions_path = PathBuf::new();
                    actions_path.push(src_dir);
                    actions_path.push("actions");

                    let mut audio_path = PathBuf::new();
                    audio_path.push(src_dir);
                    audio_path.push("audio");

                    for entry in value["actions"].entries() {
                        actions_count += 1;
                        actions_path.push(entry.0);

                        if !actions_path.as_path().is_dir() {
                            return None;
                        }

                        let jvalue = entry.1;
                        if !jvalue.is_object() {
                            return None;
                        }

                        let mut _got_frames: bool = false;
                        let mut _got_play: bool = false;
                        let frames_count = match get_frame_count(actions_path.as_path()) {
                            Some(v) => v,
                            _ => {
                                return None;
                            }
                        };
                        for info in jvalue.entries() {
                            if info.0 != "frames"
                                && info.0 != "play"
                                && info.0 != "loop"
                                && info.0 != "interval"
                            {
                                return None;
                            }

                            if info.0 == "frames" && info.1.is_array() {
                                // Check all frames
                                for member in info.1.members() {
                                    match member.as_str() {
                                        Some(range) => {
                                            let mut count = 0;
                                            let mut from: i32 = 0;
                                            let mut to: i32 = 0;
                                            for part in range.split('-') {
                                                if count == 0 {
                                                    from = match part.parse::<i32>() {
                                                        Ok(integer) => integer,
                                                        _ => 0,
                                                    };
                                                } else {
                                                    to = match part.parse::<i32>() {
                                                        Ok(integer) => integer,
                                                        _ => -1,
                                                    };
                                                }
                                                count += 1;
                                            }

                                            if count != 2 {
                                                return None;
                                            }

                                            if frames_count < from {
                                                return None;
                                            }

                                            if to > 0 {
                                                if frames_count < to {
                                                    return None;
                                                }
                                            }
                                        }
                                        _ => {
                                            return None;
                                        }
                                    }
                                }
                                _got_frames = true;
                            }

                            if info.0 == "play" && info.1.is_string() {
                                match info.1.as_str() {
                                    Some(string) => {
                                        let file_name = format!("{}.mp3", string);
                                        audio_path.push(&file_name);

                                        if !audio_path.as_path().is_file() {
                                            return None;
                                        }

                                        audio_path.pop();
                                    }
                                    _ => {
                                        return None;
                                    }
                                }
                                _got_play = true;
                            }
                        }

                        if !_got_frames {
                            return None;
                        }
                        actions_path.pop();
                    }

                    match fs::read_dir(actions_path.as_path()) {
                        Ok(actions) => {
                            let size = actions.count();
                            if size != actions_count {
                                return None;
                            }
                        }
                        _ => {
                            return None;
                        }
                    };

                    pb.inc(30);
                    return Some(value);
                }
                _ => {
                    return None;
                }
            }
        }
        _ => {
            return None;
        }
    };
}

fn compress(
    zip: &mut ZipWriter<fs::File>,
    append: &mut String,
    src_dir: &mut String,
    pb: &ProgressBar,
) -> bool {
    match fs::read_dir(&src_dir) {
        Ok(dir) => {
            for entry in dir {
                pb.set_position(pb.position());
                match entry {
                    Ok(dentry) => {
                        let path = dentry.path();
                        if path.is_dir() {
                            match path.file_name() {
                                Some(name) => match name.to_str() {
                                    Some(name_str) => {
                                        let mut name_str_slash = String::new();
                                        name_str_slash.push_str(name_str);
                                        name_str_slash.push_str("/");

                                        let mut new_root = src_dir.clone();
                                        let mut append_new = append.clone();
                                        append_new.push_str(&name_str_slash);
                                        new_root.push_str(&name_str_slash);
                                        let mut entry_dir_name = append.clone();
                                        entry_dir_name.push_str(&name_str);

                                        let options = FileOptions::default()
                                            .compression_method(CompressionMethod::Stored);

                                        match zip.add_directory(&entry_dir_name, options) {
                                            Err(_) => {
                                                return false;
                                            }
                                            _ => {}
                                        }

                                        if !compress(zip, &mut append_new, &mut new_root, pb) {
                                            return false;
                                        }
                                    }
                                    _ => {
                                        return false;
                                    }
                                },
                                _ => {
                                    return false;
                                }
                            }
                        } else {
                            match path.file_name() {
                                Some(name) => match name.to_str() {
                                    Some(name_str) => {
                                        let mut entry_file_name = append.clone();
                                        entry_file_name.push_str(name_str);

                                        let mut file_source = src_dir.clone();
                                        file_source.push_str(name_str);

                                        let options = FileOptions::default()
                                            .compression_method(CompressionMethod::Stored);

                                        match zip.start_file(&entry_file_name, options) {
                                            Err(_) => {
                                                return false;
                                            }
                                            _ => {}
                                        }
                                        match fs::File::open(&file_source) {
                                            Ok(f) => {
                                                let mut contents = vec![];
                                                let mut buf_reader = BufReader::new(f);
                                                match buf_reader.read_to_end(&mut contents) {
                                                    Err(_) => {
                                                        return false;
                                                    }
                                                    _ => {}
                                                }
                                                match zip.write(&contents) {
                                                    Err(_) => {
                                                        return false;
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            Err(_) => {
                                                return false;
                                            }
                                        }
                                    }
                                    _ => {
                                        return false;
                                    }
                                },
                                _ => {
                                    return false;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        return false;
                    }
                }
            }
            true
        }
        Err(_) => false,
    }
}

fn make_progress(level: u64, msg: &'static str) -> ProgressBar {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green/bold} {msg:.bold}▕{wide_bar}| {pos:>7}/{len:7}")
            .progress_chars("█▓▒░  "),
    );
    pb.set_message(msg);
    pb.inc(level);
    pb
}

fn task_completed(total: u64, current: u64, task: &String, pb: &ProgressBar) {
    pb.finish_and_clear();

    let task_status = format!("[{}/{}]", current, total);

    println!("{} {}✔️", task_status.bold(), task);
}

pub fn run(source_dir: &str) {
    banner::header();

    let spirit_name: String;
    let mut progress: u64 = 5;

    // Validate Directory Structure.
    let mut pb = make_progress(progress, "⭕ Checking");
    pb.set_position(progress);

    match get_directory_name(source_dir) {
        Some(name) => {
            progress = 10;
            pb.set_position(progress);

            spirit_name = String::from(name);

            let task = format!("🧻Ritual of {} ", spirit_name);
            task_completed(4, 1, &task, &pb);
        }
        _ => {
            pb.finish_and_clear();
            println!("{}", "🛑 Directory Check Failed.\n");
            //banner::footer();
            process::exit(-1);
        }
    }

    pb = make_progress(progress, "🧵Valdating");
    pb.set_position(progress);

    if !validate_directory_structure(source_dir, &pb) {
        pb.finish_and_clear();
        println!("{}", "🛑 Validation Failed.\n");
        //banner::footer();
        process::exit(-1);
    }
    progress = 20;
    pb.set_position(progress);

    {
        let task = format!("🧱Passed Valdation   ");
        task_completed(4, 2, &task, &pb);
    }

    pb = make_progress(progress, "💈Parsing");
    pb.set_position(progress);

    match parse_directory(source_dir, &pb) {
        Some(value) => {
            let task = format!(
                "🧬Parsed {} ",
                match value["name"].as_str() {
                    Some(s) => s,
                    _ => "",
                }
            );
            task_completed(4, 3, &task, &pb);
        }
        _ => {
            pb.finish_and_clear();
            println!("{}", "🛑 Parsing Failed.\n");
            process::exit(-1);
        }
    }
    progress = 50;

    pb = make_progress(progress, "🗜️Compressing ");
    pb.set_position(progress);

    let filename = format!("{}.spirit", spirit_name);
    let output = Path::new(&filename);
    if output.exists() {
        pb.finish_and_clear();
        println!("{}", "🛑 Write Failed, File Already Exists.\n");
        process::exit(-1);
    }

    let file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&filename)
    {
        Ok(f) => f,
        Err(e) => {
            pb.finish_and_clear();
            println!("{}{}.", "🛑 Write Failed, ", e);
            process::exit(-1);
        }
    };

    progress = 80;
    pb.set_position(progress);

    let mut zip = ZipWriter::new(file);
    let mut append = String::new();
    let mut source_directory = String::from(source_dir);
    if !source_dir.ends_with("/") && !source_dir.ends_with("\\") {
        source_directory.push_str("/");
    }

    if !compress(&mut zip, &mut append, &mut source_directory, &pb) {
        pb.finish_and_clear();
        println!("{}", "🛑 Compression Failed.\n");
        process::exit(-1);
    }
    match zip.finish() {
        Err(_) => {
            pb.finish_and_clear();
            println!("{}", "🛑 Archive Finalization Failed.\n");
            process::exit(-1);
        }
        _ => {}
    }
    progress = 90;
    pb.set_position(progress);

    // For eye-candy, is this inefficient??
    // But I don't care xD
    for _ in 0..10 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(200));
    }

    let task = format!("🗜️Compressed Data ");
    task_completed(4, 4, &task, &pb);

    println!(
        "\n{}{}{}",
        "✅ Made ".bold(),
        spirit_name.bold(),
        ".spirit".bold()
    );
    banner::footer();
    process::exit(0);
}
