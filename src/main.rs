#![warn(clippy::pedantic)]

use libcnb::build::{BuildContext, BuildResult, BuildResultBuilder};
use libcnb::detect::{DetectContext, DetectResult, DetectResultBuilder};
use libcnb::generic::{GenericError, GenericMetadata, GenericPlatform};
use libcnb::{buildpack_main, Buildpack};
use rand::{thread_rng, Rng};
use serde_jsonlines::write_json_lines;
use std::fs;
use std::ops::Sub;
use std::time::{Duration, SystemTime};

mod export_file;

pub(crate) struct OtelExportBuildpack;

impl Buildpack for OtelExportBuildpack {
    type Platform = GenericPlatform;
    type Metadata = GenericMetadata;
    type Error = GenericError;

    fn detect(&self, _context: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        DetectResultBuilder::pass().build()
    }

    fn build(&self, _context: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        let buildpack_span_id = generate_hex(8);
        let export = export_file::TraceExport {
            resource_spans: vec![export_file::ResourceSpan {
                resource: export_file::Resource { attributes: None },
                scope_spans: vec![export_file::ScopeSpan {
                    scope: None,
                    spans: vec![
                        export_file::Span {
                            name: "buildpacks-otel-export".to_string(),
                            trace_id: String::new(),
                            span_id: buildpack_span_id.clone(),
                            parent_span_id: String::new(),
                            start_time_unix_nano: SystemTime::now().sub(Duration::from_secs(5)),
                            end_time_unix_nano: SystemTime::now(),
                            attributes: Some(vec![export_file::Attribute {
                                key: "nodejs-version".to_string(),
                                value: export_file::AttributeValue {
                                    string_value: "18.2.0".to_string(),
                                },
                            }]),
                            dropped_attributes_count: Some(0),
                            events: None,
                            dropped_events_count: None,
                            links: None,
                            dropped_links_count: None,
                            status: None,
                        },
                        export_file::Span {
                            name: "buildpacks-nodejs-engine-download-node".to_string(),
                            trace_id: String::new(),
                            span_id: generate_hex(8),
                            parent_span_id: buildpack_span_id,
                            start_time_unix_nano: SystemTime::now().sub(Duration::from_secs(3)),
                            end_time_unix_nano: SystemTime::now().sub(Duration::from_secs(1)),
                            attributes: None,
                            dropped_attributes_count: None,
                            events: None,
                            dropped_events_count: None,
                            links: None,
                            dropped_links_count: None,
                            status: None,
                        },
                    ],
                }],
            }],
        };

        fs::create_dir_all("/tmp/cnb-telemetry").unwrap();
        write_json_lines("/tmp/cnb-telemetry/heroku_otel-export.jsonl", vec![export]).unwrap();
        BuildResultBuilder::new().build()
    }
}

const CHARSET: &[u8] = b"0123456789abcdef";
fn generate_hex(len: usize) -> String {
    let mut rng = thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

buildpack_main!(OtelExportBuildpack);
