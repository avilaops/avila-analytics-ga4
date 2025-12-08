use yew::prelude::*;
use super::{MetricCard, RealtimeChart, EventList};

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <main class="dashboard">
            <div class="dashboard-header">
                <h1>{"Real-time Dashboard"}</h1>
                <div class="date-range">
                    <button class="date-btn">{"Today"}</button>
                    <button class="date-btn">{"Last 7 days"}</button>
                    <button class="date-btn">{"Last 30 days"}</button>
                </div>
            </div>

            <div class="metrics-grid">
                <MetricCard
                    title="Active Users"
                    value="1,234"
                    change={12.5}
                    icon="👥"
                />
                <MetricCard
                    title="Page Views"
                    value="45,678"
                    change={8.3}
                    icon="📄"
                />
                <MetricCard
                    title="Events"
                    value="89,012"
                    change={-2.1}
                    icon="🎯"
                />
                <MetricCard
                    title="Conversions"
                    value="345"
                    change={15.7}
                    icon="✅"
                />
            </div>

            <div class="dashboard-grid">
                <div class="chart-section">
                    <RealtimeChart />
                </div>

                <div class="events-section">
                    <EventList />
                </div>
            </div>

            <div class="dashboard-grid-2">
                <div class="stats-card">
                    <h3>{"Top Pages"}</h3>
                    <div class="stats-list">
                        <div class="stats-item">
                            <span class="stats-label">{"/home"}</span>
                            <span class="stats-value">{"12,345"}</span>
                        </div>
                        <div class="stats-item">
                            <span class="stats-label">{"/products"}</span>
                            <span class="stats-value">{"8,901"}</span>
                        </div>
                        <div class="stats-item">
                            <span class="stats-label">{"/about"}</span>
                            <span class="stats-value">{"5,678"}</span>
                        </div>
                    </div>
                </div>

                <div class="stats-card">
                    <h3>{"Traffic Sources"}</h3>
                    <div class="stats-list">
                        <div class="stats-item">
                            <span class="stats-label">{"Direct"}</span>
                            <span class="stats-value">{"45%"}</span>
                        </div>
                        <div class="stats-item">
                            <span class="stats-label">{"Organic"}</span>
                            <span class="stats-value">{"32%"}</span>
                        </div>
                        <div class="stats-item">
                            <span class="stats-label">{"Social"}</span>
                            <span class="stats-value">{"23%"}</span>
                        </div>
                    </div>
                </div>

                <div class="stats-card">
                    <h3>{"Devices"}</h3>
                    <div class="stats-list">
                        <div class="stats-item">
                            <span class="stats-label">{"Desktop"}</span>
                            <span class="stats-value">{"60%"}</span>
                        </div>
                        <div class="stats-item">
                            <span class="stats-label">{"Mobile"}</span>
                            <span class="stats-value">{"35%"}</span>
                        </div>
                        <div class="stats-item">
                            <span class="stats-label">{"Tablet"}</span>
                            <span class="stats-value">{"5%"}</span>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
