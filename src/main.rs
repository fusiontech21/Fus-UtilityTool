use colored::Colorize;
use std::process::Command;

const VERSION: &str = "0.2.0";

// TODO  add autoremove command : LINE 132
// TODO add a help command that lists commands and names their Functions

fn run(cmd: &str, args: &[&str]) {
    let status = Command::new(cmd).args(args).status().unwrap_or_else(|_| {
        eprintln!("{}", format!("Failed to execute {}", cmd).red());
        std::process::exit(1);
    });

    if status.success() {
        println!("{}", "Finished with no errors hooray!".green());
    }
}

fn require_pkg(pkg: Option<&String>) -> &str {
    pkg.map(|s| s.as_str()).unwrap_or_else(|| {
        eprintln!("{}", "Missing package name".red());
        std::process::exit(1);
    })
}

// UPDATE SYSTEM DONT EDIT THIS (unless you can make it better)
fn checkupdate() {
    let url = "https://api.github.com/repos/fusiontech21/Fus-UtilityTool/releases/latest";

    let clint = reqwest::blocking::Client::builder()
        .user_agent("fusi")
        .build()
        .unwrap();

    if let Ok(resp) = clint.get(url).send() {
        if let Ok(txt) = resp.text() {
            if let Some(tg) = txt.split("\"tag_name\":\"").nth(1) {
                let latst = tg.split('"').next().unwrap_or("");
                if !latst.is_empty() && latst != VERSION {
                    println!(); // idk how many times i apologized BUT AGAIN SORRY FOR THE FORMATTING!! 
                    // WHOEVER READS THIS PLEASE FIX THE FORMATTING!!!!!!
                    println!(
                        "{}",
                        "╔══════════════════════════════════════════╗".yellow()
                    );
                    println!(
                        "{}",
                        format!("║  ⚠ New Version Available: {}  ║", latst)
                            .yellow()
                            .bold()
                    );
                    println!(
                        "{}",
                        "║  RUN 'fusi self-update' to update        ║".red().bold()
                    );
                    println!(
                        "{}",
                        "╚══════════════════════════════════════════╝".yellow()
                    );
                    println!();
                }
            }
        }
    }
}

fn main() {
    colored::control::set_override(true);

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!(
            "{}",
            format!("No command provided. Use: fusi <command>").red()
        );
        std::process::exit(1);
    }

    let cmd = args[0].as_str();
    let pkg = args.get(1);

    match cmd {
        // fusi install <package>
        "install" => run("sudo", &["pacman", "-S", require_pkg(pkg)]),

        // fusi remove <package>
        "remove" => run("sudo", &["pacman", "-Rns", require_pkg(pkg)]),

        // FUN
        "secret" => {
            let txt = "You are secretly a Femboy";
            secrething(&txt);
        }

        //Updates Fusi
        "self-update" => {
            run(
                // NO NOT AGAIN THE FORMATTING HOLY SHITTT ITS EVEN FORMATTING THE COMMENT REALLY WEIRD
                "bash",
                &[
                    "-c",
                    "curl -s https://raw.githubusercontent.com/fusiontech21/Fus-UtilityTool/main/setup/install.sh | bash",
                ],
            );
        }

        // searches for a package
        "search" => run("pacman", &["-Ss", require_pkg(pkg)]),

        // Updates the system
        "update" => run("sudo", &["pacman", "-Syu"]),

        // does a light clean up unlike the remove command
        "softremove" => run("sudo", &["pacman", "-R", require_pkg(pkg)]),

        // Gives you info abt a specific package
        "info" => run("pacman", &["-Si", require_pkg(pkg)]),

        // lists installed packages (not including deps)
        "list" => run("pacman", &["-Qe"]),

        // lists all installed packages including deps
        "listall" => run("pacman", &["-Q"]),

        // Checks if a specific pkg is installed
        "installed" => run("pacman", &["-Qs", require_pkg(pkg)]),

        // fusi details
        "details" => {
            // HOLY SHIT IM SO SORRY FOR THE FORMATING WHEN I SAVE IT IT DOES THIS
            println!(
                "{}",
                r#"
            ███████╗██╗   ██╗███████╗██╗
            ██╔════╝██║   ██║██╔════╝██║
            █████╗  ██║   ██║███████╗██║
            ██╔══╝  ██║   ██║╚════██║██║
            ██║     ╚██████╔╝███████║██║                
            ╚═╝      ╚═════╝ ╚══════╝╚═╝
                "#
                .cyan()
                .bold()
            );
            println!(
                "{}",
                "A Tool to help beginners use the Terminal for Arch-based distros".white()
            );
            println!("{}", "Version: 0.1.0".white());
            println!("{}", "© 2025 fusiontech21 — AGPL-3.0".white());
        }

        // upgrades a pkg
        "upgrade" => run("sudo", &["pacman", "-S", require_pkg(pkg)]),

        //downgrades a pkg
        "downgrade" => run("sudo", &["pacman", "-U", require_pkg(pkg)]),

        // removes uused deps
        // "autoremove" => run("sudo", &["pacman", "-Qdtg"]),

        // shows files owned by pkg
        "files" => run("pacman", &["-Ql", require_pkg(pkg)]),

        // shows which pagake owns a file
        "owner" => run("pacman", &["-Qo", require_pkg(pkg)]),

        // shows deps of pkg
        "deps" => run("pacman", &["-Si", require_pkg(pkg)]),

        // exact same as deps but fancier name
        "dependencies" => run("pacman", &["-Si", require_pkg(pkg)]),

        // shows install history
        "log" => run("cat", &["/var/log/pacman.log"]),

        // find,rank,update mirrorlist
        "mirrors" => run("sudo", &["reflector"]),

        // removes pacan lock file
        "unlock" => run("sudo", &["rm", "/var/lib/pacman/db.lck"]),

        // shows amount of pkgs installed
        "stats" => run("pacman", &["-Qq"]),

        // Help command idk what to say
        "help" => {
            println!("{}", "Fusi - Available Commands".cyan().bold());
            println!("{}", "─────────────────────────────────────────".cyan());
            println!(
                "{} {}",
                "fusi install <pkg>".green().bold(),
                "→ Install a package"
            ); // AGAIN SORRY FOR THE FORMATING
            println!(
                "{} {}",
                "fusi remove <pkg>".green().bold(),
                "→ Remove a package (full cleanup)"
            );
            println!(
                "{} {}",
                "fusi softremove <pkg>".green().bold(),
                "→ Remove just the package"
            );
            println!(
                "{} {}",
                "fusi search <pkg>".green().bold(),
                "→ Search for a package"
            );
            println!(
                "{} {}",
                "fusi update".green().bold(),
                "→ Update the entire system"
            );
            println!(
                "{} {}",
                "fusi upgrade <pkg>".green().bold(),
                "→ Upgrade a specific package"
            );
            println!(
                "{} {}",
                "fusi downgrade <pkg>".green().bold(),
                "→ Downgrade a package"
            );
            println!(
                "{} {}",
                "fusi info <pkg>".green().bold(),
                "→ Show info about a package"
            );
            println!(
                "{} {}",
                "fusi installed <pkg>".green().bold(),
                "→ Check if a package is installed"
            );
            println!(
                "{} {}",
                "fusi list".green().bold(),
                "→ List explicitly installed packages"
            );
            println!(
                "{} {}",
                "fusi listall".green().bold(),
                "→ List all installed packages"
            );
            println!(
                "{} {}",
                "fusi files <pkg>".green().bold(),
                "→ Show files owned by a package"
            );
            println!(
                "{} {}",
                "fusi owner <file>".green().bold(),
                "→ Show which package owns a file"
            );
            println!(
                "{} {}",
                "fusi deps <pkg>".green().bold(),
                "→ Show dependencies of a package"
            );
            println!(
                "{} {}",
                "fusi stats".green().bold(),
                "→ Show how many packages are installed"
            );
            println!(
                "{} {}",
                "fusi log".green().bold(),
                "→ Show pacman install history"
            );
            println!(
                "{} {}",
                "fusi mirrors".green().bold(),
                "→ Lists your mirrors"
            );
            println!(
                "{} {}",
                "fusi unlock".green().bold(),
                "→ Remove pacman lock file"
            );
            println!(
                "{} {}",
                "fusi details".green().bold(),
                "→ Show info about fusi"
            );
            println!("{}", "─────────────────────────────────────────".cyan());
            println!("{}", "© 2025 fusiontech21 — AGPL-3.0".white());
        }

        // anything else
        _ => {
            println!(
                "{}",
                format!(
                    "Unknown Command ({}) Type fusi help to list all Commands",
                    cmd
                )
                .yellow() // SORRY FOR THE TRASH FORMATTING WHEN I SAVE IT AUTO DOES THIS
            );
        }
    }

    checkupdate();

    std::process::exit(0);
}

fn secrething(txt: &str) {
    let colors = ["rd", "ylw", "grn", "cyn", "blue", "mgnt"];
    for (i, ch) in txt.chars().enumerate() {
        let clrs = match colors[i % colors.len()] {
            "rd" => ch.to_string().red().bold(),
            "ylw" => ch.to_string().yellow().bold(),
            "grn" => ch.to_string().green().bold(),
            "cyn" => ch.to_string().cyan().bold(),
            "blue" => ch.to_string().blue().bold(),
            "mgnt" => ch.to_string().magenta().bold(),
            _ => ch.to_string().white().bold(),
        };
        print!("{}", clrs);
    }
    println!();
}
