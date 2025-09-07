use leptos::prelude::*;

use crate::domain::ContractEvent;
#[component]
pub fn ContractEventCard(
    event: ContractEvent,
    index: ReadSignal<usize>,
    set_selected_event: WriteSignal<Option<ContractEvent>>,
    set_selected_index: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let event_copy = event.clone();
    let type_name = event_copy.type_name;
    view! {
        <div
            class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer border border-base-300 hover:border-primary"
            on:click=move|_| {
                set_selected_index.set(Some(index.get()));
                set_selected_event.set(Some(event.clone()));
            }
        >
            <div class="card-body p-3">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2">
                        <div class="badge badge-primary badge-sm">{move || index.get()}</div>
                        <span class="font-medium text-sm">{type_name}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
