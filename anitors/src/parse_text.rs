use lazy_regex::regex_is_match;

use crate::tokens::{Token, Tokens};
use crate::Element;

/// 搜索影片发布组，取第一个括号内的内容作为发布组
pub(crate) fn parse_release_group(element: &mut Element, tokens: &Tokens) {
    let first = tokens.first_open_bracket();

    let start = tokens.find_next_unknown(&first);
    let end = tokens.find_next_bracket_or_identifier(&first);
    // 如果 start 或者 end 任意一个找不到则视为没有标签
    let tokens = tokens.sub_tokens(&start, &end);

    let text = build_text(tokens, true);
    if !text.is_empty() {
        element.release_group = Some(text);
    }
}

/// 搜索影片标题
pub(crate) fn parse_title(element: &mut Element, tokens: &Tokens) {
    // 此方法在 parse_release_group 之后调用，会跳过第一个括号
    let start = tokens.first_unknown();
    let end = tokens.find_next_bracket_or_identifier(&start);

    // token_end 处于 bracket_or_identifier 的位置进行拼接
    let text = match end.is_none() {
        // 如果 start 存在，end 不存在，则为整个名称都是 title
        true => build_text(tokens.sub_tokens_start(&start), false),
        false => build_text(tokens.sub_tokens(&start, &end), false),
    };

    match text.is_empty() {
        // 如果最终标题仍未找到，可能之前识别的 tag 就是标题
        true => element.anime_title = element.release_group.take(),
        false => element.anime_title = Some(text),
    }
}

/// 将 tokens 内的有效 token 拼装成 text
fn build_text(tokens: impl Iterator<Item = Token>, keep_delimiter: bool) -> String {
    let mut text = String::new();

    for mut token in tokens {
        if token.is_valid() {
            match (keep_delimiter, token.to_text()) {
                (false, token) if regex_is_match!("^[,&/]$", &token) => text += &token,
                (false, _) if token.is_delimiter() => text += " ",
                (_, token) => text += &token,
            }

            if token.is_unknown() {
                token.set_identifier();
            }
        }
    }

    if !keep_delimiter {
        static DASH: [char; 8] = [' ', '-', '‐', '‑', '‒', '–', '—', '―'];
        text = text.trim_matches(DASH.as_ref()).to_owned();
    }

    return text;
}
