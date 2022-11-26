// Note: this code was developed in one day with "if it's not broken don't fix it" taken quite
// seriously. I apologize to any competent programmers possibly attempting to read this.
use std::{
    path::PathBuf,
    time::Duration,
    env,
    fs,
    process::Command,
};
use fastrand;

fn main() {
    println!("Hello, world!");
    let mut wp_path = PathBuf::new();
    wp_path.push(home());
    wp_path.push(".wallpaper");

    let mut argv = env::args();
    argv.next();
    let dur: u64 = match argv.next() {
        Some(v) => v.parse().unwrap_or(600),
        None => 600,
    };

    match fs::metadata(&wp_path) {
        Ok(_) => {},
        Err(_) => {
            fs::create_dir(&wp_path)
                .expect("FATAL ERROR: Couldn't make wallpaper dir at ~/.wallpaper");
            println!("Created wallpaper directory @ ~/.wallpaper. Please populate it.");
            return ()
        }
    }

    let wallpapers: Vec<String> = {
        let mut wv: Vec<String> = Vec::new();
        for f in fs::read_dir(&wp_path).expect("FATAL ERROR: Failed to read wallpaper directory!") {
            match f {
                Ok(de) => {
                    let foo = de.file_name().into_string().unwrap();
                    let bar = match foo.clone().split(".").last().unwrap() {
                        "jpg" => true,
                        "png" => true,
                        _ => false
                    };
                    if bar {
                        wv.push(foo);
                    }
                },
                Err(_) => {},
            }
        }
        wv
    };

    let o = Opts {
        backend: match &*env::var("XDG_SESSION_TYPE").expect("FATAL ERROR: No graphical session type.") {
            "x11" => {
                let feh_exists: bool = Command::new("which")
                    .arg("feh")
                    .status()
                    .unwrap()
                    .success();

                if feh_exists {
                    SupportedBackend::Feh
                } else {
                    panic!("FATAL ERROR: Could not find feh in an x11 context.")
                }
            },
            "wayland" => {
                let swaybg_exists: bool = Command::new("which")
                    .arg("swaybg")
                    .status()
                    .unwrap()
                    .success();

                if swaybg_exists {
                    SupportedBackend::Swaybg
                } else {
                    panic!("FATAL ERROR: Could not find swaybg in a wayland context.")
                }
            },
            _ => panic!("FATAL ERROR: Unsupported graphical session type.")
        },
        interval: Duration::from_secs(dur),
    };

    let set_wallpaper = match o.backend {
        SupportedBackend::Feh => |wallpapers: &Vec<String>, current_wp: &str| -> String {
            let mut new_wp: &str = wallpapers[fastrand::usize(0..wallpapers.len())].as_ref();
            while new_wp == current_wp {
                new_wp = wallpapers[fastrand::usize(..wallpapers.len())].as_ref();
            }
            let mut wp_path = PathBuf::new();
            wp_path.push(home());
            wp_path.push(".wallpaper");
            wp_path.push(new_wp);
            
            let wp_path = wp_path.to_str().unwrap();

            Command::new("feh")
                .args(["--bg-scale", wp_path])
                .spawn()
                .unwrap();
            new_wp.to_owned()
        },
        SupportedBackend::Swaybg => |wallpapers: &Vec<String>, current_wp: &str| -> String {
            let mut new_wp: &str = wallpapers[fastrand::usize(0..wallpapers.len())].as_ref();
            while new_wp == current_wp {
                new_wp = wallpapers[fastrand::usize(..wallpapers.len())].as_ref();
            }
            let mut wp_path = PathBuf::new();
            wp_path.push(home());
            wp_path.push(".wallpaper");
            wp_path.push(new_wp);
            
            let wp_path = wp_path.to_str().unwrap();
            Command::new("swaybg")
                .args(["-o", "*"])
                .args(["-m", "stretch"])
                .args(["-i", wp_path])
                .spawn()
                .unwrap();
            new_wp.to_owned()
        }
    };
    let mut cwp = set_wallpaper(&wallpapers, "");
    loop {
        std::thread::sleep(o.interval);
        cwp = set_wallpaper(&wallpapers, &*cwp);
    }
}

fn home() -> String {
    let rv = env::var("HOME")
        .expect("FATAL ERROR: Failed to read environmental variable 'HOME'");
    rv
}

enum SupportedBackend {
    Feh,
    Swaybg,
}

struct Opts {
    interval: Duration,
    backend: SupportedBackend,
}

