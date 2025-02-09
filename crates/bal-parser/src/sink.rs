//! Event sink for building syntax trees

use crate::event::Event;
use bal_syntax::BallerinaLanguage;
use rowan::{GreenNodeBuilder, Language};

pub struct Sink {
    builder: GreenNodeBuilder<'static>,
}

impl Sink {
    pub fn new() -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn process(&mut self, event: Event) {
        match event {
            Event::StartNode { kind } => self
                .builder
                .start_node(BallerinaLanguage::kind_to_raw(kind)),
            Event::AddToken { kind, text } => self
                .builder
                .token(BallerinaLanguage::kind_to_raw(kind), text.as_str()),
            Event::FinishNode => self.builder.finish_node(),
        }
    }
}
