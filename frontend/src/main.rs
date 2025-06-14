use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Issue {
    id: String,
    title: String,
    description: String,
    location: Location,
    category: String,
    severity: String,
    status: String,
    reported_by: String,
    created_at: String,
    image_url: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Location {
    latitude: f64,
    longitude: f64,
}

#[function_component(App)]
fn app() -> Html {
    let issues = use_state(Vec::new);
    let loading = use_state(|| false);

    {
        let issues = issues.clone();
        let loading = loading.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    loading.set(true);
                    let fetched_issues: Vec<Issue> = Request::get("http://localhost:8080/api/issues")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    issues.set(fetched_issues);
                    loading.set(false);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="container">
            <header>
                <h1>{"CivicLens"}</h1>
                <p>{"Community-Driven Issue Reporting Platform"}</p>
            </header>

            <main>
                <section class="report-issue">
                    <h2>{"Report an Issue"}</h2>
                    <form>
                        <div class="form-group">
                            <label for="title">{"Title"}</label>
                            <input type="text" id="title" class="form-control" />
                        </div>
                        <div class="form-group">
                            <label for="description">{"Description"}</label>
                            <textarea id="description" class="form-control"></textarea>
                        </div>
                        <div class="form-group">
                            <label for="category">{"Category"}</label>
                            <select id="category" class="form-control">
                                <option value="road">{"Road Damage"}</option>
                                <option value="garbage">{"Garbage Collection"}</option>
                                <option value="utility">{"Utility Issue"}</option>
                                <option value="other">{"Other"}</option>
                            </select>
                        </div>
                        <button type="submit" class="btn btn-primary">{"Submit Report"}</button>
                    </form>
                </section>

                <section class="issues-list">
                    <h2>{"Recent Issues"}</h2>
                    if *loading {
                        <div class="loading">{"Loading..."}</div>
                    } else {
                        <div class="issues-grid">
                            {issues.iter().map(|issue| {
                                html! {
                                    <div class="issue-card">
                                        <h3>{&issue.title}</h3>
                                        <p>{&issue.description}</p>
                                        <div class="issue-meta">
                                            <span class="category">{&issue.category}</span>
                                            <span class="severity">{&issue.severity}</span>
                                            <span class="status">{&issue.status}</span>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()}
                        </div>
                    }
                </section>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
} 