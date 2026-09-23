//! Reply — a Material Design 3 list-detail mail client.
//!
//! A three-zone expanded-width layout: a navigation rail with a compose FAB,
//! a thread list with a search bar, and a reading pane for the selected
//! thread. The theme is the sample's own palette — warm surfaces, an amber
//! selection container, and a green tertiary FAB — installed into a scoped
//! environment so managed backends can apply their defaults first without
//! losing these colors.

use core::num::NonZeroUsize;

use waterui::accessibility::AccessibilityRole;
use waterui::app::App;
use waterui::color::signal_color;
use waterui::component::vstack;
use waterui::layout::ContentMode;
use waterui::media::Photo;
use waterui::metadata::Metadata;
use waterui::prelude::*;
use waterui::shape::{Capsule, RoundedRectangle, ShapeExt};
use waterui::text::font::{Body, Caption, Subheadline, Title};
use waterui::widget::avatar;
use waterui_icons_material_icon as mdi;

use hydrolysis_m3::color::{
    self, OnSurfaceVariant, SecondaryContainer, SurfaceContainerHigh, SurfaceContainerLowest,
};
use hydrolysis_m3::navigation_rail::NavigationRailLayout;
use hydrolysis_m3::{
    Argb, MaterialColorMode, MaterialColorScheme, MaterialRoleColor, fab, icon_button,
    material_badge, navigation_rail, navigation_rail_item,
};

const RAIL_WIDTH: f32 = 80.0;
const LIST_WIDTH: f32 = 360.0;
const CARD_RADIUS: f32 = 0.10;
const AVATAR: f32 = 40.0;
const SEARCH_AVATAR: f32 = 30.0;

const ASSETS: &str =
    "https://raw.githubusercontent.com/android/compose-samples/main/Reply/app/src/main/res/drawable";

fn asset(name: &str) -> Url {
    format!("{ASSETS}/{name}")
        .parse()
        .expect("sample asset URL is valid")
}

/// The sample's light scheme: warm neutral surfaces, amber secondary
/// container, green tertiary container.
fn reply_scheme() -> MaterialColorScheme {
    const fn role(r: u8, g: u8, b: u8) -> MaterialRoleColor {
        MaterialRoleColor::new(Argb::from_rgb(r, g, b))
    }
    let mut scheme = MaterialColorScheme::baseline_light();
    scheme.mode = MaterialColorMode::Light;
    scheme.primary = role(0x80, 0x56, 0x10);
    scheme.on_primary = role(0xFF, 0xFF, 0xFF);
    scheme.primary_container = role(0xFF, 0xDD, 0xB3);
    scheme.on_primary_container = role(0x29, 0x18, 0x00);
    scheme.secondary = role(0x6F, 0x5B, 0x40);
    scheme.on_secondary = role(0xFF, 0xFF, 0xFF);
    scheme.secondary_container = role(0xFB, 0xDE, 0xBC);
    scheme.on_secondary_container = role(0x27, 0x19, 0x04);
    scheme.tertiary = role(0x51, 0x64, 0x3F);
    scheme.on_tertiary = role(0xFF, 0xFF, 0xFF);
    scheme.tertiary_container = role(0xD4, 0xEA, 0xBB);
    scheme.on_tertiary_container = role(0x10, 0x20, 0x04);
    scheme.error = role(0xBA, 0x1A, 0x1A);
    scheme.on_error = role(0xFF, 0xFF, 0xFF);
    scheme.error_container = role(0xFF, 0xDA, 0xD6);
    scheme.on_error_container = role(0x41, 0x00, 0x02);
    scheme.background = role(0xFF, 0xF8, 0xF4);
    scheme.on_background = role(0x20, 0x1B, 0x13);
    scheme.surface = role(0xFF, 0xF8, 0xF4);
    scheme.on_surface = role(0x20, 0x1B, 0x13);
    scheme.surface_variant = role(0xF0, 0xE0, 0xCF);
    scheme.on_surface_variant = role(0x4F, 0x45, 0x39);
    scheme.outline = role(0x81, 0x75, 0x67);
    scheme.outline_variant = role(0xD3, 0xC4, 0xB4);
    scheme.scrim = role(0x00, 0x00, 0x00);
    scheme.inverse_surface = role(0x36, 0x2F, 0x27);
    scheme.inverse_on_surface = role(0xFC, 0xEF, 0xE2);
    scheme.inverse_primary = role(0xF4, 0xBD, 0x6F);
    scheme.surface_dim = role(0xE4, 0xD8, 0xCC);
    scheme.surface_bright = role(0xFF, 0xF8, 0xF4);
    scheme.surface_container_lowest = role(0xFF, 0xFF, 0xFF);
    scheme.surface_container_low = role(0xFF, 0xF1, 0xE5);
    scheme.surface_container = role(0xF9, 0xEC, 0xDF);
    scheme.surface_container_high = role(0xF3, 0xE6, 0xDA);
    scheme.surface_container_highest = role(0xED, 0xE0, 0xD4);
    scheme.shadow = role(0x00, 0x00, 0x00);
    scheme.surface_tint = scheme.primary;
    scheme
}

/// Installs the sample's color scheme into `content`'s scoped environment.
struct ReplyTheme<V> {
    content: V,
}

impl<V: View> View for ReplyTheme<V> {
    fn body(self, env: &Environment) -> impl View {
        let mut scoped = Environment::new().layered_on(env);
        hydrolysis_m3::install_with_colors(&mut scoped, reply_scheme());
        Metadata::new(self.content, scoped)
    }
}

fn themed(content: impl View) -> impl View {
    ReplyTheme { content }
}

struct Message {
    sender: &'static str,
    time: &'static str,
    recipients: &'static str,
    body: &'static str,
    signature: Option<&'static str>,
    avatar_asset: &'static str,
}

struct Thread {
    sender: &'static str,
    time: &'static str,
    subject: &'static str,
    snippet: &'static str,
    avatar_asset: &'static str,
    photo_asset: Option<&'static str>,
    messages: &'static [Message],
}

const DINNER_CLUB_MESSAGES: &[Message] = &[
    Message {
        sender: "So Duri",
        time: "20 min ago",
        recipients: "To me, Ziad, and Lily",
        body: "I think it's time for us to finally try that new noodle shop downtown that doesn't use menus. Anyone else have other suggestions for dinner club this week? I'm so intrigued by this idea of a noodle restaurant where no one gets to order for themselves – could be fun, or terrible, or both :)",
        signature: Some("So"),
        avatar_asset: "avatar_3.jpg",
    },
    Message {
        sender: "Me",
        time: "4 min ago",
        recipients: "To me, Ziad, and Lily",
        body: "Yes! I forgot about that place! I'm definitely up for taking a risk this week and handing control over to someone else. Let's do it.",
        signature: None,
        avatar_asset: "avatar_10.jpg",
    },
    Message {
        sender: "Lily MacDonald",
        time: "1 hour ago",
        recipients: "To me, Ziad, and So",
        body: "Count me in! I've been wanting to try that place since it opened. Thursday works best for me.",
        signature: Some("Lily"),
        avatar_asset: "avatar_1.jpg",
    },
];

const DOUHUA_MESSAGES: &[Message] = &[Message {
    sender: "老强",
    time: "10 min ago",
    recipients: "To me",
    body: "最近忙吗？昨晚我去了你最爱的那家饭馆，点了他们的特色豆花鱼，吃着吃着就想你了。有空过来，我请你吃。",
    signature: Some("老强"),
    avatar_asset: "avatar_8.jpg",
}];

const FOOD_SHOW_MESSAGES: &[Message] = &[Message {
    sender: "Lily MacDonald",
    time: "2 hours ago",
    recipients: "To me and Karthik",
    body: "Ping– you'd love this new food show I started watching. It's produced by a Thai drummer who started a noodle cart during lockdowns, and every episode ends with a cook-along. Attached a still from last night's episode.",
    signature: Some("Lily"),
    avatar_asset: "avatar_1.jpg",
}];

const THREADS: &[Thread] = &[
    Thread {
        sender: "老强",
        time: "10 min ago",
        subject: "豆花鱼",
        snippet: "最近忙吗？昨晚我去了你最爱的那家饭馆，点了他们的特色豆花鱼，吃着吃着就想你了。",
        avatar_asset: "avatar_8.jpg",
        photo_asset: None,
        messages: DOUHUA_MESSAGES,
    },
    Thread {
        sender: "So Duri",
        time: "20 min ago",
        subject: "Dinner Club",
        snippet: "I think it's time for us to finally try that new noodle shop downtown that doesn't use me\u{2026}",
        avatar_asset: "avatar_3.jpg",
        photo_asset: None,
        messages: DINNER_CLUB_MESSAGES,
    },
    Thread {
        sender: "Lily MacDonald",
        time: "2 hours ago",
        subject: "This food show is made for you",
        snippet: "Ping– you'd love this new food show I started watching. It's produced by a Thai drummer\u{2026}",
        avatar_asset: "avatar_1.jpg",
        photo_asset: Some("paris_3.jpg"),
        messages: FOOD_SHOW_MESSAGES,
    },
];

/// An icon button sitting on a filled circle, the way the sample renders
/// secondary actions (star, delete, more) inside message surfaces.
fn circled_icon_button(label: &'static str, icon: impl View + 'static) -> impl View {
    icon_button(label, icon).background(Capsule.fill(SurfaceContainerHigh))
}

fn star(selected: bool) -> impl View {
    circled_icon_button(
        if selected { "Unstar" } else { "Star" },
        if selected {
            mdi::star()
        } else {
            mdi::star_outline()
        },
    )
}

fn sender_line(message: &'static Message, starred: bool) -> impl View {
    hstack((
        avatar(message.sender)
            .image(asset(message.avatar_asset))
            .size(AVATAR),
        vstack((
            text(message.sender).font(Subheadline),
            text(message.time).font(Caption).foreground(OnSurfaceVariant),
        ))
        .spacing(2.0),
        spacer(),
        star(starred),
    ))
    .spacing(12.0)
}

/// Sender avatar asset keyed by display name — the sample ships a fixed set.
fn avatar_of(name: &'static str) -> Url {
    asset(match name {
        "老强" => "avatar_8.jpg",
        "So Duri" => "avatar_3.jpg",
        "Lily MacDonald" => "avatar_1.jpg",
        "Me" => "avatar_10.jpg",
        other => panic!("no avatar asset for {other}"),
    })
}

fn thread_card(thread: &'static Thread, selected: Binding<usize>, index: usize) -> impl View {
    let container = signal_color(selected.clone().map(move |now| {
        if now == index {
            Color::new(SecondaryContainer)
        } else {
            Color::new(SurfaceContainerLowest)
        }
    }));
    vstack((
        hstack((
            avatar(thread.sender)
                .image(asset(thread.avatar_asset))
                .size(AVATAR),
            vstack((
                text(thread.sender).font(Subheadline),
                text(thread.time).font(Caption).foreground(OnSurfaceVariant),
            ))
            .spacing(2.0),
            spacer(),
            star(false),
        ))
        .spacing(12.0),
        text(thread.subject).font(Title),
        text(thread.snippet)
            .font(Body)
            .line_limit(NonZeroUsize::new(2).expect("2 is non-zero"))
            .foreground(OnSurfaceVariant),
        thread.photo_asset.map(|name| {
            AnyView::new(
                Photo::new(asset(name))
                    .resizable()
                    .content_mode(ContentMode::Fill)
                    .max_height(160.0)
                    .clip(RoundedRectangle::new(0.12)),
            )
        }),
    ))
    .spacing(8.0)
    .padding_with(EdgeInsets::all(16.0))
    .background(container)
    .clip(RoundedRectangle::new(CARD_RADIUS))
    .on_tap(move || selected.set(index))
    .a11y_label(thread.subject)
    .a11y_role(AccessibilityRole::Button)
}

fn search_bar() -> impl View {
    hstack((
        mdi::magnify().foreground(OnSurfaceVariant),
        text("Search replies")
            .font(Body)
            .foreground(OnSurfaceVariant),
        spacer(),
        avatar("Me").image(avatar_of("Me")).size(SEARCH_AVATAR),
    ))
    .spacing(12.0)
    .padding_with(EdgeInsets::symmetric(14.0, 16.0))
    .background(Capsule.fill(SurfaceContainerHigh))
}

fn rail_item<Icon: Clone + View + 'static>(
    selected_rail: &Binding<usize>,
    index: usize,
    label: &'static str,
    icon: Icon,
) -> impl View {
    // The sample's rail items are icon-only; the a11y label carries the name.
    navigation_rail_item("", icon, &selected_rail.condition(move |now| *now == index))
        .action({
            let selected_rail = selected_rail.clone();
            move || selected_rail.set(index)
        })
        .a11y_label(label)
}

fn rail(selected_rail: Binding<usize>) -> impl View {
    vstack((
        icon_button("Menu", mdi::menu()),
        fab("Compose", mdi::pencil()).tertiary(),
        navigation_rail((
            rail_item(&selected_rail, 0, "Chat", material_badge(4, mdi::forum())),
            rail_item(&selected_rail, 1, "Notes", mdi::newspaper()),
            rail_item(&selected_rail, 2, "Mail", mdi::message_outline()),
            rail_item(&selected_rail, 3, "Meet", mdi::video_outline()),
        ))
        .layout(NavigationRailLayout::CollapsedNarrow),
    ))
    .spacing(4.0)
    .min_width(RAIL_WIDTH)
    .max_width(RAIL_WIDTH)
    .max_height(f32::MAX)
    .background(color::Surface)
}

fn list_pane(selected: Binding<usize>) -> impl View {
    scroll(
        vstack((
            search_bar(),
            vstack(
                THREADS
                    .iter()
                    .enumerate()
                    .map(|(index, thread)| thread_card(thread, selected.clone(), index))
                    .collect::<Vec<_>>(),
            )
            .spacing(8.0),
        ))
        .spacing(16.0)
        .padding_with(EdgeInsets::new(16.0, 16.0, 4.0, 12.0)),
    )
    .min_width(LIST_WIDTH)
    .max_width(LIST_WIDTH)
    .max_height(f32::MAX)
}

fn message_body(message: &'static Message) -> impl View {
    vstack((
        sender_line(message, false),
        text(message.recipients)
            .font(Subheadline)
            .foreground(OnSurfaceVariant),
        text(message.body).font(Body),
        message
            .signature
            .map(|signature| AnyView::new(text(signature).font(Body))),
    ))
    .spacing(16.0)
}

fn reply_actions() -> impl View {
    hstack((
        button("Reply").action(|| {}).max_width(f32::MAX),
        button("Reply all").action(|| {}).max_width(f32::MAX),
    ))
    .spacing(8.0)
    .padding_with(EdgeInsets::new(0.0, 8.0, 16.0, 16.0))
}

fn detail_pane(thread: &'static Thread) -> impl View {
    let mut cards = Vec::new();
    for (index, message) in thread.messages.iter().enumerate() {
        let content = if index == 0 {
            AnyView::new(
                vstack((
                    hstack((
                        vstack((
                            text(thread.subject).font(Title),
                            text(format!("{} Messages", thread.messages.len()))
                                .font(Subheadline)
                                .foreground(OnSurfaceVariant),
                        ))
                        .spacing(4.0),
                        spacer(),
                        circled_icon_button("Delete", mdi::trash_can_outline()),
                        circled_icon_button("More", mdi::dots_vertical()),
                    ))
                    .spacing(8.0),
                    message_body(message),
                    reply_actions(),
                ))
                .spacing(24.0),
            )
        } else {
            AnyView::new(message_body(message))
        };
        cards.push(
            AnyView::new(
                vstack((content,))
                    .padding_with(EdgeInsets::new(28.0, 20.0, 20.0, 28.0))
                    .background(SurfaceContainerLowest)
                    .clip(RoundedRectangle::new(CARD_RADIUS)),
            ),
        );
    }
    scroll(vstack(cards).spacing(12.0).padding_with(EdgeInsets::new(
        12.0, 12.0, 12.0, 16.0,
    )))
    .max_width(f32::MAX)
    .max_height(f32::MAX)
}

fn reply(selected: Binding<usize>, selected_rail: Binding<usize>) -> impl View {
    themed(
        hstack((
            rail(selected_rail),
            list_pane(selected.clone()),
            watch(selected, move |index| detail_pane(&THREADS[index])),
        ))
        .max_height(f32::MAX)
        .background(color::Background),
    )
}

/// Self-contained entry for previews and embedding.
#[preview]
pub fn demo() -> impl View {
    reply(binding(1usize), binding(0usize))
}

pub fn app(env: Environment) -> App {
    let selected = binding(1usize);
    let selected_rail = binding(0usize);
    App::new(move || reply(selected.clone(), selected_rail.clone()), env)
}
