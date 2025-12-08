use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <div class="header-content">
                <div class="logo">
                    <span class="logo-icon">{"📊"}</span>
                    <span class="logo-text">{"Avila Analytics"}</span>
                </div>
                <nav class="header-nav">
                    <button class="nav-btn active">{"Dashboard"}</button>
                    <button class="nav-btn">{"Reports"}</button>
                    <button class="nav-btn">{"Settings"}</button>
                </nav>
                <div class="header-actions">
                    <button class="icon-btn" title="Notifications">{"🔔"}</button>
                    <button class="icon-btn" title="Profile">{"👤"}</button>
                </div>
            </div>
        </header>
    }
}
