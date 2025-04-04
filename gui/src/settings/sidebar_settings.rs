use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum SidebarTab {
  GridSettings,
  PlotSettings,
}

#[derive(Serialize, Deserialize)]
pub struct SidebarSettings {
  pub current_tab: SidebarTab,
}

impl Default for SidebarSettings {
  fn default() -> Self {
    Self {
      current_tab: SidebarTab::GridSettings,
    }
  }
}
