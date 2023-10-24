use serde::Serialize;
use std::collections::BTreeMap;
use std::os::unix::fs::MetadataExt;
use walkdir::WalkDir;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

const COMPR_THRESHOLD : u64 = 128;

#[derive(Serialize)]
struct FileAttrs {
    typeflag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_compr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    xattrs: Vec<(String, String)>,
    st_ino: u64,
    st_mode: u32,
    st_nlink: u64,
    st_uid: u32,
    st_gid: u32,
    st_size: u64,
    st_blocks: u64,
    st_atim_sec: i64,
    st_atim_nsec: i64,
    st_mtim_sec: i64,
    st_mtim_nsec: i64,
    st_ctim_sec: i64,
    st_ctim_nsec: i64,
}

fn push_iter(prev : char, count : i32, out : &mut String) -> () {
    let fmt = format!("{}/'{}'", count, prev);

    out.push_str(&fmt);
}

fn push_str(full_str : &String, out : &mut String) -> () {
    if full_str.len() == 0 {
        return;
    }

    let fmt = format!("#{}'{}'", full_str.len(), full_str);
    out.push_str(&fmt);
}

fn compr_str(path : &Path) -> String {
    let mut prev : char = '\0';
    let mut count : i32 = 0;
    let mut full_str = String::new();

    let mut out = String::new();
    let mut next_raw : [u8;1] = [0];

    let mut reader = BufReader::new(File::open(path).expect("Could not open file!"));

    loop {
        if reader.read(&mut next_raw).expect("Failed to run from file!") != 1 {
	    break;
        }
        let next = next_raw[0] as char;

        if next == prev {
            if full_str.is_empty() {
                count += 1;
            } else {
                full_str.pop();
                push_str(&full_str, &mut out);
                full_str.clear();
                count = 2;
            }
        } else {
            if count != 0 {
                push_iter(prev, count, &mut out);
                count = 0;
            }

            full_str.push(next);
        }

        prev = next;
    }

    if count != 0 {
        push_iter(prev, count, &mut out);
    } else {
        push_str(&full_str, &mut out);
    }

    return out;
}

fn read_file(path : &Path) -> String {
    let mut reader = BufReader::new(File::open(path).expect("Could not open file!"));
    let mut contents_str = String::new();
    let mut bfr : [u8;1] = [0];

    loop {
        if reader.read(&mut bfr).expect("Failed to run from file!") != 1 {
            break;
        }

        contents_str.push(bfr[0] as char);
    }

    contents_str
}

fn list_xattrs(path : &Path) -> Vec<(String, String)> {
    let mut xattrs : Vec<(String, String)> = vec![];

    for attr in xattr::list(&path).unwrap() {
        let mut val_str : String = "xattr could not be extracted!".to_string();

        let val : Option<Vec<u8>> = xattr::get(&path, &attr)
            .ok().flatten();

        if let Some(val_ex) = val {
            val_str = val_ex.iter().map(|x| *x as char).collect();
        }

        let name_str = attr.into_string().expect("xattr name could not be converted!");

        xattrs.push((name_str, val_str));
    }

    return xattrs;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (path, dump_contents) = match args.len() {
        2 => (&args[1], false),
        3 if args[1] == "--contents" => (&args[2], true),
        _ => {
            println!("usage: {} [--contents] <path>", args[0]);
            std::process::exit(1);
        }
    };
    let mut result = BTreeMap::new();
    for entry in WalkDir::new(path) {
        let entry = entry.expect("could not read dir entry");
        let metadata = entry.metadata().expect("could not retrieve file metadata");
        let mut contents : Option<String> = None;
        let mut contents_compr : Option<String> = None;

        if dump_contents && entry.file_type().is_file() {
            if metadata.size() >= COMPR_THRESHOLD {
                let compr_contents_str = compr_str(entry.path());
                if compr_contents_str.len() < (metadata.size() as usize) {
                    contents_compr = Some(compr_contents_str);
                }
            }

            if contents_compr.is_none() {
                contents = Some(read_file(entry.path()));
            }

        }

        let xattr = list_xattrs(entry.path());

        result.insert(
            entry.path().to_string_lossy().into_owned(),
            FileAttrs {
                typeflag: match entry.file_type() {
                    t if t.is_file() => "F",
                    t if t.is_dir() => "D",
                    t if t.is_symlink() => "SL",
                    _ => panic!("unexpected file type at {}", entry.path().display())
                }.to_string(),
                content: contents,
                content_compr: contents_compr,
                target: if dump_contents && entry.file_type().is_symlink() {
                    Some(std::fs::read_link(entry.path()).expect("could not read symlink").to_string_lossy().into_owned())
                } else {
                    None}
                    ,
                xattrs: xattr,
                st_ino: metadata.ino(),
                st_mode: metadata.mode(),
                st_nlink: metadata.nlink(),
                st_uid: metadata.uid(),
                st_gid: metadata.gid(),
                st_size: metadata.size(),
                st_blocks: metadata.blocks(),
                st_atim_sec: metadata.atime(),
                st_atim_nsec: metadata.atime_nsec(),
                st_mtim_sec: metadata.mtime(),
                st_mtim_nsec: metadata.mtime_nsec(),
                st_ctim_sec: metadata.ctime(),
                st_ctim_nsec: metadata.ctime_nsec(),
            },
        );
    }
    serde_json::to_writer_pretty(std::io::stdout(), &result).expect("could not serialize JSON");
}
