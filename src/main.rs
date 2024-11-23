use std::io::Read;

fn main() -> Result<(),String> {
    let args = std::env::args()
        .collect::<Vec<_>>();
    if args.len() > 1 {
        let (mode, path) = if args.len() == 2 {("-d", args[1].as_str())}
        else {(args[1].as_str(), args[2].as_str())};
        let strings = std::fs::read_to_string(path).map_err(|err|err.to_string())?;
        process(mode, &strings, path)?;
    } else {
        let mode = args.get(1).map_or("-d", |v| v);
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).map_err(|err|err.to_string())?;
        process(mode,&buffer,"")?;
    }
    Ok(())
}
fn process(mode:&str, text:&str, path:&str) -> Result<(), String> {
    match mode {
        "-c" => {
            let size =  text.len();
            println!("{} {}", size,path)
        },
        "-l" => {
            println!("{} {}", text.lines().count(),path)
        },
        "-w" => {
            let mut count = 0;
            for line in text.lines() {
                count += line.split_whitespace().count();
            }

            println!("{} {}", count,path)
        },
        "-m" => {
            let chars = text
                .chars().count();
            println!("{} {}", chars,path)
        },
        "-d" => {
            let lines = text.lines().count();
            let mut words = 0;
            for line in text.lines() {
                words += line.split_whitespace().count();
            }
            let chars = text.len();
            println!("{lines} {words} {chars} {path}");
        }
        _ => ()
    }
    Ok(())
}


