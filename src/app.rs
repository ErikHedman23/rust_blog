use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::{blog_previews::BlogPreviews, edit_post::EditPost, view_post::ViewPost};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
       <div class ="dark:bg-gray-800 text-white p-4">
        <div class="container mx-auto flex justify-between items-center">
            <a href="/" class="text-2xl font-bold">Rust Log</a>
            <nav>
                <ul class="flex space-x-4">
                    <li><a href="/" class="hover:text-blue-400">Blog</a></li>
                    <li><a href="/edit" class="hover:text-blue-400">Create</a></li>
                </ul>
            </nav>
        </div>
       </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/my-rust-blog.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <Navbar/>

        <Router>
            <main class="bg-gray-700 text-gray-200 p-8 h-full">
                <Routes>
                    <Route path="" view=BlogPreviews/>
                    <Route path="/edit/:post_id?" view=EditPost/>
                    <Route path="/view/:post_id?" view=ViewPost/>
                </Routes>
            </main>
        </Router>


    }
}
