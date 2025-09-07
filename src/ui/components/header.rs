use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="navbar bg-base-100 shadow-sm">
          <div class="navbar-start">
            <a class="btn btn-ghost text-xl">
             <div class="flex items-center">
             <img src="/images/compactsee.png" alt="compactsee" class="w-12 h-12 mr-2" />
             Compactsee
             </div>
            </a>
          </div>
        </div>
    }
}
