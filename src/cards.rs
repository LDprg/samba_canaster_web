use leptos::{component, html::Div, prelude::*, view, IntoView};
use leptos_use::{
    core::Position, use_mouse,
    UseMouseReturn,
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
struct CardsState {}

pub fn provide_cards_context() {
    provide_context(Store::new(CardsState::default()));
}

#[component]
pub fn Card(card_type: CardType, position: Position) -> impl IntoView {
    #[allow(unused)]
    let state = expect_context::<Store<CardsState>>();

    let UseMouseReturn { x, y, .. } = use_mouse();
    let (selected, set_selected) = signal(false);
    let (cur_pos, set_cur_pos) = signal(position);
    let (rel_pos, set_rel_pos) = signal(Position::default());

    let pos = move || {
        if selected.get() {
            let rel_pos = rel_pos.get();
            Position {
                x: x.get() - rel_pos.x,
                y: y.get() - rel_pos.y,
            }
        } else {
            let pos = cur_pos.get();
            set_rel_pos.update(|n| {
                *n = Position {
                    x: x.get() - pos.x,
                    y: y.get() - pos.y,
                }
            });
            pos
        }
    };

    let mouse_up = move |_| {
        set_selected.update(|n| *n = false);
        set_cur_pos.update(|n| {
            let rel_pos = rel_pos.get();
            *n = Position {
                x: x.get() - rel_pos.x,
                y: y.get() - rel_pos.y,
            }
        });
    };

    view! {
        <div
            class=move || {
                format!("fixed select-none cursor-move z-30 card {}", card_type.to_string())
            }
            on:mousedown=move |_| set_selected.update(|n| *n = true)
            on:mouseup=mouse_up
            style=move || format!("touch-action: none; left: {}px; top: {}px;", pos().x, pos().y)
        />
    }
}

#[component]
pub fn CardSlot() -> impl IntoView {
    #[allow(unused)]
    let state = expect_context::<Store<CardsState>>();

    let target = NodeRef::<Div>::new();

    view! {
        <div node_ref=target class="card card-slot">
        </div>
    }
}
