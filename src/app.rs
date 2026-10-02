use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use thaw::*;
use thaw::ssr::SSRMountStyleProvider;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <AutoReload options=options.clone() />
                    <HydrationScripts options/>
                    <MetaTags/>
                </head>
                <body>
                    <App/>
                </body>
            </html>
        </SSRMountStyleProvider>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/thaw_template.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <ConfigProvider>
            <h1>"Welcome to Leptos!"</h1>
            <Button appearance=ButtonAppearance::Primary on_click=on_click>
                "Click Me: " {count}
            </Button>

            <Card>
                <CardHeader>
                    <Flex justify=FlexJustify::Center inline=true>
                        <Body1>
                            "Login / Signup"
                        </Body1>
                    </Flex>
                </CardHeader>
                <CardPreview>
                    <Flex vertical=true inline=true>
                        <Input placeholder="Username" />
                        <Input input_type=InputType::Password placeholder="Password"/>
                    </Flex>
                </CardPreview>
                <CardFooter>
                    <Flex justify=FlexJustify::Center>
                        <Button>"Login"</Button>
                        <Button>"SignUp"</Button>
                    </Flex>
                </CardFooter>
            </Card>


        </ConfigProvider>
    }
}
