pub fn get_toc() {
    let pipeline = gst::parse_launch("playbin uri=file://Pink_Floyd_The_Wall_CD1.flac").unwrap();
    pipeline;
}
