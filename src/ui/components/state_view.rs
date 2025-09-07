use leptos::prelude::*;

#[component]
pub fn StateView(state: String, event_index: ReadSignal<Option<usize>>) -> impl IntoView {
    view! {
        <div class="card bg-base-100 h-full">
            <div class="card-header text-center pr-4">
                <h3 class="card-title text-lg">"Event: "{event_index}</h3>
            </div>
            <div class="card-body p-4 h-0 flex-1 overflow-auto">
                <pre class="whitespace-pre-wrap break-words text-sm font-mono bg-base-200 p-4 rounded">{state}</pre>
            </div>
        </div>
    }
}
