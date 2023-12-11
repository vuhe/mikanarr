use std::ops::Range;

use lazy_regex::{regex_captures, regex_is_match};

use crate::covert_number::AutoParseU16;
use crate::tokens::{Token, Tokens};
use crate::Element;

const EP_NUM_MAX: u16 = 1890;

/// 搜索影片 VOL
pub(crate) fn parse_volume(element: &mut Element, tokens: &Tokens) {
    for mut token in tokens.unknown_tokens() {
        if let Some((_, _, v)) = regex_captures!(r"^VOL(\.|UME)?(\d)$", &token.to_text()) {
            element.volume_number = Some(v.into());
            token.set_identifier();
            return;
        }
    }
}

/// 搜索影片年份
pub(crate) fn parse_year(element: &mut Element, tokens: &Tokens) {
    // 年份区间可以避开 2K(1440p) 和 4K(2160p)
    static YEAR_RANGE: Range<u16> = 1900..2150;

    // 查找第一个括号单数字, e.g. (2000)
    let unknown_tokens = tokens.unknown_tokens();
    let isolated_num = unknown_tokens
        .into_iter()
        .find(|it| it.to_text().is_ascii_digit() && is_token_isolated(tokens, it))
        .filter(|it| YEAR_RANGE.contains(&it.to_text().parse().unwrap_or(0)));

    let year = isolated_num.or_else(|| {
        // 未找到括号单数字，尝试直接使用没有括号的数字
        // 查找 从右到左 的第一个年份, e.g. Wonder Woman 1984 2020
        tokens
            .unknown_tokens()
            .into_iter()
            .filter(|it| it.to_text().is_ascii_digit())
            .filter(|it| YEAR_RANGE.contains(&it.to_text().parse().unwrap_or(0)))
            .rev()
            .next()
    });

    if let Some(mut year) = year {
        year.set_identifier();
        element.anime_year = Some(year.to_text().to_string());
    }
}

/// 剧集匹配，尽可能的查找 token 中的集数信息
pub(crate) fn parse_episode(element: &mut Element, tokens: &Tokens) {
    let unknown_tokens = tokens.unknown_tokens().into_iter();
    let mut num_tokens = unknown_tokens
        .filter(|it| it.to_text().has_number())
        .collect::<Vec<_>>();

    // 集季在一起
    for token in num_tokens.iter_mut() {
        match_season_and_episode(element, token);
    }
    if element.episode_number.is_some() {
        return;
    }

    // 集季分开
    for token in num_tokens.iter_mut() {
        match_single_season(element, token);
        match_multi_season(element, token);
        match_number_sign(element, token);
        match_single_episode(element, token);
    }
    if element.episode_number.is_some() {
        return;
    }

    // 不准确的集 regex 匹配
    for token in num_tokens.iter_mut() {
        match_multi_episode(element, token);
        match_fractional_episode(element, token);
        match_partial_episode(element, token);
    }
    if element.episode_number.is_some() {
        return;
    }

    // 仅使用纯数字继续尝试
    let mut num_tokens: Vec<Token> = num_tokens
        .into_iter()
        .filter(|it| it.to_text().is_ascii_digit())
        .collect();

    // 单括号较为准确
    for token in num_tokens.iter_mut() {
        match_isolated_num(element, tokens, token);
    }
    if element.episode_number.is_some() {
        return;
    }

    // 猜测匹配
    for token in num_tokens.iter_mut() {
        match_equivalent_num(element, tokens, token);
        match_separated_num(element, tokens, token);
    }
}

/// 单括号 token, e.g. (2000)
fn is_token_isolated(tokens: &Tokens, token: &Token) -> bool {
    token.enclosed()
        && tokens.find_prev_not_delimiter(token).is_open_bracket()
        && tokens.find_next_not_delimiter(token).is_closed_bracket()
}

/// 单集匹配 e.g. "01v2"
fn match_single_episode(element: &mut Element, token: &mut Token) {
    // e.g. "01v2"
    if let Some((_, ep, ver)) = regex_captures!(r"(?i)(\d{1,4})(V\d)", &token.to_text()) {
        element.episode_number = Some(format!("E{ep}"));
        element.release_version = Some(ver.into());
        token.set_identifier();
        return;
    }

    // e.g. EP21
    if let Some((_, _, _, _, _, e)) = regex_captures!(
        r"(?i)(E(P(S|ISOD(E|ES|IO))?)|CAPITULO|FOLGE)(\d{1,4})",
        &token.to_text()
    ) {
        element.episode_number = Some(format!("E{e}"));
        token.set_identifier();
        return;
    }

    // e.g. 01of24
    if let Some((_, e)) = regex_captures!(r"(?i)(\d{1,4})of\d{1,4}", &token.to_text()) {
        element.episode_number = Some(format!("E{e}"));
        token.set_identifier();
        return;
    }

    if !regex_is_match!("[全共].+[集话話期]|[集话話期]全", &token.to_text()) {
        if let Some((_, e)) = regex_captures!("第?(.+)[集话話期]", &token.to_text()) {
            token.set_identifier();
            if let Some(ep) = e.auto_parse_u16() {
                element.episode_number = Some(format!("E{ep}"));
            }
        }
    }
}

/// 多集匹配 e.g. "01-02", "03-05v2"
fn match_multi_episode(element: &mut Element, token: &mut Token) {
    if let Some((_, e1, v1, e2, v2)) = regex_captures!(
        r"(?i)(\d{1,4})(V\d)?[-~&+](\d{1,4})(V\d)?",
        &token.to_text()
    ) {
        element.episode_number = Some(format!("E{e1}-E{e2}"));
        if !v1.is_empty() {
            element.release_version = Some(v1.into());
        }
        if !v2.is_empty() {
            element.release_version = Some(v2.into());
        }
        token.set_identifier();
    }
}

/// 单季匹配 e.g. "SEASON 3"
fn match_single_season(element: &mut Element, token: &mut Token) {
    if let Some((_, _, s)) = regex_captures!(r"(?i)^S(AISON|EASON)?(\d{1,2})$", &token.to_text()) {
        element.anime_season = Some(format!("S{s}"));
        token.set_identifier();
        return;
    }

    if !regex_is_match!("[全共].+季|季全", &token.to_text()) {
        if let Some((_, s)) = regex_captures!("第?(.+)季", &token.to_text()) {
            token.set_identifier();
            if let Some(se) = s.auto_parse_u16() {
                element.anime_season = Some(format!("S{se}"));
            }
        }
    }
}

/// 多季匹配 e.g. S01-S02
fn match_multi_season(element: &mut Element, token: &mut Token) {
    if let Some((_, _, s1, _, s2)) = regex_captures!(
        r"(?i)S(AISON|EASON)?(\d{1,2})[-~&+]S(AISON|EASON)?(\d{1,2})",
        &token.to_text()
    ) {
        element.anime_season = Some(format!("S{s1}-S{s2}"));
        token.set_identifier();
    }
}

/// 季集匹配 e.g. "2x01", "S01E03", "S01-02xE001-150", "S01E06v2"
fn match_season_and_episode(element: &mut Element, token: &mut Token) {
    if let Some((_, s1, s2, e1, e2, v)) = regex_captures!(
        r"(?i)S?(\d{1,2})(?:-S?(\d{1,2}))?(?:X|[ ._-xX]?E)(\d{1,4})(?:-E?(\d{1,4}))?(V\d)?",
        &token.to_text()
    ) {
        let mut season = format!("S{s1}");
        if !s2.is_empty() {
            season.push_str("-S");
            season.push_str(s2);
        }
        element.anime_season = Some(season);

        let mut episode = format!("E{e1}");
        if !e2.is_empty() {
            episode.push_str("-E");
            episode.push_str(e2);
        }
        element.episode_number = Some(episode);

        if !v.is_empty() {
            element.release_version = Some(v.into());
        }

        token.set_identifier();
        return;
    }

    if !regex_is_match!("[全共].+[集话話期季]|[集话話期季]全", &token.to_text()) {
        if let Some((_, s, e)) = regex_captures!("第?(.+)季第?(.+)[集话話期]", &token.to_text())
        {
            token.set_identifier();
            if let Some(se) = s.auto_parse_u16() {
                element.anime_season = Some(format!("S{se}"));
            }
            if let Some(ep) = e.auto_parse_u16() {
                element.episode_number = Some(format!("E{ep}"));
            }
        }
    }
}

/// 半集匹配，仅允许 x.5, e.g. "07.5"
fn match_fractional_episode(element: &mut Element, token: &mut Token) {
    if let Some(num) = regex_captures!(r"\d+\.5", &token.to_text()) {
        element.episode_number = Some(format!("E{num}"));
        token.set_identifier();
    }
}

/// 集分部匹配 e.g. "4a", "111C"
fn match_partial_episode(element: &mut Element, token: &mut Token) {
    if let Some(ep) = regex_captures!(r"(?i)\d{1,4}[ABC]", &token.to_text()) {
        element.episode_number = Some(format!("E{ep}"));
        token.set_identifier();
    }
}

/// e.g. "#01", "#02-03v2"
fn match_number_sign(element: &mut Element, token: &mut Token) {
    if let Some((_, e1, e2, v)) =
        regex_captures!(r"#(\d{1,4})(?:[-~&+](\d{1,4}))?([vV]\d)?", &token.to_text())
    {
        let mut episode = format!("E{e1}");
        if !e2.is_empty() {
            episode.push_str("-E");
            episode.push_str(e2);
        }
        element.episode_number = Some(episode);

        if !v.is_empty() {
            element.release_version = Some(v.into());
        }

        token.set_identifier();
    }
}

/// 按大小猜测集数，准确度不高, e.g. "01 (176)", "29 (04)"
fn match_equivalent_num(element: &mut Element, tokens: &Tokens, token: &mut Token) {
    let number = token.to_text().parse().unwrap_or(1900);
    if is_token_isolated(tokens, token) || EP_NUM_MAX < number {
        return;
    }

    // 找下一个 (
    let next = tokens.find_next_not_delimiter(token);
    if !next.is_open_bracket() {
        return;
    }

    let mut next = tokens.find_next_enclosed_not_delimiter(&next);
    // 检查括号内是否为 (数字)
    let next_num = next.to_text().parse().unwrap_or(1900);
    if !(next.is_unknown() && is_token_isolated(tokens, &next) && next_num <= EP_NUM_MAX) {
        return;
    }

    next.set_identifier();
    token.set_identifier();
    if number < next_num {
        element.anime_season = Some(format!("S{number}"));
        element.episode_number = Some(format!("E{next_num}"));
    } else {
        element.anime_season = Some(format!("S{next_num}"));
        element.episode_number = Some(format!("E{number}"));
    }
}

/// 纯数字匹配，token 为纯数字 e.g. " - 08"
fn match_separated_num(element: &mut Element, tokens: &Tokens, token: &mut Token) {
    let prev = tokens.find_prev_valid(token);
    if regex_is_match!(r"\s+-\s+", &prev.to_text()) {
        // 调用时已经保证了 token 为纯数字
        element.episode_number = Some(format!("E{}", token.to_text()));
        token.set_identifier();
    }
}

/// 纯数字匹配，token 为纯数字 e.g. (12)
fn match_isolated_num(element: &mut Element, tokens: &Tokens, token: &mut Token) {
    if !token.enclosed() || !is_token_isolated(tokens, token) {
        return;
    }
    // 调用时已经保证了 token 为纯数字
    element.episode_number = Some(format!("E{}", token.to_text()));
    token.set_identifier();
}
