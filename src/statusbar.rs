use std::time::Duration;

pub fn draw_status_bar(frame_read_duration: Duration, loop_duration: Duration, sleep_time: Duration, fps: f64, frame_number: f64, audio_enabled: bool) {
    let debug_read: f64 = frame_read_duration.as_secs_f64() * 1000.0;
    let debug_loop: f64 = loop_duration.as_secs_f64() * 1000.0;
    let debug_sleep: f64 = sleep_time.as_secs_f64() * 1000.0;
    print!(
        "\x1B[0mread: {:.3}ms, draw: {:.3}ms, sleep: {:.3}ms, total: {:.3}ms, targetfps: {:.3}, frame#: {}, audio: {:?}                     ",
        debug_read,
        debug_loop,
        debug_sleep,
        debug_read + debug_loop + debug_sleep,
        fps,
        frame_number,
        audio_enabled
    );
}