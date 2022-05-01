use std::f32::consts::PI;

use glium::{Frame, Surface, Rect};

pub fn draw_pie_to_frame(frame: &mut Frame, radius: u32, color: Option<(f32, f32, f32, f32)>, x: u32, y: u32, cut: (f32, f32)) {
    for draw_y in y - radius .. y {
        let height_offset = y - draw_y;
        let width_offset = ((radius * radius - height_offset * height_offset) as f32).sqrt().round() as u32;
        let left_min = x - width_offset;
        let right_max = x + width_offset;
        let cut1 = match cut.0 > PI {
            true => Some((x as f32 - height_offset as f32 / (cut.0 - PI).tan()).round() as u32),
            false => None
        };
        let cut0 = match cut.1 > PI {
            true => Some((x as f32 - height_offset as f32 / (cut.1 - PI).tan()).round() as u32),
            false => None
        };
        let centered = cut.1 > cut.0;
        draw_line_of_pie(frame, left_min, right_max, cut0, cut1, color, draw_y, centered);
    }
    for draw_y in y + 1 .. y + radius + 1 {
        let height_offset = draw_y - y;
        let width_offset = ((radius * radius - height_offset * height_offset) as f32).sqrt().round() as u32;
        let left_min = x - width_offset;
        let right_max = x + width_offset;
        let cut0 = match cut.0 < PI {
            true => Some((x as f32 + height_offset as f32 / (cut.0).tan()).round() as u32),
            false => None
        };
        let cut1 = match cut.1 < PI {
            true => Some((x as f32 + height_offset as f32 / (cut.1).tan()).round() as u32),
            false => None
        };
        let centered = cut.1 > cut.0;
        draw_line_of_pie(frame, left_min, right_max, cut0, cut1, color, draw_y, centered);
    }
    if cut.1 <= cut.0 {
        frame.clear(Some(&Rect {
            left: x,
            bottom: y,
            width: radius + 1,
            height: 1,
        }), color, false, None, None);
    }
    if (cut.1 + PI) % (2.0 * PI) <= (cut.0 + PI) % (2.0 * PI) {
        frame.clear(Some(&Rect {
            left: x - radius,
            bottom: y,
            width: radius + 1,
            height: 1,
        }), color, false, None, None);
    }
}

fn draw_line_of_pie(frame: &mut Frame, left_min: u32, right_max: u32, cut0: Option<u32>, cut1: Option<u32>, color: Option<(f32, f32, f32, f32)>, height: u32, centered: bool) {
    match (cut0, cut1) {
        (None, None) => {
            if !centered {
                frame.clear(Some(&Rect {
                    left: left_min,
                    bottom: height,
                    width: right_max - left_min + 1,
                    height: 1,
                }), color, false, None, None);
            }
        }
        (Some(v), None) => {
            if left_min <= v {
                let max = v.min(right_max);
                frame.clear(Some(&Rect {
                    left: left_min,
                    bottom: height,
                    width: max - left_min + 1,
                    height: 1,
                }), color, false, None, None);
            }
        }
        (None, Some(v)) => {
            if v <= right_max {
                let min = v.max(left_min);
                frame.clear(Some(&Rect {
                    left: min,
                    bottom: height,
                    width: right_max - min + 1,
                    height: 1,
                }), color, false, None, None);
            }
        }
        (Some(v_0), Some(v_1)) => {
            if centered {
                let min = v_1.max(left_min);
                let max = v_0.min(right_max);
                if min <= max {
                    frame.clear(Some(&Rect {
                        left: min,
                        bottom: height,
                        width: max - min + 1,
                        height: 1,
                    }), color, false, None, None);
                }
            } else {
                if left_min <= v_0 {
                    let max = v_0.min(right_max);
                    frame.clear(Some(&Rect {
                        left: left_min,
                        bottom: height,
                        width: max - left_min + 1,
                        height: 1,
                    }), color, false, None, None);
                }
                if v_1 <= right_max {
                    let min = v_1.max(left_min);
                    frame.clear(Some(&Rect {
                        left: min,
                        bottom: height,
                        width: right_max - min + 1,
                        height: 1,
                    }), color, false, None, None);
                }
            }
        }
    }
}
