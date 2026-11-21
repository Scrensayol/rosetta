use std::{
    io::{self, Read},
    os::unix::process::ExitStatusExt,
    process::{Child, Command, ExitStatus},
    time::{Duration, Instant},
};

pub trait Decompiler: Sync {
    fn name(&self) -> &str;
    // the source, and the CPU time the decompiler used (its own and its children's)
    fn decompile(&self, bytecode: &[u8]) -> (Result<String, String>, Duration);
}

// Waits up to limit for the child, reaping it with wait4 so its CPU time comes back with its
// status. None on a timeout, with the child still running
fn wait(child: &Child, limit: Duration) -> io::Result<Option<(ExitStatus, Duration)>> {
    let pid = child.id() as libc::pid_t;
    let started = Instant::now();
    let mut nap = Duration::from_micros(20);
    loop {
        let mut status = 0;
        // SAFETY: rusage is plain data, and wait4 only writes to the two pointers
        let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
        let reaped = unsafe { libc::wait4(pid, &mut status, libc::WNOHANG, &mut usage) };
        if reaped == pid {
            let time = |t: libc::timeval| Duration::new(t.tv_sec as u64, t.tv_usec as u32 * 1000);
            return Ok(Some((ExitStatus::from_raw(status), time(usage.ru_utime) + time(usage.ru_stime))));
        }
        if reaped < 0 {
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(error);
        }
        if started.elapsed() >= limit {
            return Ok(None);
        }
        std::thread::sleep(nap);
        nap = (nap * 2).min(Duration::from_millis(1));
    }
}

// Every decompiler runs the same way, as a command: `name=program args...` where `{}` is
// replaced by the path of a raw (unencoded) bytecode file, appended when absent. The source is
// read from stdout. A clean non-zero exit is a refusal: the decompiler said it can't do this
// one, which is no wrong output. A crash (killed by a signal, or a Rust panic's exit status
// 101) or a run past 30 seconds is a failure.
pub struct External {
    name: String,
    argv: Vec<String>,
}

impl External {
    pub fn parse(spec: &str) -> Result<External, String> {
        let (name, command) = spec.split_once('=').ok_or("expected name=command")?;
        let mut argv = command.split_whitespace().map(str::to_string).collect::<Vec<_>>();
        if argv.is_empty() {
            return Err("empty command".into());
        }
        if !argv.iter().any(|a| a.contains("{}")) {
            argv.push("{}".into());
        }
        Ok(External { name: name.to_string(), argv })
    }
}

impl Decompiler for External {
    fn name(&self) -> &str {
        &self.name
    }

    fn decompile(&self, bytecode: &[u8]) -> (Result<String, String>, Duration) {
        match self.run(bytecode) {
            Ok(done) => done,
            Err(e) => (Err(e), Duration::ZERO),
        }
    }
}

impl External {
    fn run(&self, bytecode: &[u8]) -> Result<(Result<String, String>, Duration), String> {
        let file = tempfile::Builder::new().suffix(".luauc").tempfile().map_err(|e| e.to_string())?;
        std::fs::write(file.path(), bytecode).map_err(|e| e.to_string())?;
        let path = file.path().to_string_lossy();
        let argv = self.argv.iter().map(|a| a.replace("{}", &path)).collect::<Vec<_>>();

        let mut child = Command::new(&argv[0])
            .args(&argv[1..])
            .env("RAYON_NUM_THREADS", "1")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("spawn {}: {e}", argv[0]))?;
        let mut stdout = child.stdout.take().unwrap();
        let reader = std::thread::spawn(move || {
            let mut text = Vec::new();
            let _ = stdout.read_to_end(&mut text);
            text
        });
        match wait(&child, Duration::from_secs(30)).map_err(|e| e.to_string())? {
            Some((status, cpu)) => {
                let text = String::from_utf8_lossy(&reader.join().unwrap()).into_owned();
                if status.success() {
                    return Ok((Ok(text), cpu));
                }
                let mut error = String::new();
                let _ = child.stderr.take().unwrap().read_to_string(&mut error);
                let why = error.lines().find(|l| !l.trim().is_empty()).unwrap_or_default().trim().to_string();
                let error = match status.code() {
                    Some(code) if code != 101 => format!("unsupported (exit {code}): {why}"),
                    _ => format!("crashed ({status}): {why}"),
                };
                Ok((Err(error), cpu))
            }
            None => {
                let _ = child.kill();
                let _ = child.wait();
                Ok((Err("timeout".into()), Duration::from_secs(30)))
            }
        }
    }
}
