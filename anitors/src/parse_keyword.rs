use lazy_regex::{regex_captures, regex_is_match};

use crate::tokens::{Token, Tokens};
use crate::Element;

type Node<'a> = (&'a mut Element, &'a mut Token);

pub(crate) fn parse_keyword(element: &mut Element, tokens: &Tokens) {
    tokens
        .unknown_tokens()
        .for_each(|mut token| handle_single_token(element, &mut token))
}

fn handle_single_token(element: &mut Element, token: &mut Token) {
    [(element, token)]
        .into_iter()
        .filter_map(anime_type)
        .filter_map(audio_term)
        .filter_map(device_compatibility)
        .filter_map(language)
        .filter_map(other)
        .filter_map(release_group)
        .filter_map(release_information)
        .filter_map(release_version)
        .filter_map(source)
        .filter_map(subtitles)
        .filter_map(streaming)
        .filter_map(video_term)
        .filter_map(video_resolution)
        .filter_map(video_quality)
        .filter_map(video_format)
        .filter_map(file_checksum)
        .count();
}

// =================== token 处理函数，返回值决定是否继续处理 ===================

/// 动漫类型相关
fn anime_type((element, token): Node) -> Option<Node> {
    if let Some((t, _, _, _)) = regex_captures!(
        "(?i)^((NC)?ED|ENDING|(NC)?OP|OPENING|PREVIEW|PV|SP)$",
        &token.to_text()
    ) {
        element.anime_type.push(t.into());
        return None;
    }
    return Some((element, token));
}

/// 音频编码相关
fn audio_term((element, token): Node) -> Option<Node> {
    if let Some((t, _, _, _, _, _, _)) = regex_captures!(
        "(?i)^(2(.0)?CH|DTS(-ES|5.1|HD|HDMA)?|5.1(CH)?|TRUEHD5.1|\
        AAC(X2|X3|X4)?|AC3|EAC3|E-AC-3|FLAC(X2|X3|X4)?|LOSSLESS|MP3|OGG|VORBIS|Atmos|\
        DUAL[- ]?AUDIO|MULTI[- ]?AUDIO)$",
        &token.to_text()
    ) {
        element.audio_term.push(t.into());
        token.set_identifier();
        return None;
    }
    return Some((element, token));
}

/// 设备类型相关，仅做标识用于后续识别
fn device_compatibility((element, token): Node) -> Option<Node> {
    if regex_is_match!(
        "(?i)^(IPAD3|IPHONE5|IPOD|PS3|XBOX(360)?)$",
        &token.to_text()
    ) {
        token.set_identifier();
        return None;
    }
    if regex_is_match!("(?i)^ANDROID$", &token.to_text()) {
        return None;
    }
    return Some((element, token));
}

/// 影片语言相关
fn language((element, token): Node) -> Option<Node> {
    if let Some((t, _, _)) = regex_captures!(
        "(?i)^(ENG(LISH)?|ESPANOL|JAP|PT-BR|SPANISH|VOSTFR)$",
        &token.to_text()
    ) {
        element.language.push(t.into());
        token.set_identifier();
        return None;
    }
    // e.g. "Tokyo ESP", "Bokura ga Ita"
    if let Some((t, _)) = regex_captures!("(?i)^(ESP|ITA)$", &token.to_text()) {
        element.language.push(t.into());
        return None;
    }
    return Some((element, token));
}

/// 其他
fn other((element, token): Node) -> Option<Node> {
    if let Some((t, _)) = regex_captures!(
        "(?i)^(REMASTER|REMASTERED|UNCENSORED|UNCUT|TS|VFR|WIDESCREEN|WS)$",
        &token.to_text()
    ) {
        element.other.push(t.into());
        token.set_identifier();
        return None;
    }
    return Some((element, token));
}

/// 发布组相关
fn release_group((element, token): Node) -> Option<Node> {
    if let Some(t) = regex_captures!("(?i)^THORA$", &token.to_text()) {
        element.release_group = Some(t.into());
        token.set_identifier();
        return None;
    }
    return Some((element, token));
}

/// 发布信息相关
fn release_information((element, token): Node) -> Option<Node> {
    if let Some((t, _)) = regex_captures!("(?i)^(BATCH|COMPLETE|PATCH)$", &token.to_text()) {
        element.release_information.push(t.into());
        token.set_identifier();
        return None;
    }
    // e.g. "The End of Evangelion", "Final Approach"
    if let Some((t, _)) = regex_captures!("(?i)^(END|FINAL)$", &token.to_text()) {
        element.release_information.push(t.into());
        return None;
    }
    return Some((element, token));
}

/// 发布版本相关
fn release_version((element, token): Node) -> Option<Node> {
    if let Some(t) = regex_captures!("(?i)^V[01234]$", &token.to_text()) {
        element.release_version = Some(t.into());
        token.set_identifier();
        return None;
    }
    return Some((element, token));
}

/// 影片来源相关
fn source((element, token): Node) -> Option<Node> {
    if let Some((t, _, _, _, _, _, _)) = regex_captures!(
        "(?i)^(DVD(5|9|-R2J|-?RIP)?|R2(DVD|J|JDVD|JDVDRIP)|REMUX|\
        SDTV|HDTV(RIP)?|TV-?RIP|WEB-?DL|WEB(CAST|RIP)|BLU-?RAY|BD(-?RIP)?)$",
        &token.to_text()
    ) {
        element.source.push(t.into());
        token.set_identifier();
        return None;
    }
    return Some((element, token));
}

/// 影片字幕相关
fn subtitles((element, token): Node) -> Option<Node> {
    if let Some((t, _, _, _)) = regex_captures!(
        "(?i)^(ASS|BIG5|DUB(BED)?|HARDSUBS?|RAW|SOFTSUBS?|\
        SUB(BED|TITLED)?|MULTIPLE SUBTITLE|MULTI[- ]SUBS)$",
        &token.to_text()
    ) {
        element.subtitles = Some(t.into());
        token.set_identifier();
        return None;
    }
    return Some((element, token));
}

/// 流媒体相关
fn streaming((element, token): Node) -> Option<Node> {
    if let Some((t, _, _)) = regex_captures!(
        "(?i)^(BAHA|B(-GLOBAL|ILIBILI)|NETFLIX|NF|ViuTV)$",
        &token.to_text()
    ) {
        element.streaming = Some(t.into());
        token.set_identifier();
        return None;
    }
    return Some((element, token));
}

/// 视频编码相关
fn video_term((element, token): Node) -> Option<Node> {
    if let Some((t, _, _, _)) = regex_captures!(
        "(?i)^((10|8)-?BITS?|HI10P?|HI444(P|PP)?|[HX]26[45]|AVC|HEVC|\
        VC\\d?|MPEG\\d?|Xvid|DivX|HDR\\d*|3D)$",
        &token.to_text()
    ) {
        element.video_term.push(t.into());
        token.set_identifier();
        return None;
    }
    return Some((element, token));
}

/// 视频分辨率相关
fn video_resolution((element, token): Node) -> Option<Node> {
    if let Some((_, _, num, n)) = regex_captures!(
        r"(?i)(\d{3,4}X)?(480|720|1080|1440|2160|4320)[PI]?|([248])K",
        &token.to_text()
    ) {
        if !num.is_empty() {
            token.set_identifier();
            element.video_resolution = Some(format!("{num}P"));
            return None;
        }
        if !n.is_empty() {
            token.set_identifier();
            match n {
                "2" => element.video_resolution = Some("1440P".into()),
                "4" => element.video_resolution = Some("2160P".into()),
                "8" => element.video_resolution = Some("4320P".into()),
                _ => unreachable!(),
            }
            return None;
        }
    }
    return Some((element, token));
}

/// 视频质量相关
fn video_quality((element, token): Node) -> Option<Node> {
    if let Some((_, s, v)) = regex_captures!(
        r"(?i)(WEB-?DL)-?(480|720|1080|1440|2160|4320)P?",
        &token.to_text()
    ) {
        element.source.push(s.into());
        element.video_resolution = Some(v.into());
        token.set_identifier();
        return None;
    }
    return Some((element, token));
}

/// 视频容器相关，仅做标识用于后续识别
fn video_format((element, token): Node) -> Option<Node> {
    if regex_is_match!("(?i)^(MKV|AVI|RMVB|WMV(3|9)?)$", &token.to_text()) {
        token.set_identifier();
        return None;
    }
    return Some((element, token));
}

/// 文件 hash 码 (crc32) 相关，仅做标识用于后续识别
fn file_checksum((element, token): Node) -> Option<Node> {
    if regex_is_match!(r"(?i)^[a-e\d]{8}$", &token.to_text()) {
        token.set_identifier();
        return None;
    }
    return Some((element, token));
}
