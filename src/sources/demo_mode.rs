use crate::{
    config::{SourceConfig, SourceContext},
    sources,
};

use tokio::time;

use super::Sources;

#[derive(Clone, Debug)]
pub struct DemoMode {
    inner: Sources,
}

impl DemoMode {
    pub(crate) const fn new(source: Sources) -> Self {
        Self { inner: source }
    }

    pub(crate) fn build(&self, mut cx: SourceContext) -> sources::Source {
        let mut interval = time::interval(time::Duration::from_secs(1));
        let this = self.clone();

        Box::pin(async move {
            loop {
                interval.tick().await;

                let event = this.inner.generate_demo_data();
                cx.out.send_event(event).await.unwrap();
            }

            dbg!("out of scope");
        })
    }
}

impl Drop for DemoMode {
    fn drop(&mut self) {
        dbg!("Dropping!", self);
        panic!("arg");
    }
}
