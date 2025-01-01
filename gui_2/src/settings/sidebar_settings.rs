#[derive(PartialEq, Clone, Copy)]
pub enum SidebarTab {
  GridSettings,
  PlotSettings,
}

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
