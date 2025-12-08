use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <aside class="sidebar">
            <div class="sidebar-section">
                <h3 class="sidebar-title">{"Overview"}</h3>
                <ul class="sidebar-menu">
                    <li class="menu-item active">
                        <span class="menu-icon">{"📈"}</span>
                        <span class="menu-text">{"Real-time"}</span>
                    </li>
                    <li class="menu-item">
                        <span class="menu-icon">{"👥"}</span>
                        <span class="menu-text">{"Users"}</span>
                    </li>
                    <li class="menu-item">
                        <span class="menu-icon">{"📄"}</span>
                        <span class="menu-text">{"Pages"}</span>
                    </li>
                    <li class="menu-item">
                        <span class="menu-icon">{"🛒"}</span>
                        <span class="menu-text">{"E-commerce"}</span>
                    </li>
                </ul>
            </div>

            <div class="sidebar-section">
                <h3 class="sidebar-title">{"Analysis"}</h3>
                <ul class="sidebar-menu">
                    <li class="menu-item">
                        <span class="menu-icon">{"🔍"}</span>
                        <span class="menu-text">{"Events"}</span>
                    </li>
                    <li class="menu-item">
                        <span class="menu-icon">{"🎯"}</span>
                        <span class="menu-text">{"Conversions"}</span>
                    </li>
                    <li class="menu-item">
                        <span class="menu-icon">{"📊"}</span>
                        <span class="menu-text">{"Funnels"}</span>
                    </li>
                </ul>
            </div>
        </aside>
    }
}
