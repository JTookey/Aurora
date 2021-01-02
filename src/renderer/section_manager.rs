use super::Section;

// Size of the array on the stack - Note 32 is the limit of the Default trait implementation
// for higher numbers would need to use unsafe or something like the Array crate
const MAX_SECTIONS: usize = 32;


pub struct SectionManager<'frame> {
    sections: [Option<Section<'frame>>; MAX_SECTIONS],
    current_size: usize,
}

impl <'frame> SectionManager<'frame> {
    pub fn new() -> Self {
        Self {
            sections: Default::default(),
            current_size: 0,
        }
    }

    pub fn push(&mut self, new_section: Section<'frame>) -> usize {
        let mut index = 0;

        if self.current_size < MAX_SECTIONS {
            self.sections[self.current_size] = Some(new_section);
            index = self.current_size;
            self.current_size += 1;
        }

        index
    }

    pub fn len(&self) -> usize {
        self.current_size
    }

    pub fn get_sections(&mut self, start_id: usize, end_id: usize) -> &mut [Option<Section<'frame>>] {
        &mut self.sections[start_id..end_id]
    }

}