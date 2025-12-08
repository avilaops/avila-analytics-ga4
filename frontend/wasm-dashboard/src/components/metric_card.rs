use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MetricCardProps {
    pub title: String,
    pub value: String,
    pub change: Option<f64>,
    pub icon: String,
}

#[function_component(MetricCard)]
pub fn metric_card(props: &MetricCardProps) -> Html {
    let change_class = if let Some(change) = props.change {
        if change >= 0.0 {
            "metric-change positive"
        } else {
            "metric-change negative"
        }
    } else {
        "metric-change"
    };

    let change_icon = if let Some(change) = props.change {
        if change >= 0.0 { "📈" } else { "📉" }
    } else {
        ""
    };

    html! {
        <div class="metric-card">
            <div class="metric-icon">{&props.icon}</div>
            <div class="metric-content">
                <div class="metric-title">{&props.title}</div>
                <div class="metric-value">{&props.value}</div>
                if let Some(change) = props.change {
                    <div class={change_class}>
                        <span class="change-icon">{change_icon}</span>
                        <span class="change-value">{format!("{:.1}%", change.abs())}</span>
                    </div>
                }
            </div>
        </div>
    }
}
