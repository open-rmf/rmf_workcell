/*
 * Copyright (C) 2023 Open Source Robotics Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/

use crate::interaction::{InteractionAssets, Selectable};
use crate::CurrentWorkspace;
use crate::SiteAssets;
use bevy::prelude::*;
use rmf_workcell_format::NameOfWorkcell;

/// Used as an event to command that a new workcell should be made the current one
#[derive(Clone, Copy, Debug, Event)]
pub struct ChangeCurrentWorkcell {
    /// What should the current workcell root be
    pub root: Entity,
}

/// Marker component used to mark the visualization of the workcell (its origin axis).
#[derive(Component, Debug, Clone)]
pub struct WorkcellVisualizationMarker;

pub fn change_workcell(
    mut current_workspace: ResMut<CurrentWorkspace>,
    mut change_current_workcell: EventReader<ChangeCurrentWorkcell>,
    open_workcells: Query<Entity, With<NameOfWorkcell>>,
) {
    if let Some(cmd) = change_current_workcell.read().last() {
        if open_workcells.get(cmd.root).is_err() {
            error!(
                "Requested workspace change to an entity that is not an open workcell: {:?}",
                cmd.root
            );
            return;
        }

        current_workspace.root = Some(cmd.root);
        current_workspace.display = true;
    }
}

pub fn add_workcell_visualization(
    mut commands: Commands,
    new_workcells: Query<Entity, Added<NameOfWorkcell>>,
    site_assets: Res<SiteAssets>,
    interaction_assets: Res<InteractionAssets>,
) {
    for e in new_workcells.iter() {
        let body = commands
            .spawn((
                PbrBundle {
                    mesh: site_assets.site_anchor_mesh.clone(),
                    material: site_assets.passive_anchor_material.clone(),
                    ..default()
                },
                WorkcellVisualizationMarker,
                Selectable::new(e),
            ))
            .set_parent(e)
            .id();
        interaction_assets.make_orientation_cue_meshes(&mut commands, body, 1.0);
    }
}
