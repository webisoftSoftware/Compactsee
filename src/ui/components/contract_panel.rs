use leptos::prelude::*;

use crate::{
    domain::ContractEvent,
    ui::components::{contract_event_card::ContractEventCard, state_view::StateView},
};

#[component]
pub fn ContractPanel(contract_events: ReadSignal<Vec<ContractEvent>>) -> impl IntoView {
    let (selected_event, set_selected_event) = signal(None::<ContractEvent>);
    let (selected_index, set_selected_index) = signal(None::<usize>);
    view! {
        <div class="flex gap-4 h-96 w-[1000px]">
            // Left panel - Event cards
            <div class="w-1/2 overflow-y-auto border border-base-300 rounded-lg p-4">
                <div class="space-y-2">
                    <ForEnumerate
                        each=move || contract_events.get()
                        key=|event| event.state.clone() // need to change this
                        let(index, event) >
                            <ContractEventCard event=event set_selected_event=set_selected_event set_selected_index=set_selected_index index=index/>
                    </ForEnumerate>
                </div>
            </div>

            // Right panel - State view
            <div class="w-1/2 border border-base-300 rounded-lg p-4">
                <Show
                    when=move || { selected_event.get().is_some() }
                    fallback=move || view! {
                        <div class="card bg-base-200 h-full">
                            <div class="card-body flex items-center justify-center">
                                <p class="text-base-content/60">"Select an event to view details"</p>
                            </div>
                        </div>
                    }>
                    <StateView state=selected_event.get().unwrap().state event_index=selected_index />
                </Show>
            </div>
        </div>
    } .into_any()
}
