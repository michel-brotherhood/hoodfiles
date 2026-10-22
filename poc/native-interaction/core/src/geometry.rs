//! Posicionamento do overlay perto do cursor em ambiente multi-monitor e com DPI misto.
//! Todas as coordenadas são físicas (pixels), no espaço virtual da área de trabalho.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn right(&self) -> i64 {
        self.x as i64 + self.w as i64
    }
    pub fn bottom(&self) -> i64 {
        self.y as i64 + self.h as i64
    }
    pub fn contains(&self, px: i32, py: i32) -> bool {
        (px as i64) >= self.x as i64
            && (px as i64) < self.right()
            && (py as i64) >= self.y as i64
            && (py as i64) < self.bottom()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Monitor {
    /// Área útil em pixels físicos.
    pub work_area: Rect,
    pub scale_factor: f64,
}

/// Escolhe o monitor sob o cursor; se nenhum o contiver (borda, cursor fora após
/// mudança de layout), usa o de centro mais próximo. `None` só se a lista é vazia.
pub fn monitor_for_cursor(cx: i32, cy: i32, monitors: &[Monitor]) -> Option<&Monitor> {
    monitors
        .iter()
        .find(|m| m.work_area.contains(cx, cy))
        .or_else(|| {
            monitors.iter().min_by_key(|m| {
                let mx = m.work_area.x as i64 + m.work_area.w as i64 / 2;
                let my = m.work_area.y as i64 + m.work_area.h as i64 / 2;
                let (dx, dy) = (mx - cx as i64, my - cy as i64);
                dx * dx + dy * dy
            })
        })
}

/// Posiciona um overlay de `logical_size` (pixels lógicos) deslocado do cursor.
/// O tamanho é convertido com a escala do monitor do cursor. Se não couber à
/// direita/abaixo, inverte o lado; no fim, limita à área útil.
pub fn place_overlay(
    cursor: (i32, i32),
    logical_size: (u32, u32),
    offset_logical: (i32, i32),
    monitors: &[Monitor],
) -> Option<Rect> {
    let m = monitor_for_cursor(cursor.0, cursor.1, monitors)?;
    let s = if m.scale_factor.is_finite() && m.scale_factor > 0.0 {
        m.scale_factor
    } else {
        1.0
    };
    let w = ((logical_size.0 as f64) * s).round().max(1.0) as u32;
    let h = ((logical_size.1 as f64) * s).round().max(1.0) as u32;
    let ox = ((offset_logical.0 as f64) * s).round() as i64;
    let oy = ((offset_logical.1 as f64) * s).round() as i64;
    let wa = m.work_area;

    let mut x = cursor.0 as i64 + ox;
    let mut y = cursor.1 as i64 + oy;
    if x + w as i64 > wa.right() {
        x = cursor.0 as i64 - ox - w as i64;
    }
    if y + h as i64 > wa.bottom() {
        y = cursor.1 as i64 - oy - h as i64;
    }
    // Clamp final (overlay maior que a área vira alinhado ao canto superior esquerdo).
    let max_x = (wa.right() - w as i64).max(wa.x as i64);
    let max_y = (wa.bottom() - h as i64).max(wa.y as i64);
    x = x.clamp(wa.x as i64, max_x);
    y = y.clamp(wa.y as i64, max_y);
    Some(Rect {
        x: x as i32,
        y: y as i32,
        w,
        h,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mon(x: i32, y: i32, w: u32, h: u32, s: f64) -> Monitor {
        Monitor {
            work_area: Rect { x, y, w, h },
            scale_factor: s,
        }
    }

    #[test]
    fn places_right_below_cursor_when_fits() {
        let r = place_overlay(
            (100, 100),
            (200, 100),
            (16, 16),
            &[mon(0, 0, 1920, 1080, 1.0)],
        )
        .unwrap();
        assert_eq!(
            r,
            Rect {
                x: 116,
                y: 116,
                w: 200,
                h: 100
            }
        );
    }

    #[test]
    fn flips_near_bottom_right_edge() {
        let r = place_overlay(
            (1900, 1070),
            (200, 100),
            (16, 16),
            &[mon(0, 0, 1920, 1080, 1.0)],
        )
        .unwrap();
        assert!(r.right() <= 1920 && r.bottom() <= 1080);
        assert!(
            (r.x as i64) < 1900 && (r.y as i64) < 1070,
            "deve ficar à esquerda/acima do cursor"
        );
    }

    #[test]
    fn scales_size_by_monitor_under_cursor_mixed_dpi() {
        let ms = [mon(0, 0, 1920, 1080, 1.0), mon(1920, 0, 3840, 2160, 2.0)];
        let r = place_overlay((2000, 100), (200, 100), (16, 16), &ms).unwrap();
        assert_eq!((r.w, r.h), (400, 200));
        assert!(r.x >= 1920);
    }

    #[test]
    fn negative_coordinates_monitor_left_of_primary() {
        let ms = [mon(-1920, 0, 1920, 1080, 1.0), mon(0, 0, 1920, 1080, 1.0)];
        let r = place_overlay((-10, 500), (200, 100), (16, 16), &ms).unwrap();
        assert!(
            r.x >= -1920 && r.right() <= 0,
            "não pode cruzar para o outro monitor: {r:?}"
        );
    }

    #[test]
    fn cursor_outside_all_monitors_uses_nearest() {
        let ms = [mon(0, 0, 1000, 1000, 1.0), mon(1000, 0, 1000, 1000, 1.0)];
        let m = monitor_for_cursor(5000, 500, &ms).unwrap();
        assert_eq!(m.work_area.x, 1000);
    }

    #[test]
    fn empty_monitors_and_invalid_scale() {
        assert!(place_overlay((0, 0), (10, 10), (0, 0), &[]).is_none());
        let r = place_overlay((0, 0), (10, 10), (0, 0), &[mon(0, 0, 100, 100, f64::NAN)]).unwrap();
        assert_eq!((r.w, r.h), (10, 10));
    }

    #[test]
    fn overlay_larger_than_work_area_anchors_top_left() {
        let r = place_overlay((50, 50), (500, 500), (0, 0), &[mon(0, 0, 100, 100, 1.0)]).unwrap();
        assert_eq!((r.x, r.y), (0, 0));
    }
}
