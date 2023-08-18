use serde::{Deserialize, Serialize};
use serde_with::{formats::Flexible, serde_as, TimestampNanoSeconds};
use std::time::SystemTime;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TraceExport {
    pub(crate) resource_spans: Vec<ResourceSpan>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ResourceSpan {
    pub(crate) resource: Resource,
    pub(crate) scope_spans: Vec<ScopeSpan>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Resource {
    pub(crate) attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Attribute {
    pub(crate) key: String,
    pub(crate) value: AttributeValue,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AttributeValue {
    pub(crate) string_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ScopeSpan {
    pub(crate) scope: Option<Scope>,
    pub(crate) spans: Vec<Span>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Scope {}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Span {
    pub(crate) trace_id: String,
    pub(crate) span_id: String,
    pub(crate) parent_span_id: String,
    pub(crate) name: String,
    #[serde_as(as = "TimestampNanoSeconds<String, Flexible>")]
    pub(crate) start_time_unix_nano: SystemTime,
    #[serde_as(as = "TimestampNanoSeconds<String, Flexible>")]
    pub(crate) end_time_unix_nano: SystemTime,
    pub(crate) attributes: Option<Vec<Attribute>>,
    pub(crate) dropped_attributes_count: Option<usize>,
    pub(crate) events: Option<Vec<Event>>,
    pub(crate) dropped_events_count: Option<usize>,
    pub(crate) links: Option<Vec<Link>>,
    pub(crate) dropped_links_count: Option<usize>,
    pub(crate) status: Option<Status>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Event {
    #[serde_as(as = "TimestampNanoSeconds<String, Flexible>")]
    pub(crate) time_unix_nano: SystemTime,
    pub(crate) name: String,
    pub(crate) attributes: Option<Vec<Attribute>>,
    pub(crate) dropped_attributes_count: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Link {
    pub(crate) trace_id: String,
    pub(crate) span_id: String,
    pub(crate) attributes: Option<Vec<Attribute>>,
    pub(crate) dropped_attributes_count: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Status {
    pub(crate) message: Option<String>,
    pub(crate) code: Option<usize>,
}
