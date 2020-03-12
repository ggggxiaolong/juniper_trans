use serde_json::Value;
use regex::Regex;

lazy_static! {
    static ref RE: Regex= Regex::new(r"(?P<i>\d)a{6}(?P<s>s|d)").unwrap();
    static ref RE2: Regex= Regex::new(r"%(?P<i>\d)\$(?P<s>s|d)").unwrap();
    static ref RE3: Regex= Regex::new(r"&#096;").unwrap();
    static ref RE4: Regex= Regex::new("\n").unwrap();
    static ref RE5: Regex= Regex::new("mm200gg").unwrap();
}

fn beforeTrans(source: &str) -> String {
    let after = RE2.replace_all(source, "${i}aaaaaa${s}").into_owned();
    let after = RE3.replace_all(&after, "'").into_owned();
    let after = RE4.replace_all(&after, "mm200gg").into_owned();
    return after
}

fn afterTrans(source: &str) -> String {
    let after = RE4.replace_all(source, "").into_owned();
    let after = RE.replace_all(&after, "%${i}$$${s}").into_owned();
    let after = RE5.replace_all(&after, "\n ");
    return after.into_owned()
}

fn compute_checksum(term: &str) -> (u32, u32) {
    let mut checksum: u32 = 429955;
    for &byte in term.as_bytes() {
        checksum = checksum.wrapping_add(byte as _);
        checksum = checksum.wrapping_add(checksum << 10);
        checksum ^= checksum >> 6;
    }
    checksum = checksum.wrapping_add(checksum << 3);
    checksum ^= checksum >> 11;
    checksum = checksum.wrapping_add(checksum << 15);
    checksum ^= 3864579582;
    checksum %= 1_000_000;
    (checksum, checksum ^ 429955)
}

fn url(term: &str, from: &str, to: &str) -> String {
    let checksum = compute_checksum(term);
    format!(
        "https://translate.google.cn/translate_a/single?client=webapp&sl={}&tl={}&hl=zh-CN&dt=at&dt=bd&dt=ex&dt=ld&dt=md&dt=qca&dt=rw&dt=rm&dt=ss&dt=t&otf=2&ssel=3&tsel=0&kc=6&tk={}.{}&q={}",
        from,
        to,
        checksum.0,
        checksum.1,
        term,
    )
}

async fn make_request(term: &str, from: &str, to: &str) -> Option<String> {
    let api_url = url(term, from, to);
    println!("{}", api_url);
    let res = reqwest::Client::new().get(&api_url)
        .header("User-Agent"," Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2743.116 Safari/537.36")
        .header("Content-Type","application/json; charset=UTF-8")
        .header("Host","translate.google.cn")
        .send().await;

    match res {
        Ok(e) => match e.text().await {
            Ok(i) => Some(i),
            _ => None,
        },
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}


pub async fn translate(text: &str, from: &str, to: &str) -> String {
    let body = make_request(&beforeTrans(text), from, to).await;
    let mut text = String::new();
    if let Some(json) = body {
        if let Ok(v) = serde_json::from_str::<Value>(&json) {
            if let Value::Array(mut root) = v {
                if root.len() > 1 {
                    let part1 = root.remove(0);
                    if let Value::Array(part2) = part1 {
                        for part3 in part2 {
                            if let Value::Array(mut part4) = part3 {
                                if part4.len() > 1 {
                                    if let Value::String(part5) = part4.remove(0){
                                        text += &part5;
                                    }
                                }
                            }
                        }
                    }   
                }
            }
        } else {
            println!("error");
        }
    };
    afterTrans(&text)
}