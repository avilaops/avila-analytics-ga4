use yew::prelude::*;
use gloo_timers::callback::Interval;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;

#[function_component(RealtimeChart)]
pub fn realtime_chart() -> Html {
    let canvas_ref = use_node_ref();
    let data_points = use_state(|| vec![0; 60]);

    {
        let canvas_ref = canvas_ref.clone();
        let data_points = data_points.clone();

        use_effect_with((), move |_| {
            let interval = Interval::new(1000, move || {
                // Simulate new data point
                let new_value = (js_sys::Math::random() * 100.0) as i32;
                let mut points = (*data_points).clone();
                points.remove(0);
                points.push(new_value);
                data_points.set(points.clone());

                // Draw chart
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    draw_chart(&canvas, &points);
                }
            });

            move || drop(interval)
        });
    }

    html! {
        <div class="realtime-chart-container">
            <h3 class="chart-title">{"Active Users (Last 60 seconds)"}</h3>
            <canvas ref={canvas_ref} class="chart-canvas" width="800" height="300"></canvas>
        </div>
    }
}

fn draw_chart(canvas: &HtmlCanvasElement, data: &[i32]) {
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;

    // Clear canvas
    ctx.clear_rect(0.0, 0.0, width, height);

    // Draw grid
    ctx.set_stroke_style(&"#e0e0e0".into());
    ctx.set_line_width(0.5);
    for i in 0..5 {
        let y = (height / 4.0) * i as f64;
        ctx.begin_path();
        ctx.move_to(0.0, y);
        ctx.line_to(width, y);
        ctx.stroke();
    }

    // Draw line
    if data.is_empty() {
        return;
    }

    let max_value = *data.iter().max().unwrap_or(&1) as f64;
    let step = width / (data.len() - 1) as f64;

    ctx.set_stroke_style(&"#4CAF50".into());
    ctx.set_line_width(2.0);
    ctx.begin_path();

    for (i, &value) in data.iter().enumerate() {
        let x = i as f64 * step;
        let y = height - (value as f64 / max_value * height * 0.9);

        if i == 0 {
            ctx.move_to(x, y);
        } else {
            ctx.line_to(x, y);
        }
    }

    ctx.stroke();

    // Draw points
    ctx.set_fill_style(&"#4CAF50".into());
    for (i, &value) in data.iter().enumerate() {
        let x = i as f64 * step;
        let y = height - (value as f64 / max_value * height * 0.9);

        ctx.begin_path();
        ctx.arc(x, y, 3.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        ctx.fill();
    }
}
