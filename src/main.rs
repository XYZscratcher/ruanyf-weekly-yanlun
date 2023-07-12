use glob::glob;
use regex::Regex;
use std::fs::read_to_string;

fn main() {
    // TODO:拿到发布日期，通过封面图的URL
    let yanlun = Regex::new(r"(?m)^##\s+言论((?:和|与)数(?:字|据))?(\r)?\n(\r)?\n").unwrap();//I did it!!!
    let yanluns = Regex::new(r"(?m)(?:\d、\n\n)(?<y>^.+)").unwrap();
    for path in glob("weekly/docs/issue-*.md").unwrap() {
        let path = path.unwrap();
        let c = read_to_string(&path).unwrap();
        let first_line = c.lines().next().unwrap();
        let is_weekly = first_line.contains("# 科技爱好者周刊");
        let has_yanlun=c.contains("言论");
        if is_weekly && has_yanlun{
            dbg!(path);
            let full = yanlun.split(&c).collect::<Vec<_>>();
            //dbg!(&full);
            let y=full[1];
            let matches = yanluns.captures_iter(y).map(|x| x.extract());
            for (_,[y]) in matches{
                dbg!(y);
            }
            //dbg!(matches);
            //let matches = yanlun.find_iter(&c);
        }
        //dbg!(is_weekly);
    }
}
