use metrics::counter;
use vector_core::internal_event::InternalEvent;

#[derive(Debug)]
pub(crate) struct LokiEventUnlabeled;

impl InternalEvent for LokiEventUnlabeled {
    fn emit_metrics(&self) {
        counter!("processing_errors_total", 1,
                "error_type" => "unlabeled_event");
    }
}

#[derive(Debug)]
pub(crate) struct LokiEventsProcessed {
    pub(crate) byte_size: usize,
}

impl InternalEvent for LokiEventsProcessed {
    fn emit_metrics(&self) {
        counter!("processed_bytes_total", self.byte_size as u64); // deprecated
    }
}

#[derive(Debug)]
pub(crate) struct LokiOutOfOrderEventDropped;

impl InternalEvent for LokiOutOfOrderEventDropped {
    fn emit_logs(&self) {
        debug!(
            message = "Received out-of-order event; dropping event.",
            internal_log_rate_secs = 30
        );
    }

    fn emit_metrics(&self) {
        counter!("events_discarded_total", 1,
                "reason" => "out_of_order"); // deprecated
        counter!("processing_errors_total", 1,
                "error_type" => "out_of_order"); // deprecated
        counter!("component_discarded_events_total", 1,
                "reason" => "out_of_order");
    }
}

#[derive(Debug)]
pub(crate) struct LokiOutOfOrderEventRewritten;

impl InternalEvent for LokiOutOfOrderEventRewritten {
    fn emit_logs(&self) {
        debug!(
            message = "Received out-of-order event, rewriting timestamp.",
            internal_log_rate_secs = 30
        );
    }

    fn emit_metrics(&self) {
        counter!("processing_errors_total", 1,
                "error_type" => "out_of_order"); // deprecated
        counter!("rewritten_timestamp_events_total", 1);
    }
}
