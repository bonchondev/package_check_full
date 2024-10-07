use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
fn Form() -> impl IntoView {
    let AccessPkgName {
        set_name,
        get_name,
        set_type,
        get_type,
    } = expect_context::<AccessPkgName>();
    let input_el: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let value = input_el().expect("input el not yet mounted").value();
        set_name(value);
    };
    view! {
        <form on:submit=on_submit>
            <label
                for="countries"
                class="block pb-2 text-lg font-medium text-gray-900 dark:text-white"
            >
                Which type of package
            </label>
            <select
                on:change=move |ev| {
                    set_type(event_target_value(&ev));
                }
                prop:value=get_type
                id="countries"
                class="bg-gray-50 border p-4 border-gray-300 text-gray-900 text-lg rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            >
                <option value="base">Regular</option>
                <option value="bioconductor">BiocManager</option>
                <option value="devtools">Devtools</option>
                <option value="remote">Remote</option>
                <option value="pip">Python</option>
            </select>
            <label
                for="large-input"
                class="block pt-4 pb-2 text-lg font-medium text-gray-900 dark:text-white"
            >
                Add (insert this) packages here
            </label>
            <input
                type="text"
                id="large-input"
                value=get_name
                node_ref=input_el
                class="block w-full p-4 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 sm:text-md focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                placeholder="Choose which you're trying to install"
            />
            <div class="pt-4">
                <button
                    type="submit"
                    class="text-white bg-gray-800 hover:bg-gray-900 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700"
                >
                    Install
                </button>
            </div>
        </form>
    }
}

#[derive(Copy, Clone)]
struct AccessPkgName {
    set_name: WriteSignal<String>,
    get_name: ReadSignal<String>,
    set_type: WriteSignal<String>,
    get_type: ReadSignal<String>,
}

#[component]
fn Home() -> impl IntoView {
    let (pkg_name, set_pkg_name) = create_signal("".to_string());
    let (pkg_type, set_pkg_type) = create_signal("base".to_string());
    provide_context(AccessPkgName {
        set_name: set_pkg_name,
        get_name: pkg_name,
        set_type: set_pkg_type,
        get_type: pkg_type,
    });
    view! {
        <section class="flex flex-col items-center justify-center pt-8">
            <section class="bg-white dark:bg-gray-900 mx-auto">
                <div class="py-4 px-4 max-w-screen-xl text-center lg:pt-4">
                    <h1 class="pb-4 text-4xl font-extrabold tracking-tight leading-none text-gray-900 md:text-5xl lg:text-6xl dark:text-white">
                        Submit package(s) here
                    </h1>
                    <p class="pb-2 text-lg font-normal text-gray-500 lg:text-xl sm:px-16 lg:px-48 dark:text-gray-400">
                        Place packages in form to test against current version and save the packages ot be added later
                    </p>
                </div>
                <section class="w-3/4 mx-auto">
                    <div class="pb-2">
                        <Form />
                    </div>
                    <section class="rounded-lg border border-gray-300 p-3 h-64 bg-gray-200">
                        <Show
                            when=move || { !pkg_name().is_empty() }
                            fallback=|| view! { <p>waiting on feedback...</p> }
                        >
                            <span class="font-bold">ubuntu@localhost:~$</span>
                            {move || format!(" {}::install({})", pkg_type(), pkg_name())}
                        </Show>

                    </section>
                </section>
            </section>
        </section>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    let formatter = |text| format!("{text} — Leptos Online");
    view! {
        <Stylesheet id="app" href="/style/output.css" />
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
        <Title formatter />
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home /> } />
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
