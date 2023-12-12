use lazy_regex::{regex, regex_is_match, Regex};

use crate::tokens::{Token, Tokens};

/// 将创建的 token 切分
pub(crate) fn tokenizer(tokens: &mut Tokens) {
    split_brackets(tokens);
    for token in tokens.unknown_tokens().collect::<Vec<_>>() {
        tokenize_year(tokens, &token);
        tokenize_tv_number(tokens, &token);
        remove_size(tokens, &token);
        remove_date(tokens, &token);
        remove_invalid_tag(tokens, &token);
        remove_ignore_type(tokens, &token);
    }
    split_delimiter(tokens);
    fix_split(tokens);
}

/// 按括号将 token 分割，并区分 token 是否在括号内
fn split_brackets(tokens: &mut Tokens) {
    for token in tokens.unknown_tokens().collect::<Vec<_>>() {
        let mut result = Vec::new();
        let mut next = token.to_text();
        let mut enclosed = false;

        while next.is_not_empty() {
            let bracket_regex = match enclosed {
                true => regex!(r"[)\]}」』】）》〉]"),
                false => regex!(r"[(\[{「『【（《〈]"),
            };
            let (left, sp, right) = next
                .split_once(bracket_regex)
                .unwrap_or_else(|| (next.clone(), Default::default(), Default::default()));
            if left.is_not_empty() {
                result.push(Token::unknown(left, enclosed));
            }
            if sp.is_not_empty() {
                result.push(match enclosed {
                    true => Token::bracket_closed(sp.clone(), true),
                    false => Token::bracket_open(sp.clone(), true),
                });
                enclosed = !enclosed;
            }
            next = right;
        }
        if next.is_not_empty() {
            result.push(Token::unknown(next, enclosed));
        }

        tokens.replace(&token, result);
    }
}

/// 切分剩余所有的分隔符
fn split_delimiter(tokens: &mut Tokens) {
    for token in tokens.unknown_tokens().collect::<Vec<_>>() {
        let mut result = Vec::new();
        let mut next = token.to_text();
        let enclosed = token.enclosed();

        while next.is_not_empty() {
            let re = regex!(
                r"\.|\s+-\s+|\s+|\+|/|～|;|&|\||#|_|~|\(|\)|\[|]|\{|}|「|」|『|』|【|】|（|）"
            );
            let (left, sp, right) = next
                .split_once(re)
                .unwrap_or_else(|| (next.clone(), Default::default(), Default::default()));
            if left.is_not_empty() {
                result.push(Token::unknown(left, enclosed));
            }
            if sp.is_not_empty() {
                result.push(Token::delimiter(sp, enclosed));
            }
            next = right;
        }
        if next.is_not_empty() {
            result.push(Token::unknown(next, enclosed));
        }

        tokens.replace(&token, result);
    }
}

/// 将年份缩减为前一个
fn tokenize_year(tokens: &mut Tokens, token: &Token) {
    tokenize_by_pat(tokens, token, regex!(r"([\s.]+\d{4})-\d{4}"));
}

/// 将 TV xxx 缩减为 xxx
fn tokenize_tv_number(tokens: &mut Tokens, token: &Token) {
    tokenize_by_pat(tokens, token, regex!(r"TV\s+(\d{1,4}([-~&+]\d{1,4})?)"));
}

/// 删除 xx番剧漫
fn remove_invalid_tag(tokens: &mut Tokens, token: &Token) {
    if regex_is_match!("新番|月?番|[日美国][漫剧]", &token.to_text()) {
        remove_by_pat(tokens, token, regex!(".*月新?番.?|.*[日美国][漫剧]"));
    }
}

/// 删除分类
fn remove_ignore_type(tokens: &mut Tokens, token: &Token) {
    let regex = regex!(
        "(?i)[动漫画纪录片电影视连续剧集日美韩中港台海外亚洲华语大陆综艺原盘高清\
        動畫紀錄電視連續劇韓臺亞華語陸綜藝盤]{2,}|Animations?|Documentar|Anime"
    );
    remove_by_pat(tokens, token, regex);
}

/// 删除文件大小
fn remove_size(tokens: &mut Tokens, token: &Token) {
    remove_by_pat(tokens, token, regex!(r"(?i)\d+(\.\d+)?\s*[MGT]i?B"));
}

/// 删除年月日，e.g. 2000-2-2
fn remove_date(tokens: &mut Tokens, token: &Token) {
    remove_by_pat(tokens, token, regex!(r"\d{4}[\s._-]\d{1,2}[\s._-]\d{1,2}"));
}

fn tokenize_by_pat(tokens: &mut Tokens, token: &Token, pat: &Regex) {
    let enclosed = token.enclosed();
    let text = token.to_text();
    if let Some((pre, sp, next)) = text.split_once(pat) {
        let sp = pat.captures(&sp).unwrap().get(1).unwrap();
        let pre = Token::unknown(pre, enclosed);
        let sp = Token::unknown(sp.as_str().into(), enclosed);
        let next = Token::unknown(next, enclosed);
        tokens.replace(token, [pre, sp, next])
    }
}

fn remove_by_pat(tokens: &mut Tokens, token: &Token, pat: &Regex) {
    let enclosed = token.enclosed();
    let text = token.to_text();
    if let Some((pre, _, next)) = text.split_once(pat) {
        let mut vec = Vec::with_capacity(2);
        if pre.is_not_empty() {
            vec.push(Token::unknown(pre, enclosed));
        }
        if next.is_not_empty() {
            vec.push(Token::unknown(next, enclosed));
        }
        tokens.replace(token, vec)
    }
}

type Node<'a> = (&'a mut Tokens, &'a mut Token);

/// 修正过度切分的 token
fn fix_split(tokens: &mut Tokens) {
    for mut token in tokens.all_tokens().collect::<Vec<_>>() {
        [(&mut *tokens, &mut token)]
            .into_iter()
            .filter_map(fix_audio_language)
            .filter_map(fix_point_num)
            .filter_map(fix_episode)
            .filter_map(fix_chinese_episode)
            .count();
    }
}

// ============================= 拆分修正 =============================

/// e.g. DUAL AUDIO, MULTI AUDIO
fn fix_audio_language((tokens, token): Node) -> Option<Node> {
    if regex_is_match!("^(?i)AUDIO$", &token.to_text()) {
        let sp = tokens.find_prev_valid(token);
        let prev = tokens.find_prev_valid(&sp);
        let word = prev.to_text();
        if sp.to_text() == " " && regex_is_match!("^(?i)DUAL|MULTI$", &word) {
            let mut new_token = prev.clone() + sp + token;
            new_token.set_unknown();
            tokens.replace(&prev, [new_token]);
            return None;
        }
    }
    return Some((tokens, token));
}

/// e.g. 2.0CH, 5.1, 5.1CH, DTS5.1, TRUEHD5.1
fn fix_point_num((tokens, token): Node) -> Option<Node> {
    if token.to_text() == "." {
        let prev = tokens.find_prev_valid(token);
        let next = tokens.find_next_valid(token);
        let pr_txt = prev.to_text();
        let ne_txt = next.to_text();

        if regex_is_match!("[\\dHhXx]$", &pr_txt) && regex_is_match!("(?i)^\\d[A-Z]*$", &ne_txt) {
            let replace_token = prev.clone();
            let mut new_token = prev + token + next;
            new_token.set_unknown();
            tokens.replace(&replace_token, [new_token]);
            return None;
        }
    }
    return Some((tokens, token));
}

/// e.g. "8 & 10", "01 of 24", "EP 90"
fn fix_episode<'a>((tokens, token): Node) -> Option<Node> {
    let text = token.to_text();
    // e.g. "8 & 10", "01 of 24", "01 + 02"
    if regex_is_match!(r"^(&|of|\+)$", &text) {
        let prev = tokens.find_prev_unknown(token);
        let next = tokens.find_next_unknown(token);
        if prev.to_text().is_ascii_digit() && next.to_text().is_ascii_digit() {
            let mut new_token = prev.clone() + token + next;
            new_token.set_unknown();
            tokens.replace(&prev, [new_token]);
            return None;
        }
    }
    // e.g. "EP 90", "#13"
    if regex_is_match!(
        r"(?i)^S(AISON|EASON)?$|^E(P(S|ISOD(E|ES|IO))?)$|^CAPITULO$|^FOLGE$|^#$|^VOL(\.|UME)?$",
        &text
    ) {
        let next = tokens.find_next_unknown(token);
        if next.to_text().is_ascii_digit() {
            let mut new_token = token.clone() + next;
            new_token.set_unknown();
            tokens.replace(&token, [new_token]);
            return None;
        }
    }
    return Some((tokens, token));
}

/// e.g. "第 四 集"
fn fix_chinese_episode((tokens, token): Node) -> Option<Node> {
    let text = token.to_text();
    if regex_is_match!("^[0-9一二三四五六七八九十百千零]+$", &text) {
        let mut replace_token = token.clone();
        let mut new_token = token.clone();
        let next = tokens.find_next_unknown(token);
        if regex_is_match!("^[集话話期季]全?$", &next.to_text()) {
            new_token = new_token + next;
        }
        let prev = tokens.find_prev_unknown(token);
        if regex_is_match!("^[第全共]$", &prev.to_text()) {
            replace_token = prev.clone();
            new_token = prev + new_token;
        }
        if replace_token != new_token {
            new_token.set_unknown();
            tokens.replace(&replace_token, [new_token]);
            return None;
        }
    }
    return Some((tokens, token));
}
