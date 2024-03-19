use leptos::*;
use leptos_router::*;

use crate::note::Note;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="" view=Note/>
                <Route path="/note" view=Note/>
            </Routes>
        </Router>
    }
}