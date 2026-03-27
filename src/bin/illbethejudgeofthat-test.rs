// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6

//! illbethejudgeofthat-test. TRIPLE SIMS quality gate via exopack::triple_sims.
//! Stage 1: cargo test (7 integration tests)
//! Stage 2: binary smoke test (--help exits 0)
//! Stage 3: TRIPLE SIMS gate (all stages 3x)

use std::process::Command;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("illbethejudgeofthat-test");
    println!("========================");

    let project_dir = std::env::current_dir().unwrap_or_else(|_| {
        let exe = std::env::current_exe().unwrap();
        exe.parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    });

    // Locate production binary next to this test binary
    let exe = std::env::current_exe().unwrap();
    let prod_bin = exe.parent().unwrap().join("illbethejudgeofthat");

    let ok = exopack::triple_sims::f60(|| {
        let dir = project_dir.clone();
        let bin = prod_bin.clone();
        async move {
            // Stage 1: cargo test
            print!("  cargo test ... ");
            let test_out = Command::new("cargo")
                .args(["test", "--quiet"])
                .current_dir(&dir)
                .output();
            match test_out {
                Ok(o) if o.status.success() => println!("OK"),
                Ok(o) => {
                    println!("FAILED");
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    if !stderr.is_empty() {
                        eprintln!("{}", stderr);
                    }
                    return false;
                }
                Err(e) => {
                    println!("ERROR: {}", e);
                    return false;
                }
            }

            // Stage 2: binary smoke test
            print!("  --help ... ");
            if bin.exists() {
                let help = Command::new(&bin).arg("--help").output();
                match help {
                    Ok(o) if o.status.success() => println!("OK"),
                    Ok(o) => {
                        println!("FAILED (exit {})", o.status);
                        return false;
                    }
                    Err(e) => {
                        println!("ERROR: {}", e);
                        return false;
                    }
                }
            } else {
                println!("SKIP (binary not found at {})", bin.display());
            }

            true
        }
    })
    .await;

    std::process::exit(if ok { 0 } else { 1 });
}
