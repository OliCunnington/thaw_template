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

            // <Spacer />
            // <NavTest />
            <Spacer />
            //Login Card... Centering issues TODO
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

            <Spacer />
            // Table test
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell resizable=true min_width=100.0>"Product"</TableHeaderCell>
                        <TableHeaderCell resizable=true>"Count"</TableHeaderCell>
                        <TableHeaderCell>"Date"</TableHeaderCell>
                        <TableHeaderCell>"Actions"</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell>
                            <TableCellLayout truncate=true>
                                "Renders content with overflow: hidden and text-overflow: ellipsis"
                            </TableCellLayout>
                        </TableCell>
                        <TableCell>
                            <TableCellLayout truncate=true>
                                "Renders content with overflow: hidden and text-overflow: ellipsis"
                            </TableCellLayout>
                        </TableCell>
                        <TableCell>
                            <TableCellLayout>
                                "2023-10-08"
                            </TableCellLayout>
                        </TableCell>
                        <Actions />
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <TableCellLayout>
                                "Apple"
                            </TableCellLayout>
                        </TableCell>
                        <TableCell>
                            <TableCellLayout>
                                "2"
                            </TableCellLayout>
                        </TableCell>
                        <TableCell>
                            <TableCellLayout>
                                "2026-10-02"
                            </TableCellLayout>
                        </TableCell>
                        <Actions />
                    </TableRow>
                </TableBody>
            </Table>
        </ConfigProvider>
    }
}



#[component]
fn Actions() -> impl IntoView {
    view!{
        <TableCell>
            <TableCellLayout>
                <img src="/icons/edit-svgrepo-com.svg" alt="Edit" width="24" height="24"/>
                <img src="/icons/info-square-svgrepo-com.svg" alt="Info" width="24" height="24"/>
                <img src="/icons/more-vertical-svgrepo-com.svg" alt="More" width="24" height="24"/>
                <img src="/icons/x-square-svgrepo-com.svg" alt="Delete" width="24" height="24"/>
            </TableCellLayout>
        </TableCell>
    }
}

#[component]
fn Spacer() -> impl IntoView {
    view!{
        <div style="padding: 30px 0;">
            <Divider />
        </div>
    }
}

#[component]
fn NavTest() -> impl IntoView {
    // breaks tokio?
    view!{
        <NavDrawer>
            <NavCategory value="web_page">
                <NavCategoryItem slot>
                    "Manage Page"
                </NavCategoryItem>
                <NavSubItem value="target">
                    "Target"
                </NavSubItem>
            </NavCategory>
            <NavCategory value="product_page">
                <NavCategoryItem slot>
                    "Manage Products and Orders"
                </NavCategoryItem>
                <NavSubItem value="target">
                    "Target"
                </NavSubItem>
            </NavCategory>
        </NavDrawer>
    }
}