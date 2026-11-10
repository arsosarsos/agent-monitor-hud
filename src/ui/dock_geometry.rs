#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Edge { Left, Right, Top, Bottom }

pub fn nearest_edge(window: (i32, i32, i32, i32), panel: (i32, i32, i32, i32)) -> Edge {
    [
        ((panel.0 - window.0).abs(), Edge::Left),
        ((panel.2 - window.2).abs(), Edge::Right),
        ((panel.1 - window.1).abs(), Edge::Top),
        ((panel.3 - window.3).abs(), Edge::Bottom),
    ].into_iter().min_by_key(|(distance, _)| *distance).unwrap().1
}

pub fn dock_position(edge: Edge, window: (i32, i32, i32, i32), size: (i32, i32), offset: i32) -> (i32, i32) {
    let (left, top, right, bottom) = window;
    let (width, height) = size;
    let x = left + offset.clamp(0, (right - left - width).max(0));
    let y = top + offset.clamp(0, (bottom - top - height).max(0));
    match edge {
        Edge::Left => (left, y),
        Edge::Right => ((right - width).max(left), y),
        Edge::Top => (x, top),
        Edge::Bottom => (x, (bottom - height).max(top)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stays_inside_edge_and_clamps_when_window_shrinks() {
        assert_eq!(dock_position(Edge::Right, (100, 200, 700, 600), (200, 50), 100), (500, 300));
        assert_eq!(dock_position(Edge::Right, (150, 250, 550, 350), (200, 50), 100), (350, 300));
        assert_eq!(dock_position(Edge::Bottom, (150, 250, 550, 350), (200, 50), 100), (250, 300));
    }

    #[test]
    fn chooses_nearest_window_edge() {
        assert_eq!(nearest_edge((100, 100, 900, 700), (105, 250, 305, 300)), Edge::Left);
        assert_eq!(nearest_edge((100, 100, 900, 700), (698, 250, 898, 300)), Edge::Right);
        assert_eq!(nearest_edge((100, 100, 900, 700), (300, 649, 500, 699)), Edge::Bottom);
    }
}
