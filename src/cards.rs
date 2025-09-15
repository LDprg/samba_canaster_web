use leptos::{component, html::Div, prelude::*, view, IntoView};
use leptos_use::{
    core::Position, use_draggable_with_options, use_mouse_in_element, UseDraggableOptions,
    UseDraggableReturn, UseMouseInElementReturn,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardType {
    Hearts(Rank),
    Diamonds(Rank),
    Clubs(Rank),
    Spades(Rank),
    Joker(Color),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::Red => "red",
            Color::Black => "black",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn as_str(&self) -> &'static str {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }
}

impl CardType {
    pub fn to_string(&self) -> String {
        match self {
            CardType::Hearts(card) => format!("card-{}-hearts", card.as_str()),
            CardType::Diamonds(card) => format!("card-{}-diamonds", card.as_str()),
            CardType::Clubs(card) => format!("card-{}-clubs", card.as_str()),
            CardType::Spades(card) => format!("card-{}-spades", card.as_str()),
            CardType::Joker(color) => format!("card-{}-joker", color.as_str()),
        }
    }
}

#[component]
pub fn Card(card_type: CardType) -> impl IntoView {
    let (focused, set_focused) = signal(false);
    let (pos, set_pos) = signal((0.0, 0.0));

    let fixed_el = NodeRef::<Div>::new();

    let UseDraggableReturn {
        style: fixed_style, ..
    } = use_draggable_with_options(
        fixed_el,
        UseDraggableOptions::default()
            .initial_value(Position { x: 100.0, y: 100.0 })
            .on_start(move |arg| {
                *set_focused.write() = true;
                *set_pos.write() = (arg.position.x, arg.position.y);
                true
            })
            .on_end(move |_| {
                *set_focused.write() = false;
            }),
    );

    view! {
        <div
            node_ref=fixed_el
            class=move || {
                format!(
                    "fixed select-none cursor-move z-{} card {}",
                    if focused.get() { 31 } else { 30 },
                    card_type.to_string(),
                )
            }
            style=move || format!("touch-action: none; {}", fixed_style())
        ></div>
    }
}

#[component]
pub fn CardSlot() -> impl IntoView {
    let target = NodeRef::<Div>::new();
    let UseMouseInElementReturn { is_outside, .. } = use_mouse_in_element(target);

    view! {
        <div node_ref=target class="card card-slot">
            is_outside:
            {is_outside}
        </div>
    }
}
