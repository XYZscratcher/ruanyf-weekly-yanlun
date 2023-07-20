use glob::glob;
use regex::Regex;
//use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    //let map:HashMap<, _>=HashMap::new();
    let issue_re = Regex::new(r"^# 科技爱好者周刊(：)?(（)?第 (?<issue>[0-9]+) 期(）)?").unwrap();
    let yanlun = Regex::new(r"(?m)^##\s+言论((?:和|与)数(?:字|据))?(\r)?\n(\r)?\n").unwrap(); //I did it!!!
    let yanluns =
        Regex::new(r"(?m)(?:\d、\n\n)(?<y>(?:^[^-]{2}.+$\n\n){1,2})(?<a>^\-\-.+$)").unwrap();
    for path in glob("weekly/docs/issue-*.md").unwrap() {
        let path = path.unwrap();
        let c = read_to_string(&path).unwrap();
        let first_line = c.lines().next().unwrap();
        let is_weekly = first_line.contains("# 科技爱好者周刊");
        let has_yanlun = c.contains("言论");
        if is_weekly && has_yanlun {
            let full = yanlun.split(&c).collect::<Vec<_>>();
            //dbg!(&full);
            let y = full[1];
            let issue:i16 = issue_re
                .captures(first_line)
                .unwrap()
                .name("issue")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let matches = yanluns.captures_iter(y).map(|x| x.extract());
            if issue > 240 {
                println!("{path:?}\n\n\n");
                for (_, [y, a]) in matches {
                    println!("{}\n", y.trim_end().replace("\n\n", "\n"));
                    println!("—— {}\n\n", a.replace("--", "").trim_start());
                }
            }
        }
    }
}
