use anitors::Element;

#[test]
fn case_1() {
    let title = "【爪爪字幕组】★7月新番[欢迎来到实力至上主义的教室 第二季/Youkoso Jitsuryoku Shijou Shugi no Kyoushitsu e S2][11][1080p][HEVC][GB][MP4][招募翻译校对]";
    println!("{:#?}", Element::parse(title));
}

#[test]
fn case_2() {
    let title = "[ANi] 捡走被人悔婚的千金，教会她坏坏的幸福生活 - 03 [1080P][Baha][WEB-DL][AAC AVC][CHT][MP4]";
    println!("{:#?}", Element::parse(title));
}
