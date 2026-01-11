use gpui::{Axis, Bounds, Context, Entity, Pixels, Point};
use parking_lot::Mutex;
use std::sync::Arc;

use super::pane::Pane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Left,
    Right,
    Up,
    Down,
}

impl SplitDirection {
    pub fn axis(&self) -> Axis {
        match self {
            SplitDirection::Left | SplitDirection::Right => Axis::Horizontal,
            SplitDirection::Up | SplitDirection::Down => Axis::Vertical,
        }
    }

    pub fn increasing(&self) -> bool {
        matches!(self, SplitDirection::Right | SplitDirection::Down)
    }
}

#[derive(Debug, Clone)]
pub enum Member {
    Axis(PaneAxis),
    Pane(Entity<Pane>),
}

#[derive(Debug, Clone)]
pub struct PaneAxis {
    axis: Axis,
    members: Vec<Member>,
    flexes: Arc<Mutex<Vec<f32>>>,
    bounding_boxes: Arc<Mutex<Vec<Option<Bounds<Pixels>>>>>,
}

impl PaneAxis {
    pub fn new(axis: Axis, members: Vec<Member>) -> Self {
        let flexes = vec![1.0; members.len()];
        let bounding_boxes = vec![None; members.len()];

        Self {
            axis,
            members,
            flexes: Arc::new(Mutex::new(flexes)),
            bounding_boxes: Arc::new(Mutex::new(bounding_boxes)),
        }
    }

    pub fn split(
        &mut self,
        old_pane_id: gpui::EntityId,
        new_pane: Entity<Pane>,
        direction: SplitDirection,
    ) -> anyhow::Result<()> {
        for (idx, member) in self.members.iter().enumerate() {
            if let Member::Pane(pane) = member {
                if pane.entity_id() == old_pane_id {
                    let axis = direction.axis();
                    if axis != self.axis {
                        anyhow::bail!("Cannot split in different axis direction");
                    }

                    let new_member = Member::Pane(new_pane);
                    let insert_idx = if direction.increasing() { idx + 1 } else { idx };

                    self.members.insert(insert_idx, new_member);
                    self.flexes.lock().insert(insert_idx, 1.0);
                    self.bounding_boxes.lock().insert(insert_idx, None);

                    return Ok(());
                }
            }
        }

        anyhow::bail!("Pane not found")
    }

    pub fn remove(&mut self, pane_id: gpui::EntityId) -> bool {
        if let Some(idx) = self
            .members
            .iter()
            .position(|m| matches!(m, Member::Pane(p) if p.entity_id() == pane_id))
        {
            self.members.remove(idx);
            self.flexes.lock().remove(idx);
            self.bounding_boxes.lock().remove(idx);
            true
        } else {
            false
        }
    }

    pub fn pane_at_pixel_position(&self, _coordinate: Point<Pixels>) -> Option<&Entity<Pane>> {
        for member in &self.members {
            if let Member::Pane(pane) = member {
                return Some(pane);
            }
        }
        None
    }

    pub fn bounding_box_for_pane(&self, pane_id: gpui::EntityId) -> Option<Bounds<Pixels>> {
        if let Some(idx) = self
            .members
            .iter()
            .position(|m| matches!(m, Member::Pane(p) if p.entity_id() == pane_id))
        {
            self.bounding_boxes.lock().get(idx).copied().flatten()
        } else {
            None
        }
    }

    pub fn root_pane(&self) -> Option<&Entity<Pane>> {
        self.members.first().and_then(|m| {
            if let Member::Pane(pane) = m {
                Some(pane)
            } else {
                None
            }
        })
    }

    pub fn mark_positions(&mut self, _cx: &Context<PaneGroup>) {
        for (idx, _member) in self.members.iter().enumerate() {
            let bounding_box = self.bounding_boxes.lock();
            if bounding_box[idx].is_none() {}
        }
    }
}

pub struct PaneGroup {
    root: Member,
    is_center: bool,
}

impl PaneGroup {
    pub fn with_root(root: Member) -> Self {
        Self {
            root,
            is_center: false,
        }
    }

    pub fn new(pane: Entity<Pane>) -> Self {
        Self {
            root: Member::Pane(pane),
            is_center: false,
        }
    }

    pub fn set_is_center(&mut self, is_center: bool) {
        self.is_center = is_center;
    }

    pub fn split(
        &mut self,
        old_pane_id: gpui::EntityId,
        new_pane: Entity<Pane>,
        direction: SplitDirection,
        cx: &mut Context<PaneGroup>,
    ) -> anyhow::Result<()> {
        let result = match &mut self.root {
            Member::Pane(pane) => {
                if pane.entity_id() == old_pane_id {
                    self.root = Member::new_axis(pane.clone(), new_pane.clone(), direction);
                    Ok(())
                } else {
                    anyhow::bail!("Pane not found")
                }
            }
            Member::Axis(axis) => axis.split(old_pane_id, new_pane.clone(), direction),
        };

        if result.is_ok() {
            self.mark_positions(cx);
        }

        result
    }

    pub fn remove(&mut self, pane_id: gpui::EntityId, cx: &mut Context<PaneGroup>) -> bool {
        let result = match &mut self.root {
            Member::Pane(_) => false,
            Member::Axis(axis) => axis.remove(pane_id),
        };

        if result {
            self.mark_positions(cx);
        }

        result
    }

    pub fn bounding_box_for_pane(&self, pane_id: gpui::EntityId) -> Option<Bounds<Pixels>> {
        match &self.root {
            Member::Pane(_) => None,
            Member::Axis(axis) => axis.bounding_box_for_pane(pane_id),
        }
    }

    pub fn pane_at_pixel_position(&self, coordinate: Point<Pixels>) -> Option<&Entity<Pane>> {
        match &self.root {
            Member::Pane(pane) => Some(pane),
            Member::Axis(axis) => axis.pane_at_pixel_position(coordinate),
        }
    }

    pub fn mark_positions(&mut self, cx: &Context<PaneGroup>) {
        match &mut self.root {
            Member::Pane(_) => {}
            Member::Axis(axis) => axis.mark_positions(cx),
        }
    }
}

impl Member {
    pub fn new_axis(
        old_pane: Entity<Pane>,
        new_pane: Entity<Pane>,
        direction: SplitDirection,
    ) -> Self {
        let axis = direction.axis();
        let members = if direction.increasing() {
            vec![Member::Pane(old_pane), Member::Pane(new_pane)]
        } else {
            vec![Member::Pane(new_pane), Member::Pane(old_pane)]
        };

        Self::Axis(PaneAxis::new(axis, members))
    }

    pub fn mark_positions(&mut self, cx: &Context<PaneGroup>) {
        match self {
            Member::Pane(_) => {}
            Member::Axis(axis) => axis.mark_positions(cx),
        }
    }
}
