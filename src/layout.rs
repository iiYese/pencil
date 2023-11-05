use crate::{Ui, Units};
use aery::prelude::*;
use bevy::prelude::*;
use bevy_vector_shapes::painter::Canvas;
use pencil_case_macros::*;

#[derive(Component, Clone, Copy)]
pub struct Rect {
    /// Top left
    pub pos: Vec2,
    pub len: Vec2,
}

// What portion of the canvas is being viewed
#[derive(Component, Clone, Deref, DerefMut)]
pub struct View(pub Rect);

/// What direction to layout children.
#[derive(Component, Clone, Copy)]
pub enum Grain {
    Vertical,
    Horizontal,
}

/// How to automatically fit a child to its parent's grain.
/// Parent must have a [`Grain`] direction.
/// - Fit(Units::Exact(_)):
/// Takes up that many logical units in the parents grain direction.
/// - Fit(Units::Ratio(_)):
/// Takes up the available space as a ratio to siblings that also fit to the grain.
#[derive(Component, Hereditary, Clone, Copy, Deref, DerefMut)]
pub struct Fit(pub Units);

/// Space between children that fit to a grain.
#[derive(Component, Hereditary, Clone, Copy, Deref, DerefMut, Default)]
pub struct Spacing(pub f32);

/// Space inner contents of a [`Rect`] with a grain will not occupy.
#[derive(Component, Hereditary, Clone, Copy, Default)]
pub struct Inset {
    pub with: [Units; 2],
    pub against: [Units; 2],
}

impl Inset {
    pub fn symmetric(amount: f32) -> Self {
        Inset {
            with: [Units::Exact(amount), Units::Exact(amount)],
            against: [Units::Exact(amount), Units::Exact(amount)],
        }
    }

    pub fn with_grain(start: Units, end: Units) -> Self {
        Inset {
            with: [start, end],
            against: [Units::Exact(0.), Units::Exact(0.)],
        }
    }

    pub fn against_grain(start: Units, end: Units) -> Self {
        Inset {
            with: [Units::Exact(0.), Units::Exact(0.)],
            against: [start, end],
        }
    }
}

// TODO: Filter in view
#[rustfmt::skip]
pub fn fit_rects(
    mut rects: Query<(&mut Rect, Option<&Fit>)>,
    canvases: Query<(), With<Canvas>>,
    roots: Query<Entity, Root<Ui>>,
    tree: Query<
        ((Entity, Option<&Grain>, Option<&Inset>, Option<&Spacing>), Relations<Ui>),
        Or<(Root<Ui>, Branch<Ui>)>
    >,
) {
    tree.traverse::<Ui>(roots.iter()).for_each(|(entity, grain, inset, spacing), edges| {
        let Some(grain) = grain else { return };

        let Some(p_rect) = rects
            .get(*entity)
            .map(|(rect, _)| *rect)
            .ok()
            .filter(|_| canvases.get(*entity).is_err())
        else {
            return
        };

        let inset = inset
            .copied()
            .unwrap_or_default();

        let (mut against_len, mut divisible_with) = match grain {
            Grain::Horizontal => (p_rect.len.y, p_rect.len.x),
            Grain::Vertical => (p_rect.len.x, p_rect.len.y),
        };

        // Against calculations
        against_len -= inset
            .against
            .iter()
            .flat_map(Units::exact)
            .sum::<f32>();

        let against_denom = 1. + inset
            .against
            .iter()
            .flat_map(Units::ratio)
            .sum::<f32>();

        against_len -= inset
            .against
            .iter()
            .flat_map(Units::ratio)
            .map(|r| (r / against_denom) * against_len)
            .sum::<f32>();

        let against_start = inset.against[0].calculate(against_denom, against_len);

        // With calculations
        divisible_with -= inset
            .with
            .iter()
            .flat_map(Units::exact)
            .sum::<f32>();

        let mut with_denom = inset
            .with
            .iter()
            .flat_map(Units::ratio)
            .sum::<f32>();

        let spacing = *spacing.copied().unwrap_or_default();

        edges.join::<Ui>(&rects).for_each(|(_, fit)| if let Some(Fit(units)) = fit {
            divisible_with -= spacing;
            match units {
                Units::Exact(val) => divisible_with -= val,
                Units::Ratio(val) => with_denom += val,
            }
        });

        let mut with_start = inset.with[0].calculate(with_denom, divisible_with);

        // Fit rects
        let rect_ctor = match grain {
            Grain::Horizontal => |(pos_x, len_x): (f32, f32), (pos_y, len_y): (f32, f32)| Rect {
                pos: Vec2 { x: pos_x, y: pos_y },
                len: Vec2 { x: len_x, y: len_y },
            },
            Grain::Vertical => |(pos_y, len_y): (f32, f32), (pos_x, len_x): (f32, f32)| Rect {
                pos: Vec2 { x: pos_x, y: pos_y },
                len: Vec2 { x: len_x, y: len_y },
            }
        };

        edges.join::<Ui>(&mut rects).for_each(|(mut rect, fit)| if let Some(Fit(units)) = fit {
            let with_len = units.calculate(with_denom, divisible_with);
            *rect = rect_ctor((with_start, with_len), (against_start, against_len));
            with_start += spacing + with_len;
        });
    });
}
