use std::process::Command;
use std::io::BufReader;
use tar::Archive;
use std::fs;
use zstd::stream::read::Decoder;
use tempfile::tempdir;
use crate::state;
use std::path::PathBuf; // needed for join in remove

fn err(msg: &str) -> impl Fn(std::io::Error) -> () + '_ {
    move |e| {
        eprintln!("{}: {}", msg, e);
        ()
    }
}

pub fn install(archive: &str) -> Result<(), ()> {
    let name = archive.trim_end_matches(".tar.zst").to_string();

    // check if already installed
    if let Ok(contents) = std::fs::read_to_string("/var/lib/vlpkg/installed.log") {
        if contents.lines().any(|line| line == name) {
            println!("{} is already installed.", &name);
            return Ok(());
        }
    }

    // setup temp stuff
    let tempdir = tempdir().map_err(err("Failed to create tempdir"))?;
    let extract_dir = tempdir.path().join(&name);
    println!("Installing {}", &name);
    fs::create_dir_all(&extract_dir)
        .map_err(err("Failed to create extract dir"))?;

    // open and extract archive
    println!("[*] Unpacking package..");
    let file = fs::File::open(archive)
        .map_err(err("Failed to open archive"))?;
    let decoder = Decoder::new(BufReader::new(file))
        .map_err(err("Failed to create zstd decoder"))?;
    let mut archive = Archive::new(decoder);

    archive.unpack(&extract_dir)
        .map_err(err("Failed to unpack archive"))?;

    // run install script
    println!("[*] Running install script..\n");
    let output = Command::new("sh")
        .arg(extract_dir.join("install.sh"))
        .current_dir(&extract_dir)
        .output()
        .map_err(err("Failed to run install.sh"))?;

    if !output.status.success() {
        eprintln!(
            "install.sh failed (exit {:?})\nstderr:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(());
    }
    println!("{}", String::from_utf8_lossy(&output.stdout));

    // copy removal script
    println!("[*] Copying neccessary files..");
    fs::create_dir_all(format!("/var/lib/vlpkg/remove/{}", name))
        .map_err(err("Failed to create remove dir"))?;
    fs::copy(
        extract_dir.join("remove.sh"),
        format!("/var/lib/vlpkg/remove/{}/remove.sh", name),
    )
    .map_err(err("Failed to copy remove.sh"))?;

    // done
    println!("{} installed!", &name);
    state::log_install(&name).map_err(err("Failed to log install"))?;
    Ok(())
}

pub fn remove(name: &str) -> Result<(), ()> {
    // check if not installed
    let installed = std::fs::read_to_string("/var/lib/vlpkg/installed.log")
        .unwrap_or_default();
    if !installed.lines().any(|line| line == name) {
        println!("{} is not installed.", name);
        return Ok(());
    }

    let remove_script: PathBuf = PathBuf::from("/var/lib/vlpkg/remove").join(name).join("remove.sh");

    // run remove script
    println!("[*] Running remove script..");
    let output = Command::new("sh")
        .arg(remove_script)
        .output()
        .map_err(err("Failed to run remove.sh"))?;

    if !output.status.success() {
        eprintln!(
            "remove.sh failed (exit {:?})\nstderr:\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(());
    }
    println!("{}", String::from_utf8_lossy(&output.stdout));

    state::remove_log(name).map_err(err("Failed to remove log"))?;
    Ok(())
}

pub fn init() -> Result<(), ()> {
    fs::create_dir_all("/var/lib/vlpkg")
        .map_err(err("Failed to create /var/lib/vlpkg"))?;

    fs::create_dir_all("/var/lib/vlpkg/remove")
        .map_err(err("Failed to create /var/lib/vlpkg/remove"))?;

    fs::File::create("/var/lib/vlpkg/installed.log")
        .map_err(err("Failed to create installed.log"))?;
    Ok(())
}

