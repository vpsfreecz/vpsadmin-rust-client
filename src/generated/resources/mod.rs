// Generated resource modules. Component boundaries use double underscores.
pub mod action_state;
pub mod api_server;
pub mod cluster;
pub mod cluster_resource;
pub mod cluster_resource_package;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod cluster_resource_package__item;
pub mod component;
pub mod dataset;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod dataset__plan;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod dataset__property_history;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod dataset__snapshot;
pub mod dataset_expansion;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod dataset_expansion__history;
pub mod dataset_plan;
pub mod debug;
pub mod default_object_cluster_resource;
pub mod dns_record;
pub mod dns_record_log;
pub mod dns_resolver;
pub mod dns_server;
pub mod dns_server_zone;
pub mod dns_server_zone_transfer_log;
pub mod dns_tsig_key;
pub mod dns_zone;
pub mod dns_zone_transfer;
pub mod dnssec_record;
pub mod environment;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod environment__dataset_plan;
pub mod export;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod export__host;
pub mod export_outage;
pub mod help_box;
pub mod host_ip_address;
pub mod incident_report;
pub mod incoming_payment;
pub mod ip_address;
pub mod ip_address_assignment;
pub mod ip_release_campaign;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod ip_release_campaign__address;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod ip_release_campaign__attempt;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod ip_release_campaign__notice;
pub mod ip_release_request;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod ip_release_request__address;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod ip_release_request__notice;
pub mod language;
pub mod location;
pub mod location_network;
pub mod mail_log;
pub mod mail_recipient;
pub mod mail_template;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod mail_template__recipient;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod mail_template__translation;
pub mod mailbox;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod mailbox__handler;
pub mod metrics_access_token;
pub mod migration_plan;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod migration_plan__vps_migration;
pub mod monitored_event;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod monitored_event__log;
pub mod network;
pub mod network_interface;
pub mod network_interface_accounting;
pub mod network_interface_monitor;
pub mod news_log;
pub mod node;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod node__kernel_history;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod node__status;
pub mod node_cgroup_state;
pub mod node_ebpf_program;
pub mod node_ebpf_program_link;
pub mod node_ebpf_program_object;
pub mod node_kernel_configuration_option;
pub mod node_kernel_event;
pub mod node_kernel_evidence;
pub mod node_kernel_evidence_error;
pub mod node_kernel_history_gap;
pub mod node_kernel_history_state;
pub mod node_kernel_livepatch;
pub mod node_kernel_livepatch_patch;
pub mod node_kernel_module;
pub mod node_kernel_parameter;
pub mod node_software_change;
pub mod node_software_deployment;
pub mod node_software_version;
pub mod node_sysctl;
pub mod node_sysctl_change;
pub mod node_system_state;
pub mod node_transfer_connection;
pub mod oauth2_client;
pub mod object_history;
pub mod oom_report;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod oom_report__stat;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod oom_report__task;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod oom_report__usage;
pub mod oom_report_rule;
pub mod os_family;
pub mod os_template;
pub mod outage;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod outage__entity;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod outage__handler;
pub mod outage_security_advisory;
pub mod outage_update;
pub mod password_change_log;
pub mod payment_stats;
pub mod pool;
pub mod security_advisory;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod security_advisory__node_status;
pub mod security_advisory_cve;
pub mod security_advisory_update;
pub mod snapshot_download;
pub mod system_config;
pub mod transaction;
pub mod transaction_chain;
pub mod user;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user__cluster_resource;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user__environment_config;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user__known_device;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user__mail_role_recipient;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user__mail_template_recipient;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user__public_key;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user__state_log;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user__totp_device;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user__webauthn_credential;
pub mod user_account;
pub mod user_cluster_resource_package;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user_cluster_resource_package__item;
pub mod user_namespace;
pub mod user_namespace_map;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user_namespace_map__entry;
pub mod user_outage;
pub mod user_payment;
pub mod user_request;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user_request__change;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod user_request__registration;
pub mod user_security_advisory;
pub mod user_session;
pub mod vps;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod vps__console_token;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod vps__feature;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod vps__maintenance_window;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod vps__mount;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod vps__ssh_host_key;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod vps__state_log;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod vps__status;
pub mod vps_outage;
pub mod vps_security_advisory;
pub mod vps_user_data;
pub mod webauthn;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod webauthn__authentication;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod webauthn__registration;
pub mod webui_user_setting;
// Double underscores preserve resource component boundaries in the public API.
#[allow(non_snake_case)]
pub mod authentication__token__token;
