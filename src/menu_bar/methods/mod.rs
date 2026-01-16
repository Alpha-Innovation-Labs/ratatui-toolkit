use crate::menu_bar::MenuBar;

impl MenuBar {
    pub fn update_hover(&mut self, column: u16, row: u16) {
        for item in &mut self.items {
            item.hovered = if let Some(area) = item.area {
                column >= area.x
                    && column < area.x + area.width
                    && row >= area.y
                    && row < area.y + area.height
            } else {
                false
            };
        }
    }

    pub fn handle_click(&mut self, column: u16, row: u16) -> Option<usize> {
        let clicked_index = self.items.iter().enumerate().find_map(|(i, item)| {
            if let Some(area) = item.area {
                if column >= area.x
                    && column < area.x + area.width
                    && row >= area.y
                    && row < area.y + area.height
                {
                    return Some(i);
                }
            }
            None
        });

        if let Some(clicked) = clicked_index {
            for (i, item) in self.items.iter_mut().enumerate() {
                item.selected = i == clicked;
            }
        }

        clicked_index
    }

    pub fn selected(&self) -> Option<usize> {
        self.items.iter().position(|item| item.selected)
    }
}
