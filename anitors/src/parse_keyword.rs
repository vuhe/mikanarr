use lazy_regex::{regex_captures, regex_is_match};

use crate::tokens::{Token, Tokens};
use crate::Element;

type Node<'a> = (&'a mut Element, &'a mut Token);

pub(crate) fn parse_keyword(element: &mut Element, tokens: &Tokens) {
    tokens
        .unknown_tokens()
        .iter_mut()
        .for_each(|token| handle_single_token(element, token))
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
fn anime_type(node: Node) -> Option<Node> {
    if let Some((t, _, _, _)) = regex_captures!(
        "(?i)^((NC)?ED|ENDING|(NC)?OP|OPENING|PREVIEW|PV|SP)$",
        &node.1.to_text()
    ) {
        node.0.anime_type.push(t.into());
        return None;
    }
    return Some(node);
}

/// 音频编码相关
fn audio_term(node: Node) -> Option<Node> {
    if let Some((t, _, _, _, _, _, _)) = regex_captures!(
        "(?i)^(2(.0)?CH|DTS(-ES|5.1|HD|HDMA)?|5.1(CH)?|TRUEHD5.1|\
        AAC(X2|X3|X4)?|AC3|EAC3|E-AC-3|FLAC(X2|X3|X4)?|LOSSLESS|MP3|OGG|VORBIS|Atmos|\
        DUAL[- ]?AUDIO|MULTI[- ]?AUDIO)$",
        &node.1.to_text()
    ) {
        node.0.audio_term.push(t.into());
        node.1.set_identifier();
        return None;
    }
    return Some(node);
}

/// 设备类型相关，仅做标识用于后续识别
fn device_compatibility(node: Node) -> Option<Node> {
    if regex_is_match!(
        "(?i)^(IPAD3|IPHONE5|IPOD|PS3|XBOX(360)?)$",
        &node.1.to_text()
    ) {
        node.1.set_identifier();
        return None;
    }
    if regex_is_match!("(?i)^ANDROID$", &node.1.to_text()) {
        return None;
    }
    return Some(node);
}

/// 影片语言相关
fn language(node: Node) -> Option<Node> {
    if let Some((t, _, _)) = regex_captures!(
        "(?i)^(ENG(LISH)?|ESPANOL|JAP|PT-BR|SPANISH|VOSTFR)$",
        &node.1.to_text()
    ) {
        node.0.language.push(t.into());
        node.1.set_identifier();
        return None;
    }
    // e.g. "Tokyo ESP", "Bokura ga Ita"
    if let Some((t, _)) = regex_captures!("(?i)^(ESP|ITA)$", &node.1.to_text()) {
        node.0.language.push(t.into());
        return None;
    }
    return Some(node);
}

/// 其他
fn other(node: Node) -> Option<Node> {
    if let Some((t, _)) = regex_captures!(
        "(?i)^(REMASTER|REMASTERED|UNCENSORED|UNCUT|TS|VFR|WIDESCREEN|WS)$",
        &node.1.to_text()
    ) {
        node.0.other.push(t.into());
        node.1.set_identifier();
        return None;
    }
    return Some(node);
}

/// 发布组相关
fn release_group(node: Node) -> Option<Node> {
    if let Some(t) = regex_captures!("(?i)^THORA$", &node.1.to_text()) {
        node.0.release_group = Some(t.into());
        node.1.set_identifier();
        return None;
    }
    return Some(node);
}

/// 发布信息相关
fn release_information(node: Node) -> Option<Node> {
    if let Some((t, _)) = regex_captures!("(?i)^(BATCH|COMPLETE|PATCH)$", &node.1.to_text()) {
        node.0.release_information.push(t.into());
        node.1.set_identifier();
        return None;
    }
    // e.g. "The End of Evangelion", "Final Approach"
    if let Some((t, _)) = regex_captures!("(?i)^(END|FINAL)$", &node.1.to_text()) {
        node.0.release_information.push(t.into());
        return None;
    }
    return Some(node);
}

/// 发布版本相关
fn release_version(node: Node) -> Option<Node> {
    if let Some(t) = regex_captures!("(?i)^V[01234]$", &node.1.to_text()) {
        node.0.release_version = Some(t.into());
        node.1.set_identifier();
        return None;
    }
    return Some(node);
}

/// 影片来源相关
fn source(node: Node) -> Option<Node> {
    if let Some((t, _, _, _, _, _, _)) = regex_captures!(
        "(?i)^(DVD(5|9|-R2J|-?RIP)?|R2(DVD|J|JDVD|JDVDRIP)|REMUX|\
        SDTV|HDTV(RIP)?|TV-?RIP|WEB-?DL|WEB(CAST|RIP)|BLU-?RAY|BD(-?RIP)?)$",
        &node.1.to_text()
    ) {
        node.0.source.push(t.into());
        node.1.set_identifier();
        return None;
    }
    return Some(node);
}

/// 影片字幕相关
fn subtitles(node: Node) -> Option<Node> {
    if let Some((t, _, _, _)) = regex_captures!(
        "(?i)^(ASS|BIG5|DUB(BED)?|HARDSUBS?|RAW|SOFTSUBS?|\
        SUB(BED|TITLED)?|MULTIPLE SUBTITLE|MULTI[- ]SUBS)$",
        &node.1.to_text()
    ) {
        node.0.subtitles = Some(t.into());
        node.1.set_identifier();
        return None;
    }
    return Some(node);
}

/// 流媒体相关
fn streaming(node: Node) -> Option<Node> {
    if let Some((t, _, _)) = regex_captures!(
        "(?i)^(BAHA|B(-GLOBAL|ILIBILI)|NETFLIX|NF|ViuTV)$",
        &node.1.to_text()
    ) {
        node.0.streaming = Some(t.into());
        node.1.set_identifier();
        return None;
    }
    return Some(node);
}

/// 视频编码相关
fn video_term(node: Node) -> Option<Node> {
    if let Some((t, _, _, _)) = regex_captures!(
        "(?i)^((10|8)-?BITS?|HI10P?|HI444(P|PP)?|[HX]26[45]|AVC|HEVC|\
        VC\\d?|MPEG\\d?|Xvid|DivX|HDR\\d*|3D)$",
        &node.1.to_text()
    ) {
        node.0.video_term.push(t.into());
        node.1.set_identifier();
        return None;
    }
    return Some(node);
}

/// 视频分辨率相关
fn video_resolution(node: Node) -> Option<Node> {
    if let Some((_, _, num, n)) = regex_captures!(
        r"(?i)(\d{3,4}X)?(480|720|1080|1440|2160|4320)[PI]?|([248])K",
        &node.1.to_text()
    ) {
        if !num.is_empty() {
            node.1.set_identifier();
            node.0.video_resolution = Some(format!("{num}P"));
            return None;
        }
        if !n.is_empty() {
            node.1.set_identifier();
            match n {
                "2" => node.0.video_resolution = Some("1440P".into()),
                "4" => node.0.video_resolution = Some("2160P".into()),
                "8" => node.0.video_resolution = Some("4320P".into()),
                _ => unreachable!(),
            }
            return None;
        }
    }
    return Some(node);
}

/// 视频质量相关
fn video_quality(node: Node) -> Option<Node> {
    if let Some((_, s, v)) = regex_captures!(
        r"(?i)(WEB-?DL)-?(480|720|1080|1440|2160|4320)P?",
        &node.1.to_text()
    ) {
        node.0.source.push(s.into());
        node.0.video_resolution = Some(v.into());
        node.1.set_identifier();
        return None;
    }
    return Some(node);
}

/// 视频容器相关，仅做标识用于后续识别
fn video_format(node: Node) -> Option<Node> {
    if regex_is_match!("(?i)^(MKV|AVI|RMVB|WMV(3|9)?)$", &node.1.to_text()) {
        node.1.set_identifier();
        return None;
    }
    return Some(node);
}

/// 文件 hash 码 (crc32) 相关，仅做标识用于后续识别
fn file_checksum(node: Node) -> Option<Node> {
    if regex_is_match!(r"(?i)^[a-e\d]{8}$", &node.1.to_text()) {
        node.1.set_identifier();
        return None;
    }
    return Some(node);
}
