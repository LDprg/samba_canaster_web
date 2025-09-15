use leptos::{component, html::Div, prelude::*, view, IntoView};
use leptos_use::{
    core::{IntoElementMaybeSignal, Position}, use_mouse_in_element, UseMouseInElementReturn,
};
use reactive_stores::Store;

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

#[derive(Clone, Debug, Default, Store)]
struct CardsState {
}

pub fn provide_cards_context() {
    provide_context(Store::new(CardsState::default()));
}

pub fn use_draggable<El, M>(target: El, init: Position) -> Signal<String>
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>,
{
    let (position, set_position) = signal(init);

    return Signal::derive(move || {
        let pos = position.get();
        format!("left: {}px; top: {}px;", pos.x, pos.y)
    });
}

#[component]
pub fn Card(card_type: CardType) -> impl IntoView {
    let state = expect_context::<Store<CardsState>>();

    let (focused, set_focused) = signal(false);
    let (pos, set_pos) = signal((0.0, 0.0));

    let card = NodeRef::<Div>::new();
    let style = use_draggable(card, Position{x: 100.0, y: 100.0});

    view! {
        <div
            node_ref=card
            class=move || {
                format!(
                    "fixed select-none cursor-move z-{} card {}",
                    if focused.get() { 31 } else { 30 },
                    card_type.to_string(),
                )
            }
            style=move || format!("touch-action: none; {}", style())
        ></div>
    }
}

#[component]
pub fn CardSlot() -> impl IntoView {
    let state = expect_context::<Store<CardsState>>();

    let target = NodeRef::<Div>::new();
    let UseMouseInElementReturn { is_outside, .. } = use_mouse_in_element(target);

    view! {
        <div node_ref=target class="card card-slot">
            is_outside:
            {is_outside}
        </div>
    }
}
