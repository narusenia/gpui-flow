use gpui::*;
use gpui_component::button::Button;
use gpui_component::badge::Badge;
use gpui_component::label::Label;
use gpui_component::Sizable;
use gpui_flow::*;

const BG: u32 = 0x09090b;
const GRID: u32 = 0x18181b;
const CARD: u32 = 0x0a0a0c;
const CARD_BORDER: u32 = 0x27272a;
const TEXT: u32 = 0xfafafa;

struct FlowView {
    flow: Entity<FlowGraph>,
}

impl Render for FlowView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .relative()
            .child(self.flow.clone())
    }
}

fn render_with_button(node: &FlowNode, _w: &mut Window, _cx: &mut App) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(Label::new(node.label.to_string()).text_sm())
        .child(
            div()
                .flex()
                .gap_1()
                .child(Button::new("run").label("Run").small())
                .child(Button::new("stop").label("Stop").small()),
        )
        .into_any_element()
}

fn render_with_badge(node: &FlowNode, _w: &mut Window, _cx: &mut App) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(Label::new(node.label.to_string()).text_sm())
        .child(Badge::new().count(3))
        .into_any_element()
}

fn main() {
    gpui_platform::application().run(move |cx: &mut App| {
        gpui_component::init(cx);
        gpui_component::Theme::sync_system_appearance(None, cx);

        let bounds = Bounds::centered(None, size(px(900.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| {
                let nodes = vec![
                    FlowNode::new("btn_node", 100.0, 100.0)
                        .label("Action Node")
                        .node_type("action")
                        .size(200.0, 90.0)
                        .handles(vec![HandleDef::source(HandlePosition::Right)]),
                    FlowNode::new("badge_node", 420.0, 100.0)
                        .label("Status Node")
                        .node_type("status")
                        .size(180.0, 80.0)
                        .handles(vec![HandleDef::target(HandlePosition::Left)]),
                ];

                let edges = vec![
                    FlowEdge::new("e1", "btn_node", "badge_node")
                        .color(0x3b82f6)
                        .stroke_width(2.0),
                ];

                let state = cx.new(|_| FlowState::new(nodes, edges));
                let flow = cx.new(|cx| {
                    FlowGraph::new(state, cx)
                        .bg_color(BG)
                        .grid_color(GRID)
                        .node_bg_color(CARD)
                        .node_border_color(CARD_BORDER)
                        .node_renderer("action", render_with_button)
                        .node_renderer("status", render_with_badge)
                });

                cx.new(|_| FlowView { flow })
            },
        )
        .expect("Failed to open window");
    });
}
